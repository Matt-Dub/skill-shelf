//! Origin resolver: works out where a user skill comes from, given the shared
//! skills store's lock file. Pure: no I/O.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Origin {
    /// Where the widget sends the user: the skill's folder in the repo, or the repo root.
    pub url: String,
    /// The repository the skill comes from, used to group the Library (e.g. `owner/repo`).
    pub repo: String,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct SkillLock {
    #[serde(default)]
    skills: HashMap<String, LockEntry>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LockEntry {
    source: String,
    skill_path: Option<String>,
}

impl SkillLock {
    pub fn parse(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Resolves a skill's Origin from its lock entry or, for a skill cloned by hand,
/// from the remote in its `.git/config` (`git_config` is that file's contents).
pub fn resolve_origin(folder_name: &str, lock: &SkillLock, git_config: Option<&str>) -> Option<Origin> {
    let Some(entry) = lock.skills.get(folder_name) else {
        let (host, path) = split_source(remote_url(git_config?)?)?;
        let repo = if host == "github.com" { path.to_string() } else { format!("{host}/{path}") };
        return Some(Origin { url: format!("https://{host}/{path}"), repo });
    };
    let (host, path) = split_source(&entry.source)?;
    let root = format!("https://{host}/{path}");
    if host != "github.com" {
        return Some(Origin { url: root, repo: format!("{host}/{path}") });
    }
    let repo = path.to_string();
    let folder = entry
        .skill_path
        .as_deref()
        .map(|path| path.trim_end_matches("SKILL.md").trim_end_matches('/'))
        .filter(|folder| !folder.is_empty());
    let url = match folder {
        Some(folder) => format!("{root}/tree/HEAD/{folder}"),
        None => root,
    };
    Some(Origin { url, repo })
}

/// Resolves a plugin skill's Origin from its marketplace's `source` record (as in
/// `known_marketplaces.json`) and the plugin's `source` in that marketplace's manifest.
pub fn resolve_plugin_origin(
    marketplace_source: &serde_json::Value,
    plugin_source: Option<&serde_json::Value>,
) -> Option<Origin> {
    // A plugin hosted in its own repository (`github`, `url`, `git-subdir` sources).
    if let Some(own_repo) = plugin_source.and_then(repo_of) {
        let (host, path) = split_source(own_repo)?;
        let folder = plugin_source.and_then(|s| s.get("path")).and_then(|p| p.as_str());
        return Some(origin_in_repo(host, path, folder.map(trim_folder)));
    }
    let (host, path) = split_source(repo_of(marketplace_source)?)?;
    let folder = plugin_source.and_then(|source| source.as_str()).map(trim_folder);
    Some(origin_in_repo(host, path, folder))
}

/// The repository a marketplace or plugin `source` record names, if any.
fn repo_of(source: &serde_json::Value) -> Option<&str> {
    source.get("repo").or_else(|| source.get("url"))?.as_str()
}

fn trim_folder(folder: &str) -> &str {
    folder.trim_start_matches("./").trim_matches('/')
}

/// The Origin of a folder inside a repository: the folder itself on GitHub when
/// known, the repository root otherwise.
fn origin_in_repo(host: &str, path: &str, folder: Option<&str>) -> Origin {
    let root = format!("https://{host}/{path}");
    if host != "github.com" {
        return Origin { url: root, repo: format!("{host}/{path}") };
    }
    let url = match folder.filter(|folder| !folder.is_empty()) {
        Some(folder) => format!("{root}/tree/HEAD/{folder}"),
        None => root,
    };
    Origin { url, repo: path.to_string() }
}

/// The URL of the `origin` remote in a git config file, or of its first remote.
fn remote_url(git_config: &str) -> Option<&str> {
    let mut remotes: Vec<(&str, &str)> = Vec::new();
    let mut section: Option<&str> = None;
    for line in git_config.lines().map(str::trim) {
        if let Some(header) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = header
                .strip_prefix("remote")
                .map(|name| name.trim().trim_matches('"'));
        } else if let (Some(name), Some((key, value))) = (section, line.split_once('='))
            && key.trim() == "url"
        {
            remotes.push((name, value.trim()));
        }
    }
    remotes
        .iter()
        .find(|(name, _)| *name == "origin")
        .or(remotes.first())
        .map(|(_, url)| *url)
}

/// Splits a lock-file source into host and repository path. Accepts `owner/repo`
/// (GitHub), `https://host/owner/repo(.git)` and `git@host:owner/repo(.git)`.
fn split_source(source: &str) -> Option<(&str, &str)> {
    let (host, path) = if let Some((_, rest)) = source.split_once("://") {
        let rest = rest.rsplit_once('@').map_or(rest, |(_, after)| after);
        rest.split_once('/')?
    } else if let Some((user_host, path)) = source.split_once(':') {
        (user_host.rsplit_once('@').map_or(user_host, |(_, host)| host), path)
    } else {
        ("github.com", source)
    };
    let path = path.trim_end_matches('/').trim_end_matches(".git");
    (!host.is_empty() && !path.is_empty()).then_some((host, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin(url: &str, repo: &str) -> Option<Origin> {
        Some(Origin { url: url.into(), repo: repo.into() })
    }

    #[test]
    fn resolves_origin_from_lock_entry() {
        let cases: &[(&str, &str, Option<Origin>)] = &[(
            "owner/repo with a skill path",
            r#"{"skills":{"diagnose":{"source":"mattpocock/skills","skillPath":"skills/engineering/diagnose/SKILL.md"}}}"#,
            origin(
                "https://github.com/mattpocock/skills/tree/HEAD/skills/engineering/diagnose",
                "mattpocock/skills",
            ),
        ), (
            "owner/repo without a skill path",
            r#"{"skills":{"diagnose":{"source":"mattpocock/skills"}}}"#,
            origin("https://github.com/mattpocock/skills", "mattpocock/skills"),
        ), (
            "skill path at the repo root",
            r#"{"skills":{"diagnose":{"source":"acme/diagnose","skillPath":"SKILL.md"}}}"#,
            origin("https://github.com/acme/diagnose", "acme/diagnose"),
        ), (
            "GitHub https git URL",
            r#"{"skills":{"diagnose":{"source":"https://github.com/mattpocock/skills.git","skillPath":"skills/diagnose/SKILL.md"}}}"#,
            origin("https://github.com/mattpocock/skills/tree/HEAD/skills/diagnose", "mattpocock/skills"),
        ), (
            "GitHub ssh git URL",
            r#"{"skills":{"diagnose":{"source":"git@github.com:mattpocock/skills.git","skillPath":"skills/diagnose/SKILL.md"}}}"#,
            origin("https://github.com/mattpocock/skills/tree/HEAD/skills/diagnose", "mattpocock/skills"),
        ), (
            "other host: repo root even with a skill path",
            r#"{"skills":{"diagnose":{"source":"https://gitlab.com/acme/tools.git","skillPath":"skills/diagnose/SKILL.md"}}}"#,
            origin("https://gitlab.com/acme/tools", "gitlab.com/acme/tools"),
        ), (
            "no entry for the skill",
            r#"{"skills":{"caveman":{"source":"JuliusBrussee/caveman"}}}"#,
            None,
        )];
        for (label, lock, expected) in cases {
            let lock = SkillLock::parse(lock).unwrap();
            assert_eq!(&resolve_origin("diagnose", &lock, None), expected, "case: {label}");
        }
    }

    #[test]
    fn resolves_origin_from_git_remote_when_not_in_lock_file() {
        let git_config = "[core]\n\trepositoryformatversion = 0\n\tbare = false\n[remote \"origin\"]\n\turl = https://github.com/acme/humanizer.git\n\tfetch = +refs/heads/*:refs/remotes/origin/*\n[branch \"main\"]\n\tremote = origin\n";
        let lock = SkillLock::default();

        assert_eq!(
            resolve_origin("humanizer", &lock, Some(git_config)),
            origin("https://github.com/acme/humanizer", "acme/humanizer"),
        );
    }

    #[test]
    fn skill_without_lock_entry_or_git_remote_has_no_origin() {
        let lock = SkillLock::default();
        let repo_without_remote = "[core]\n\trepositoryformatversion = 0\n[branch \"main\"]\n\tmerge = refs/heads/main\n";

        assert_eq!(resolve_origin("mine", &lock, None), None, "not a git clone");
        assert_eq!(resolve_origin("mine", &lock, Some(repo_without_remote)), None, "clone without remote");
    }

    #[test]
    fn prefers_the_origin_remote_and_falls_back_to_another_one() {
        let lock = SkillLock::default();
        let fork = "[remote \"upstream\"]\n\turl = git@github.com:acme/humanizer.git\n[remote \"origin\"]\n\turl = git@github.com:ada/humanizer.git\n";
        let other = "[remote \"gitlab\"]\n\turl = https://gitlab.com/acme/humanizer.git\n";

        assert_eq!(
            resolve_origin("humanizer", &lock, Some(fork)),
            origin("https://github.com/ada/humanizer", "ada/humanizer"),
        );
        assert_eq!(
            resolve_origin("humanizer", &lock, Some(other)),
            origin("https://gitlab.com/acme/humanizer", "gitlab.com/acme/humanizer"),
        );
    }

    #[test]
    fn plugin_origin_from_github_marketplace_points_to_the_plugin_folder() {
        let marketplace = serde_json::json!({"source": "github", "repo": "Matt-Dub/soundfingers-claude"});
        let plugin = serde_json::json!("./plugins/soundfingers-marketing");

        assert_eq!(
            resolve_plugin_origin(&marketplace, Some(&plugin)),
            origin(
                "https://github.com/Matt-Dub/soundfingers-claude/tree/HEAD/plugins/soundfingers-marketing",
                "Matt-Dub/soundfingers-claude",
            ),
        );
    }

    #[test]
    fn plugin_origin_from_git_url_marketplace() {
        let plugin = serde_json::json!("./plugins/code-review");
        let cases = [(
            "GitHub git URL: plugin folder",
            serde_json::json!({"source": "git", "url": "https://github.com/anthropics/claude-plugins-official.git"}),
            origin(
                "https://github.com/anthropics/claude-plugins-official/tree/HEAD/plugins/code-review",
                "anthropics/claude-plugins-official",
            ),
        ), (
            "other host: repo root",
            serde_json::json!({"source": "git", "url": "https://gitlab.com/acme/plugins.git"}),
            origin("https://gitlab.com/acme/plugins", "gitlab.com/acme/plugins"),
        )];
        for (label, marketplace, expected) in cases {
            assert_eq!(resolve_plugin_origin(&marketplace, Some(&plugin)), expected, "case: {label}");
        }
    }

    #[test]
    fn plugin_hosted_in_its_own_repo_links_there() {
        let marketplace = serde_json::json!({"source": "git", "url": "https://github.com/anthropics/claude-plugins-official.git"});
        let cases = [(
            "git-subdir: plugin folder in its repo",
            Some(serde_json::json!({"source": "git-subdir", "url": "https://github.com/adobe/skills.git", "path": "plugins/creative-cloud/adobe-for-creativity", "ref": "main"})),
            origin("https://github.com/adobe/skills/tree/HEAD/plugins/creative-cloud/adobe-for-creativity", "adobe/skills"),
        ), (
            "url: plugin repo root",
            Some(serde_json::json!({"source": "url", "url": "https://github.com/endorlabs/ai-plugins.git", "sha": "2de0088"})),
            origin("https://github.com/endorlabs/ai-plugins", "endorlabs/ai-plugins"),
        ), (
            "github: plugin repo root",
            Some(serde_json::json!({"source": "github", "repo": "acme/review-plugin"})),
            origin("https://github.com/acme/review-plugin", "acme/review-plugin"),
        ), (
            "unknown plugin source: marketplace root",
            None,
            origin("https://github.com/anthropics/claude-plugins-official", "anthropics/claude-plugins-official"),
        )];
        for (label, plugin, expected) in cases {
            assert_eq!(resolve_plugin_origin(&marketplace, plugin.as_ref()), expected, "case: {label}");
        }
    }

    #[test]
    fn lock_entry_wins_over_git_remote() {
        let lock = SkillLock::parse(r#"{"skills":{"diagnose":{"source":"mattpocock/skills"}}}"#).unwrap();
        let git_config = "[remote \"origin\"]\n\turl = https://github.com/ada/diagnose.git\n";

        assert_eq!(
            resolve_origin("diagnose", &lock, Some(git_config)),
            origin("https://github.com/mattpocock/skills", "mattpocock/skills"),
        );
    }
}

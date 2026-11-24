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

pub fn resolve_origin(folder_name: &str, lock: &SkillLock) -> Option<Origin> {
    let entry = lock.skills.get(folder_name)?;
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
            assert_eq!(&resolve_origin("diagnose", &lock), expected, "case: {label}");
        }
    }
}

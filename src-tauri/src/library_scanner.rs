//! Library scanner: builds the user's Library from the locations an agent adapter names.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::agent_adapter::AgentAdapter;
use crate::origin_resolver::{resolve_origin, resolve_plugin_origin, Origin, SkillLock};
use crate::skill_parser::parse_skill;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    /// The skill's folder as found in the agent's skills folder (not resolved).
    pub path: PathBuf,
    /// Where the skill comes from; `None` for a skill written locally.
    pub origin: Option<Origin>,
}

pub fn scan_library(adapter: &dyn AgentAdapter) -> Vec<Skill> {
    let mut library = user_skills(adapter);
    library.extend(plugin_skills(adapter));
    library.sort_by(|a, b| a.name.cmp(&b.name));
    library
}

fn user_skills(adapter: &dyn AgentAdapter) -> Vec<Skill> {
    let lock = fs::read_to_string(adapter.skill_lock_file())
        .ok()
        .and_then(|json| SkillLock::parse(&json).ok())
        .unwrap_or_default();
    skill_folders(&adapter.user_skills_dir())
        .filter_map(|(folder_name, path)| {
            let info = parse_skill(&fs::read_to_string(path.join("SKILL.md")).ok()?).ok()?;
            let git_config = fs::read_to_string(path.join(".git").join("config")).ok();
            let origin = resolve_origin(&folder_name, &lock, git_config.as_deref());
            Some(Skill { name: info.name, description: info.description, path, origin })
        })
        .collect()
}

/// Skills shipped by the enabled plugins, named `plugin:skill` as the agent names them.
fn plugin_skills(adapter: &dyn AgentAdapter) -> Vec<Skill> {
    let settings = read_json(&adapter.settings_file());
    let installed = read_json(&adapter.installed_plugins_file());
    let marketplaces = read_json(&adapter.known_marketplaces_file());
    let Some(enabled) = settings.get("enabledPlugins").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut skills = Vec::new();
    for (id, _) in enabled.iter().filter(|(_, on)| on.as_bool() == Some(true)) {
        let Some((plugin, marketplace_name)) = id.rsplit_once('@') else { continue };
        let Some(install_path) = installed["plugins"][id][0]["installPath"].as_str() else { continue };
        let marketplace = &marketplaces[marketplace_name];
        let plugin_source = marketplace["installLocation"]
            .as_str()
            .map(|dir| read_json(&Path::new(dir).join(".claude-plugin").join("marketplace.json")))
            .and_then(|manifest| {
                manifest["plugins"]
                    .as_array()?
                    .iter()
                    .find(|entry| entry["name"].as_str() == Some(plugin))?
                    .get("source")
                    .cloned()
            });
        let origin = resolve_plugin_origin(&marketplace["source"], plugin_source.as_ref());
        for (_, path) in skill_folders(&Path::new(install_path).join("skills")) {
            let Some(info) = fs::read_to_string(path.join("SKILL.md"))
                .ok()
                .and_then(|contents| parse_skill(&contents).ok())
            else {
                continue;
            };
            skills.push(Skill {
                name: format!("{plugin}:{}", info.name),
                description: info.description,
                path,
                origin: origin.clone(),
            });
        }
    }
    skills
}

/// The entries of a skills folder as (folder name, path); empty when it is missing.
fn skill_folders(dir: &Path) -> impl Iterator<Item = (String, PathBuf)> {
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| (entry.file_name().to_string_lossy().into_owned(), entry.path()))
}

/// A JSON file's contents, or `null` when it is missing or malformed.
fn read_json(path: &Path) -> Value {
    fs::read_to_string(path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_adapter::ClaudeCodeAdapter;


    fn write_skill(dir: &Path, name: &str, description: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: {description}\n---\n\n# {name}\n"),
        )
        .unwrap();
    }

    fn symlink_dir(target: &Path, link: &Path) {
        fs::create_dir_all(link.parent().unwrap()).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(target, link)
            .expect("creating a symlink on Windows needs Developer Mode or admin rights");
    }

    #[test]
    fn lists_symlinked_and_plain_user_skills_sorted_by_name() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        let stored = adapter.shared_store_dir().join("wizard");
        write_skill(&stored, "wizard", "Interactive wizard.");
        symlink_dir(&stored, &adapter.user_skills_dir().join("wizard"));
        write_skill(&adapter.user_skills_dir().join("tdd"), "tdd", "Test-driven development.");
        write_skill(&adapter.user_skills_dir().join("caveman"), "caveman", "Terse mode.");

        let library = scan_library(&adapter);

        assert_eq!(names(&library), vec!["caveman", "tdd", "wizard"]);
        assert_eq!(library[2].description, "Interactive wizard.");
    }

    #[test]
    fn skips_broken_symlinks_folders_without_skill_md_and_stray_files() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        let skills = adapter.user_skills_dir();
        write_skill(&skills.join("tdd"), "tdd", "Test-driven development.");
        symlink_dir(&adapter.shared_store_dir().join("deleted"), &skills.join("deleted"));
        fs::create_dir_all(skills.join("humanizer").join("notes")).unwrap();
        fs::write(skills.join("README.md"), "not a skill").unwrap();

        let library = scan_library(&adapter);

        assert_eq!(names(&library), vec!["tdd"]);
    }

    #[test]
    fn missing_user_skills_folder_gives_an_empty_library() {
        let home = tempfile::tempdir().unwrap();
        assert!(scan_library(&ClaudeCodeAdapter::for_home(home.path())).is_empty());
    }

    #[test]
    fn attaches_the_origin_recorded_in_the_skill_lock_file() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        let stored = adapter.shared_store_dir().join("diagnose");
        write_skill(&stored, "diagnose", "Diagnosis loop.");
        symlink_dir(&stored, &adapter.user_skills_dir().join("diagnose"));
        write_skill(&adapter.user_skills_dir().join("mine"), "mine", "Written by hand.");
        fs::write(
            adapter.skill_lock_file(),
            r#"{"version":3,"skills":{"diagnose":{"source":"mattpocock/skills","sourceType":"github","skillPath":"skills/engineering/diagnose/SKILL.md"}}}"#,
        )
        .unwrap();

        let library = scan_library(&adapter);

        assert_eq!(
            library[0].origin,
            Some(Origin {
                url: "https://github.com/mattpocock/skills/tree/HEAD/skills/engineering/diagnose".into(),
                repo: "mattpocock/skills".into(),
            })
        );
        assert_eq!(library[1].name, "mine");
        assert_eq!(library[1].origin, None);
    }

    #[test]
    fn malformed_skill_lock_file_leaves_skills_without_origin() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        write_skill(&adapter.user_skills_dir().join("diagnose"), "diagnose", "Diagnosis loop.");
        fs::create_dir_all(adapter.skill_lock_file().parent().unwrap()).unwrap();
        fs::write(adapter.skill_lock_file(), "{ not json").unwrap();

        let library = scan_library(&adapter);

        assert_eq!(names(&library), vec!["diagnose"]);
        assert_eq!(library[0].origin, None);
    }

    #[test]
    fn attaches_the_git_remote_of_a_hand_cloned_skill() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        let clone = adapter.user_skills_dir().join("humanizer");
        write_skill(&clone, "humanizer", "Make text sound human.");
        fs::create_dir_all(clone.join(".git")).unwrap();
        fs::write(
            clone.join(".git").join("config"),
            "[core]\n\tbare = false\n[remote \"origin\"]\n\turl = git@github.com:acme/humanizer.git\n",
        )
        .unwrap();

        let library = scan_library(&adapter);

        assert_eq!(
            library[0].origin,
            Some(Origin { url: "https://github.com/acme/humanizer".into(), repo: "acme/humanizer".into() })
        );
    }

    /// Lays out a plugin the way Claude Code does: its marketplace record, manifest
    /// and install folder, plus an `enabledPlugins` entry when `enabled` is set.
    fn install_plugin(adapter: &ClaudeCodeAdapter, plugin: &str, enabled: Option<bool>, skills: &[&str]) {
        let marketplace = "soundfingers";
        let id = format!("{plugin}@{marketplace}");
        let plugins_dir = adapter.known_marketplaces_file().parent().unwrap().to_path_buf();
        let market_dir = plugins_dir.join("marketplaces").join(marketplace);
        let install_dir = plugins_dir.join("cache").join(marketplace).join(plugin).join("1.0.0");
        for skill in skills {
            write_skill(&install_dir.join("skills").join(skill), skill, "A plugin skill.");
        }
        let manifest = market_dir.join(".claude-plugin").join("marketplace.json");
        fs::create_dir_all(manifest.parent().unwrap()).unwrap();
        fs::write(
            manifest,
            serde_json::json!({"name": marketplace, "plugins": [{"name": plugin, "source": format!("./plugins/{plugin}")}]})
                .to_string(),
        )
        .unwrap();
        fs::write(
            adapter.known_marketplaces_file(),
            serde_json::json!({marketplace: {
                "source": {"source": "github", "repo": "Matt-Dub/soundfingers-claude"},
                "installLocation": market_dir,
            }})
            .to_string(),
        )
        .unwrap();
        fs::write(
            adapter.installed_plugins_file(),
            serde_json::json!({"version": 2, "plugins": {&id: [{"scope": "user", "installPath": install_dir}]}})
                .to_string(),
        )
        .unwrap();
        let enabled = enabled.map_or(serde_json::json!({}), |on| serde_json::json!({&id: on}));
        fs::create_dir_all(adapter.settings_file().parent().unwrap()).unwrap();
        fs::write(adapter.settings_file(), serde_json::json!({"enabledPlugins": enabled}).to_string()).unwrap();
    }

    #[test]
    fn lists_skills_of_enabled_plugins_under_their_qualified_name() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        install_plugin(&adapter, "soundfingers-marketing", Some(true), &["soundfingers-voice"]);

        let library = scan_library(&adapter);

        assert_eq!(names(&library), vec!["soundfingers-marketing:soundfingers-voice"]);
        assert_eq!(
            library[0].origin,
            Some(Origin {
                url: "https://github.com/Matt-Dub/soundfingers-claude/tree/HEAD/plugins/soundfingers-marketing".into(),
                repo: "Matt-Dub/soundfingers-claude".into(),
            })
        );
    }

    #[test]
    fn leaves_out_plugins_that_are_not_enabled() {
        for enabled in [Some(false), None] {
            let home = tempfile::tempdir().unwrap();
            let adapter = ClaudeCodeAdapter::for_home(home.path());
            install_plugin(&adapter, "soundfingers-marketing", enabled, &["soundfingers-voice"]);

            assert!(scan_library(&adapter).is_empty(), "enabled: {enabled:?}");
        }
    }

    #[test]
    fn user_skill_and_plugin_skill_with_the_same_name_are_two_entries() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        write_skill(&adapter.user_skills_dir().join("brain"), "brain", "My own brain.");
        install_plugin(&adapter, "brain", Some(true), &["brain"]);

        let library = scan_library(&adapter);

        assert_eq!(names(&library), vec!["brain", "brain:brain"]);
        assert_eq!(library[0].origin, None);
        assert_eq!(library[1].origin.as_ref().map(|o| o.repo.as_str()), Some("Matt-Dub/soundfingers-claude"));
    }

    fn names(library: &[Skill]) -> Vec<&str> {
        library.iter().map(|s| s.name.as_str()).collect()
    }

    #[test]
    fn lists_a_plain_user_skill() {
        let home = tempfile::tempdir().unwrap();
        let adapter = ClaudeCodeAdapter::for_home(home.path());
        write_skill(&adapter.user_skills_dir().join("tdd"), "tdd", "Test-driven development.");

        let library = scan_library(&adapter);

        assert_eq!(
            library,
            vec![Skill {
                name: "tdd".into(),
                description: "Test-driven development.".into(),
                path: adapter.user_skills_dir().join("tdd"),
                origin: None,
            }]
        );
    }
}

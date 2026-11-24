//! Library scanner: builds the user's Library from the locations an agent adapter names.

use std::fs;
use std::path::PathBuf;

use crate::agent_adapter::AgentAdapter;
use crate::skill_parser::parse_skill;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    /// The skill's folder as found in the agent's skills folder (not resolved).
    pub path: PathBuf,
}

pub fn scan_library(adapter: &dyn AgentAdapter) -> Vec<Skill> {
    let Ok(entries) = fs::read_dir(adapter.user_skills_dir()) else {
        return Vec::new();
    };
    let mut library: Vec<Skill> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            let contents = fs::read_to_string(path.join("SKILL.md")).ok()?;
            let info = parse_skill(&contents).ok()?;
            Some(Skill { name: info.name, description: info.description, path })
        })
        .collect();
    library.sort_by(|a, b| a.name.cmp(&b.name));
    library
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_adapter::ClaudeCodeAdapter;

    use std::path::Path;

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
            }]
        );
    }
}

//! Agent adapter: the only place that knows where an agent keeps its skills on disk.

use std::path::{Path, PathBuf};

/// Where an agent keeps the skills that make up the user's Library.
pub trait AgentAdapter {
    /// The folder holding the user skills (plain folders or symlinks into the shared store).
    fn user_skills_dir(&self) -> PathBuf;
    /// The shared skills store that user skills may be linked from.
    fn shared_store_dir(&self) -> PathBuf;
    /// The shared store's lock file, recording where each stored skill was installed from.
    fn skill_lock_file(&self) -> PathBuf;
}

pub struct ClaudeCodeAdapter {
    home: PathBuf,
}

impl ClaudeCodeAdapter {
    pub fn for_home(home: impl AsRef<Path>) -> Self {
        Self { home: home.as_ref().to_path_buf() }
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn user_skills_dir(&self) -> PathBuf {
        self.home.join(".claude").join("skills")
    }
    fn shared_store_dir(&self) -> PathBuf {
        self.home.join(".agents").join("skills")
    }
    fn skill_lock_file(&self) -> PathBuf {
        self.home.join(".agents").join(".skill-lock.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_code_user_skills_live_under_dot_claude() {
        let adapter = ClaudeCodeAdapter::for_home("/home/ada");
        assert_eq!(adapter.user_skills_dir(), Path::new("/home/ada/.claude/skills"));
    }

    #[test]
    fn claude_code_shared_store_lives_under_dot_agents() {
        let adapter = ClaudeCodeAdapter::for_home("/home/ada");
        assert_eq!(adapter.shared_store_dir(), Path::new("/home/ada/.agents/skills"));
    }

    #[test]
    fn claude_code_skill_lock_file_sits_next_to_the_shared_store() {
        let adapter = ClaudeCodeAdapter::for_home("/home/ada");
        assert_eq!(adapter.skill_lock_file(), Path::new("/home/ada/.agents/.skill-lock.json"));
    }
}

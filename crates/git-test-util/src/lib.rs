use std::fs::File;
use std::path::Path;
use std::process::{Command, Output};
use tempdir::TempDir;

/// Represents a temporary git repository
#[derive(Debug)]
pub struct TempRepo {
    path: TempDir,
}

impl TempRepo {
    /// Creates a new temporary git repository.
    pub fn new() -> std::io::Result<Self> {
        let dir = TempDir::new("git-temp")?;
        File::create(dir.path().join("test.txt"))?;
        let repo = Self { path: dir };
        repo.run_git_cmd(&["init"])?;
        repo.run_git_cmd(&["config", "user.email", "test@test.com"])?;
        repo.run_git_cmd(&["config", "user.name", "Test User"])?;
        repo.run_git_cmd(&["add", "."])?;
        repo.run_git_cmd(&["commit", "-m", "feat: initial commit"])?;
        Ok(repo)
    }

    /// Gets the path to the temporary repository.
    #[inline]
    #[must_use]
    pub fn get_path(&self) -> &Path {
        self.path.path()
    }

    /// Runs a command in the repository.
    #[inline]
    pub fn run_cmd(&self, program: &str, args: &[&str]) -> std::io::Result<Output> {
        Command::new(program).current_dir(&self.path).args(args).output()
    }

    /// Runs a git command in the repository.
    #[inline]
    pub fn run_git_cmd(&self, args: &[&str]) -> std::io::Result<Output> {
        self.run_cmd("git", args)
    }
}

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repo = TempRepo::new().expect("Failed to create temp repo");
        assert!(repo.path.path().exists());
    }

    #[test]
    fn test_get_path() {
        let repo = TempRepo::new().expect("Failed to create temp repo");
        assert_eq!(repo.get_path(), repo.path.path());
    }

    #[test]
    fn test_run_cmd() {
        let repo = TempRepo::new().expect("Failed to create temp repo");
        let output = repo.run_cmd("git", &["status"]).expect("Failed to run command");
        assert!(!output.stdout.is_empty());
    }

    #[test]
    fn test_run_git_cmd() {
        let repo = TempRepo::new().expect("Failed to create temp repo");
        let output = repo.run_git_cmd(&["status"]).expect("Failed to run git command");
        assert!(!output.stdout.is_empty());
    }
}

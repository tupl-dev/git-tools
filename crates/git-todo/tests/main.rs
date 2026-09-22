#![cfg(test)]

use git_test_util::TempRepo;

pub struct TempTestRepo(TempRepo);

impl TempTestRepo {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self(TempRepo::new()?))
    }

    pub fn run_todo_cmd(&self, args: &[&str]) -> std::io::Result<std::process::Output> {
        self.0.run_cmd(env!("CARGO_BIN_EXE_git-todo"), args)
    }
}

#[test]
fn test_add_and_list() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0: implement secret feature 1"));
    assert!(str.contains("[ ] 1: implement secret feature 2"));
    assert!(str.contains("[ ] 2: implement secret feature 3"));
    Ok(())
}

#[test]
fn test_clear_all() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    repo.run_todo_cmd(&["clear"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert_eq!(str, "");
    Ok(())
}

#[test]
fn test_clear_complete_only() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    repo.run_todo_cmd(&["complete", "0", "1"])?;
    repo.run_todo_cmd(&["clear", "--done"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0: implement secret feature 3"));
    Ok(())
}

#[test]
fn test_complete() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    repo.run_todo_cmd(&["complete", "1", "2"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0: implement secret feature 1"));
    assert!(str.contains("[x] 1: implement secret feature 2"));
    assert!(str.contains("[x] 2: implement secret feature 3"));

    repo.run_todo_cmd(&["complete", "0", "1"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[x] 0: implement secret feature 1"));
    assert!(str.contains("[ ] 1: implement secret feature 2"));
    assert!(str.contains("[x] 2: implement secret feature 3"));
    Ok(())
}

#[test]
fn test_complete_undone() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    repo.run_todo_cmd(&["complete", "0"])?;
    let output = repo.run_todo_cmd(&["list", "--undone"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 1: implement secret feature 2"));
    assert!(str.contains("[ ] 2: implement secret feature 3"));
    Ok(())
}

#[test]
fn test_delete() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 3"])?;
    repo.run_todo_cmd(&["delete", "0", "1"])?;
    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(!str.contains("implement secret feature 1"));
    assert!(!str.contains("implement secret feature 2"));
    assert!(str.contains("[ ] 0: implement secret feature 3"));
    Ok(())
}

#[test]
fn test_check_status_code() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement secret feature 1"])?;
    repo.run_todo_cmd(&["add", "implement secret feature 2"])?;
    assert!(!repo.run_todo_cmd(&["check", "--quiet"])?.status.success());

    repo.run_todo_cmd(&["complete", "0"])?;
    assert!(!repo.run_todo_cmd(&["check", "--quiet"])?.status.success());

    repo.run_todo_cmd(&["complete", "1"])?;
    assert!(repo.run_todo_cmd(&["check", "--quiet"])?.status.success());
    Ok(())
}

#[test]
fn test_branch() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_todo_cmd(&["add", "implement feature"])?;
    repo.run_todo_cmd(&["-b", "secret", "add", "implement secret feature"])?;

    let output = repo.run_todo_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0: implement feature"));

    let output = repo.run_todo_cmd(&["-b", "secret", "list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0: implement secret feature"));
    Ok(())
}

#![cfg(test)]

use git_test_util::TempRepo;

pub struct TempTestRepo(TempRepo);

impl TempTestRepo {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self(TempRepo::new()?))
    }

    pub fn run_user_cmd(&self, args: &[&str]) -> std::io::Result<std::process::Output> {
        let path = self.0.get_path().join("users.json").display().to_string();
        self.0
            .run_cmd(env!("CARGO_BIN_EXE_git-user"), &[&["--config", &path], args].concat())
    }
}

#[test]
fn test_add_multiple() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"])?;
    repo.run_user_cmd(&[
        "add",
        "user2",
        "user2@test.com",
        "-s",
        "ssh -i ~/.ssh/key2",
        "-k",
        "~/.ssh/key2.pub",
    ])?;
    repo.run_user_cmd(&["add", "user3", "user3@test.com", "-p", "personal"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user1@test.com"));
    assert!(str.contains("user2@test.com"));
    assert!(str.contains("SSH Command: ssh -i ~/.ssh/key1"));
    assert!(str.contains("Signing Key: ~/.ssh/key2.pub"));
    assert!(str.contains("SSH Command: ssh -i ~/.ssh/key2"));
    assert!(str.contains("Profile: personal"));
    Ok(())
}

#[test]
fn test_add_with_ssh_command() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "-s", "ssh -i ~/.ssh/key"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("SSH Command: ssh -i ~/.ssh/key"));
    Ok(())
}

#[test]
fn test_add_with_signing_key() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "-k", "~/.ssh/key.pub"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("Signing Key: ~/.ssh/key.pub"));
    Ok(())
}

#[test]
fn test_add_with_profile_name() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "-p", "personal"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("Profile: personal"));
    Ok(())
}

#[test]
fn test_add_with_gpg_format() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "--gpg-format", "ssh"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user@test.com"));
    assert!(str.contains("GPG Format: ssh"));
    Ok(())
}

#[test]
fn test_delete() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"])?;
    repo.run_user_cmd(&[
        "add",
        "user2",
        "user2@test.com",
        "-s",
        "ssh -i ~/.ssh/key2",
        "-k",
        "~/.ssh/key2.pub",
    ])?;
    repo.run_user_cmd(&["delete", "user1"])?;
    let output = repo.run_user_cmd(&["list"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(!str.contains("user1@test.com"));
    assert!(!str.contains("SSH Command: ssh -i ~/.ssh/key1"));
    Ok(())
}

#[test]
fn test_use_multiple() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"])?;
    repo.run_user_cmd(&[
        "add",
        "user2",
        "user2@test.com",
        "-s",
        "ssh -i ~/.ssh/key2",
        "-k",
        "~/.ssh/key2.pub",
    ])?;
    repo.run_user_cmd(&["use", "user1"])?;
    let output = repo.0.run_git_cmd(&["config", "--list", "--local"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user.name=user1"));
    assert!(str.contains("user.email=user1@test.com"));
    assert!(str.contains("core.sshcommand=ssh -i ~/.ssh/key1"));

    repo.run_user_cmd(&["use", "user2"])?;
    let output = repo.0.run_git_cmd(&["config", "--list", "--local"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user.name=user2"));
    assert!(str.contains("user.email=user2@test.com"));
    assert!(str.contains("user.signingkey=~/.ssh/key2.pub"));
    assert!(str.contains("core.sshcommand=ssh -i ~/.ssh/key2"));
    Ok(())
}

#[test]
fn test_use_ssh_command() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "-s", "ssh -i ~/.ssh/key1"])?;
    repo.run_user_cmd(&["use", "user"])?;
    let output = repo.0.run_git_cmd(&["config", "--list", "--local"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("core.sshcommand=ssh -i ~/.ssh/key1"));
    Ok(())
}

#[test]
fn test_use_signing_key() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "-k", "~/.ssh/key.pub"])?;
    repo.run_user_cmd(&["use", "user"])?;
    let output = repo.0.run_git_cmd(&["config", "--list", "--local"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user.signingkey=~/.ssh/key.pub"));
    Ok(())
}

#[test]
fn test_use_gpg_format() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user", "user@test.com", "--gpg-format", "ssh"])?;
    repo.run_user_cmd(&["use", "user"])?;
    let output = repo.0.run_git_cmd(&["config", "--list", "--local"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("gpg.format=ssh"));
    Ok(())
}

#[test]
fn test_export() -> std::io::Result<()> {
    let repo = TempTestRepo::new()?;
    repo.run_user_cmd(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"])?;
    repo.run_user_cmd(&[
        "add",
        "user2",
        "user2@test.com",
        "-s",
        "ssh -i ~/.ssh/key2",
        "-k",
        "~/.ssh/key2.pub",
    ])?;
    let output = repo.run_user_cmd(&["export"])?;
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert_ne!(str, "");
    Ok(())
}

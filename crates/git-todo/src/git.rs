use crate::config::Config;
use crate::error::AppError;
use std::fs;
use std::path::{Path, PathBuf};

/// Gets the current git branch name.
pub fn get_branch(path: impl AsRef<Path>) -> Result<String, AppError> {
    let repo = gix::discover(&path).map_err(|e| AppError::Git(e.to_string()))?;
    let head = repo.head().map_err(|e| AppError::Git(e.to_string()))?;
    let branch = head
        .referent_name()
        .ok_or_else(|| AppError::Git("Detached HEAD".to_owned()))?;
    Ok(branch.shorten().to_string())
}

/// Gets the TODO config for the current repository.
pub fn get_todo_config(path: impl AsRef<Path>) -> Result<Config, AppError> {
    let todo_path = get_todo_path(&path)?;
    if todo_path.exists() {
        let content = fs::read_to_string(&todo_path).map_err(AppError::File)?;
        return serde_json::from_str(&content).map_err(AppError::Json);
    }

    let config = Config::default();
    write_todo_config(&path, &config)?;
    Ok(config)
}

/// Writes the TODO config for the current repository.
pub fn write_todo_config(path: impl AsRef<Path>, config: &Config) -> Result<(), AppError> {
    let path = get_todo_path(&path)?;
    let content = serde_json::to_string(config).map_err(AppError::Json)?;
    fs::write(&path, content).map_err(AppError::File)
}

/// Gets the TODO config file path.
fn get_todo_path(path: impl AsRef<Path>) -> Result<PathBuf, AppError> {
    gix::discover(&path)
        .map(|repo| repo.git_dir().join("todo.json"))
        .map_err(|e| AppError::Git(e.to_string()))
}

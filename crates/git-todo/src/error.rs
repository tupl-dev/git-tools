use std::io;
use thiserror::Error;

/// Represents application errors.
#[derive(Debug, Error)]
pub enum AppError {
    /// Error with file IO.
    #[error("File error: {0}")]
    File(#[from] io::Error),

    /// Error with Git.
    #[error("Git error: {0}")]
    Git(String),

    /// Error with JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

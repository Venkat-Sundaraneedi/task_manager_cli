//! Custom error types and a `Result` alias for the task manager application.

use std::io;
use thiserror::Error;

/// Custom error types for the task manager.
///
/// This enum encapsulates all possible errors that can occur within the application,
/// providing specific variants for different failure modes.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("File system error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse TOML data: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Failed to serialize data to TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Task with ID '{0}' not found.")]
    TaskNotFound(u32),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),
}

/// A convenient type alias for `Result` that uses `AppError` as the error type.
///
/// This reduces boilerplate by allowing `Result<T>` instead of `Result<T, AppError>`.
pub type Result<T> = std::result::Result<T, AppError>;

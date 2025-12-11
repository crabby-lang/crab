use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrabError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid project name: {0}. Project name must not be empty and contain only alphanumeric characters and underscores")]
    InvalidProjectName(String),

    #[error("Project already exists at: {0}")]
    ProjectAlreadyExists(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Failed to parse Crab.toml: {0}")]
    TomlParseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Build failed: {0}")]
    BuildError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl CrabError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CrabError::InvalidProjectName(_) => 1,
            CrabError::ProjectAlreadyExists(_) => 2,
            CrabError::ProjectNotFound(_) => 3,
            CrabError::AuthenticationError(_) => 4,
            CrabError::NetworkError(_) => 5,
            CrabError::BuildError(_) => 6,
            _ => 1,
        }
    }
}

pub type CrabResult<T> = Result<T, CrabError>;

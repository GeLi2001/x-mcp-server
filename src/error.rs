//! Error types for X MCP Server

use thiserror::Error;

/// Result type alias for X operations
pub type XResult<T> = Result<T, XError>;

/// Error types for X API operations
#[derive(Error, Debug)]
pub enum XError {
    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// RMCP server initialization errors
    #[error("Server initialization error: {0}")]
    ServerInit(String),

    /// Join errors from tokio
    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// API errors from X
    #[error("X API error: {status} - {message}")]
    Api { status: u16, message: String },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic errors
    #[error("Error: {0}")]
    Generic(String),
}

impl From<anyhow::Error> for XError {
    fn from(err: anyhow::Error) -> Self {
        XError::Generic(err.to_string())
    }
}

impl<T> From<rmcp::service::ServerInitializeError<T>> for XError
where
    T: std::fmt::Display,
{
    fn from(err: rmcp::service::ServerInitializeError<T>) -> Self {
        XError::ServerInit(err.to_string())
    }
}

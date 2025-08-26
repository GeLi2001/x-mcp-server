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

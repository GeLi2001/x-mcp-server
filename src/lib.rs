//! X (Twitter) MCP Server
//!
//! A Model Context Protocol (MCP) server that provides access to X (Twitter) API
//! for basic utilities like posting tweets, getting user information, and more.
//!
//! ## Features
//! 
//! - **User Information**: Get user profiles by username or ID
//! - **Tweet Operations**: Post tweets, reply to tweets, get specific tweets
//! - **Search**: Search for tweets with various filters
//! - **User Timeline**: Get a user's recent tweets
//! - **OAuth 1.0a**: Secure authentication with X API
//!
//! ## Usage
//!
//! ```rust,no_run
//! use x_mcp_server::XMcpServer;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create and run server (requires X_BEARER_TOKEN env var)
//!     let server = XMcpServer::from_env()?;
//!     server.run_stdio().await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod server;
pub mod types;

pub use client::XClient;
pub use error::{XError, XResult};
pub use server::XMcpServer;

/// Version of the X MCP Server
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().next().unwrap().is_ascii_digit());
    }
}
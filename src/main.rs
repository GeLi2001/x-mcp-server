//! X MCP Server - Main binary


use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use x_mcp_server::{XMcpServer, XResult};

#[tokio::main]
async fn main() -> XResult<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "x_mcp_server=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();

    tracing::info!("Starting X MCP Server v{}", x_mcp_server::VERSION);

    // Create the server
    let server = XMcpServer::from_env().map_err(|e| {
        tracing::error!("Failed to create server: {}", e);
        e
    })?;

    tracing::info!("Server started, listening for MCP requests...");

    // Run the server
    server.run_stdio().await.map_err(|e| {
        tracing::error!("Server error: {}", e);
        e
    })?;

    Ok(())
}

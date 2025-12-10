//! Wise MCP Server - Model Context Protocol server for the Wise Platform API.

use rmcp::transport::io::stdio;
use rmcp::ServiceExt;
use tracing_subscriber::{self, EnvFilter};

mod server;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging to stderr (stdout is used for MCP protocol)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Wise MCP server");

    // Get API token from environment
    let api_token = std::env::var("WISE_API_TOKEN")
        .map_err(|_| anyhow::anyhow!("WISE_API_TOKEN environment variable not set"))?;

    // Check if read-only mode is disabled
    let read_only = std::env::var("WISE_READ_ONLY")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    // Check if production mode is enabled
    let production = std::env::var("WISE_PRODUCTION")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    // Create the server
    let server = server::WiseMcpServer::new(api_token, read_only, production)?;

    // Run with stdio transport
    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}

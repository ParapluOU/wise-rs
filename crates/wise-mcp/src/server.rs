//! MCP server implementation.

use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParam, CallToolResult, Content, Implementation, ListToolsResult,
    ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use serde_json::json;
use std::sync::Arc;
use wise_client::{ClientConfig, ReadOnlyClient};

use crate::tools;

/// Wise MCP Server.
#[derive(Clone)]
pub struct WiseMcpServer {
    client: Arc<ReadOnlyClient>,
    #[allow(dead_code)]
    read_only: bool,
}

impl WiseMcpServer {
    /// Create a new Wise MCP server.
    pub fn new(api_token: String, read_only: bool, production: bool) -> anyhow::Result<Self> {
        let config = ClientConfig::with_token(api_token);
        let config = if production {
            config.production()
        } else {
            config.sandbox()
        };

        let client = ReadOnlyClient::new(config)?;

        Ok(Self {
            client: Arc::new(client),
            read_only,
        })
    }

    fn create_tool(name: &str, description: &str, schema: serde_json::Value) -> Tool {
        Tool::new(
            name.to_string(),
            description.to_string(),
            schema.as_object().cloned().unwrap_or_default(),
        )
    }

    fn tools_list() -> Vec<Tool> {
        vec![
            Self::create_tool(
                "get_user",
                "Get the authenticated user's information from Wise",
                json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            ),
            Self::create_tool(
                "list_profiles",
                "List all profiles (personal and business) for the authenticated user",
                json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            ),
            Self::create_tool(
                "list_balances",
                "List all balance accounts for a specific profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID to list balances for"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            Self::create_tool(
                "list_transfers",
                "List transfers with optional filters",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID to filter by (optional)"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of results (optional)"
                        }
                    },
                    "required": []
                }),
            ),
            Self::create_tool(
                "get_transfer",
                "Get details of a specific transfer by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "transfer_id": {
                            "type": "integer",
                            "description": "Transfer ID"
                        }
                    },
                    "required": ["transfer_id"]
                }),
            ),
            Self::create_tool(
                "get_rate",
                "Get current exchange rate between two currencies",
                json!({
                    "type": "object",
                    "properties": {
                        "source": {
                            "type": "string",
                            "description": "Source currency code (e.g., GBP)"
                        },
                        "target": {
                            "type": "string",
                            "description": "Target currency code (e.g., USD)"
                        }
                    },
                    "required": ["source", "target"]
                }),
            ),
            Self::create_tool(
                "list_recipients",
                "List recipient accounts for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID to list recipients for"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
        ]
    }
}

impl ServerHandler for WiseMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "wise-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: None,
                icons: None,
                website_url: None,
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "Wise API server for querying user info, profiles, balances, transfers, and exchange rates.".to_string()
            ),
            ..Default::default()
        }
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        Ok(ListToolsResult {
            tools: Self::tools_list(),
            next_cursor: None,
            meta: None,
        })
    }

    fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, rmcp::ErrorData>> + Send + '_ {
        let client = self.client.clone();
        async move {
            let result = match request.name.as_ref() {
                "get_user" => tools::get_user(&client).await,
                "list_profiles" => tools::list_profiles(&client).await,
                "list_balances" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::list_balances(&client, profile_id).await
                }
                "list_transfers" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args.get("profile_id").and_then(|v| v.as_i64());
                    let limit = args.get("limit").and_then(|v| v.as_u64()).map(|v| v as u32);
                    tools::list_transfers(&client, profile_id, limit).await
                }
                "get_transfer" => {
                    let args = request.arguments.unwrap_or_default();
                    let transfer_id = args
                        .get("transfer_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("transfer_id is required", None)
                        })?;
                    tools::get_transfer(&client, transfer_id).await
                }
                "get_rate" => {
                    let args = request.arguments.unwrap_or_default();
                    let source = args
                        .get("source")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("source is required", None)
                        })?;
                    let target = args
                        .get("target")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("target is required", None)
                        })?;
                    tools::get_rate(&client, source, target).await
                }
                "list_recipients" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::list_recipients(&client, profile_id).await
                }
                _ => {
                    return Err(rmcp::ErrorData::invalid_params(
                        format!("Unknown tool: {}", request.name),
                        None,
                    ))
                }
            };

            match result {
                Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    }
}

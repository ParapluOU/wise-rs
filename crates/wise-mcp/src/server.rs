//! MCP server implementation.

use chrono::{DateTime, Utc};
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
            // User & Profile
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
                "get_profile",
                "Get a specific profile by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            // Balances
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
                "get_balance",
                "Get a specific balance by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "balance_id": {
                            "type": "integer",
                            "description": "Balance ID"
                        }
                    },
                    "required": ["profile_id", "balance_id"]
                }),
            ),
            // Transfers
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
            // Rates & Currencies
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
                "list_currencies",
                "List all supported currencies",
                json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            ),
            // Recipients
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
            Self::create_tool(
                "get_recipient",
                "Get a specific recipient by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "account_id": {
                            "type": "integer",
                            "description": "Recipient account ID"
                        }
                    },
                    "required": ["account_id"]
                }),
            ),
            // Quotes
            Self::create_tool(
                "get_quote",
                "Get a quote by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "quote_id": {
                            "type": "string",
                            "description": "Quote ID (UUID)"
                        }
                    },
                    "required": ["profile_id", "quote_id"]
                }),
            ),
            // Activities
            Self::create_tool(
                "list_activities",
                "List recent activities for a profile (transactions, transfers, etc.)",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "size": {
                            "type": "integer",
                            "description": "Number of activities to return (1-100, default 10)"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            // Addresses
            Self::create_tool(
                "list_addresses",
                "List addresses for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            Self::create_tool(
                "get_address",
                "Get an address by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "address_id": {
                            "type": "integer",
                            "description": "Address ID"
                        }
                    },
                    "required": ["address_id"]
                }),
            ),
            // Statements
            Self::create_tool(
                "get_statement",
                "Get a balance statement for a specific period",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "balance_id": {
                            "type": "integer",
                            "description": "Balance ID"
                        },
                        "currency": {
                            "type": "string",
                            "description": "Currency code (e.g., EUR)"
                        },
                        "interval_start": {
                            "type": "string",
                            "description": "Start date (ISO 8601 format, e.g., 2024-01-01T00:00:00Z)"
                        },
                        "interval_end": {
                            "type": "string",
                            "description": "End date (ISO 8601 format, e.g., 2024-03-31T23:59:59Z)"
                        }
                    },
                    "required": ["profile_id", "balance_id", "currency", "interval_start", "interval_end"]
                }),
            ),
            // Bank Details
            Self::create_tool(
                "list_bank_details",
                "List bank account details for receiving money into a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            // Batch Groups
            Self::create_tool(
                "get_batch_group",
                "Get a batch group by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "batch_group_id": {
                            "type": "string",
                            "description": "Batch group ID (UUID)"
                        }
                    },
                    "required": ["profile_id", "batch_group_id"]
                }),
            ),
            // Cards
            Self::create_tool(
                "list_cards",
                "List cards for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "Items per page (10-100, default 10)"
                        },
                        "page_number": {
                            "type": "integer",
                            "description": "Page number (>= 1, default 1)"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            Self::create_tool(
                "get_card",
                "Get a card by token",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "card_token": {
                            "type": "string",
                            "description": "Card token"
                        }
                    },
                    "required": ["profile_id", "card_token"]
                }),
            ),
            Self::create_tool(
                "get_card_permissions",
                "Get spending permissions for a card",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "card_token": {
                            "type": "string",
                            "description": "Card token"
                        }
                    },
                    "required": ["profile_id", "card_token"]
                }),
            ),
            // Card Orders
            Self::create_tool(
                "list_card_orders",
                "List card orders for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "Items per page (10-100, default 10)"
                        },
                        "page_number": {
                            "type": "integer",
                            "description": "Page number (>= 1, default 1)"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            Self::create_tool(
                "get_card_order",
                "Get a card order by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "card_order_id": {
                            "type": "integer",
                            "description": "Card order ID"
                        }
                    },
                    "required": ["profile_id", "card_order_id"]
                }),
            ),
            Self::create_tool(
                "get_card_availability",
                "Get available card programs for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            // Card Transactions
            Self::create_tool(
                "get_card_transaction",
                "Get a card transaction by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "transaction_id": {
                            "type": "string",
                            "description": "Transaction ID"
                        }
                    },
                    "required": ["profile_id", "transaction_id"]
                }),
            ),
            // Partner Cases
            Self::create_tool(
                "get_case",
                "Get a partner case by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "case_id": {
                            "type": "integer",
                            "description": "Case ID"
                        }
                    },
                    "required": ["case_id"]
                }),
            ),
            Self::create_tool(
                "get_case_comments",
                "Get comments for a partner case",
                json!({
                    "type": "object",
                    "properties": {
                        "case_id": {
                            "type": "integer",
                            "description": "Case ID"
                        }
                    },
                    "required": ["case_id"]
                }),
            ),
            // Direct Debits
            Self::create_tool(
                "list_direct_debits",
                "List direct debit accounts for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "debit_type": {
                            "type": "string",
                            "description": "Direct debit type: ACH (for USD) or EFT (for CAD)"
                        },
                        "currency": {
                            "type": "string",
                            "description": "Currency code (USD or CAD)"
                        }
                    },
                    "required": ["profile_id", "debit_type", "currency"]
                }),
            ),
            // Disputes
            Self::create_tool(
                "list_disputes",
                "List disputes for a profile",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "status": {
                            "type": "string",
                            "description": "Filter by status: ACTIVE or CLOSED (optional)"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "Items per page (10-100, default 10)"
                        },
                        "page_number": {
                            "type": "integer",
                            "description": "Page number (>= 1, default 1)"
                        }
                    },
                    "required": ["profile_id"]
                }),
            ),
            Self::create_tool(
                "get_dispute",
                "Get a dispute by ID",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
                        },
                        "dispute_id": {
                            "type": "string",
                            "description": "Dispute ID"
                        }
                    },
                    "required": ["profile_id", "dispute_id"]
                }),
            ),
            Self::create_tool(
                "get_dispute_reasons",
                "Get available dispute reasons",
                json!({
                    "type": "object",
                    "properties": {
                        "profile_id": {
                            "type": "integer",
                            "description": "Profile ID"
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
                "Wise API server for querying user info, profiles, balances, transfers, cards, disputes, and more.".to_string()
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
                // User & Profile
                "get_user" => tools::get_user(&client).await,
                "list_profiles" => tools::list_profiles(&client).await,
                "get_profile" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::get_profile(&client, profile_id).await
                }
                // Balances
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
                "get_balance" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let balance_id = args
                        .get("balance_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("balance_id is required", None)
                        })?;
                    tools::get_balance(&client, profile_id, balance_id).await
                }
                // Transfers
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
                // Rates & Currencies
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
                "list_currencies" => tools::list_currencies(&client).await,
                // Recipients
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
                "get_recipient" => {
                    let args = request.arguments.unwrap_or_default();
                    let account_id = args
                        .get("account_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("account_id is required", None)
                        })?;
                    tools::get_recipient(&client, account_id).await
                }
                // Quotes
                "get_quote" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let quote_id = args
                        .get("quote_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("quote_id is required", None)
                        })?;
                    tools::get_quote(&client, profile_id, quote_id).await
                }
                // Activities
                "list_activities" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let size = args.get("size").and_then(|v| v.as_u64()).map(|v| v as u32);
                    tools::list_activities(&client, profile_id, size).await
                }
                // Addresses
                "list_addresses" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::list_addresses(&client, profile_id).await
                }
                "get_address" => {
                    let args = request.arguments.unwrap_or_default();
                    let address_id = args
                        .get("address_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("address_id is required", None)
                        })?;
                    tools::get_address(&client, address_id).await
                }
                // Statements
                "get_statement" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let balance_id = args
                        .get("balance_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("balance_id is required", None)
                        })?;
                    let currency = args
                        .get("currency")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("currency is required", None)
                        })?;
                    let interval_start = args
                        .get("interval_start")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("interval_start is required", None)
                        })?;
                    let interval_end = args
                        .get("interval_end")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("interval_end is required", None)
                        })?;
                    let start: DateTime<Utc> = interval_start.parse().map_err(|_| {
                        rmcp::ErrorData::invalid_params(
                            "interval_start must be ISO 8601 format",
                            None,
                        )
                    })?;
                    let end: DateTime<Utc> = interval_end.parse().map_err(|_| {
                        rmcp::ErrorData::invalid_params(
                            "interval_end must be ISO 8601 format",
                            None,
                        )
                    })?;
                    tools::get_statement(&client, profile_id, balance_id, currency, start, end)
                        .await
                }
                // Bank Details
                "list_bank_details" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::list_bank_details(&client, profile_id).await
                }
                // Batch Groups
                "get_batch_group" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let batch_group_id = args
                        .get("batch_group_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("batch_group_id is required", None)
                        })?;
                    tools::get_batch_group(&client, profile_id, batch_group_id).await
                }
                // Cards
                "list_cards" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let page_size = args
                        .get("page_size")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    let page_number = args
                        .get("page_number")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    tools::list_cards(&client, profile_id, page_size, page_number).await
                }
                "get_card" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let card_token = args
                        .get("card_token")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("card_token is required", None)
                        })?;
                    tools::get_card(&client, profile_id, card_token).await
                }
                "get_card_permissions" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let card_token = args
                        .get("card_token")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("card_token is required", None)
                        })?;
                    tools::get_card_permissions(&client, profile_id, card_token).await
                }
                // Card Orders
                "list_card_orders" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let page_size = args
                        .get("page_size")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    let page_number = args
                        .get("page_number")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    tools::list_card_orders(&client, profile_id, page_size, page_number).await
                }
                "get_card_order" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let card_order_id = args
                        .get("card_order_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("card_order_id is required", None)
                        })?;
                    tools::get_card_order(&client, profile_id, card_order_id).await
                }
                "get_card_availability" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::get_card_availability(&client, profile_id).await
                }
                // Card Transactions
                "get_card_transaction" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let transaction_id = args
                        .get("transaction_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("transaction_id is required", None)
                        })?;
                    tools::get_card_transaction(&client, profile_id, transaction_id).await
                }
                // Partner Cases
                "get_case" => {
                    let args = request.arguments.unwrap_or_default();
                    let case_id = args
                        .get("case_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("case_id is required", None)
                        })?;
                    tools::get_case(&client, case_id).await
                }
                "get_case_comments" => {
                    let args = request.arguments.unwrap_or_default();
                    let case_id = args
                        .get("case_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("case_id is required", None)
                        })?;
                    tools::get_case_comments(&client, case_id).await
                }
                // Direct Debits
                "list_direct_debits" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let debit_type = args
                        .get("debit_type")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("debit_type is required", None)
                        })?;
                    let currency = args
                        .get("currency")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("currency is required", None)
                        })?;
                    tools::list_direct_debits(&client, profile_id, debit_type, currency).await
                }
                // Disputes
                "list_disputes" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let status = args.get("status").and_then(|v| v.as_str());
                    let page_size = args
                        .get("page_size")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    let page_number = args
                        .get("page_number")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    tools::list_disputes(&client, profile_id, status, page_size, page_number).await
                }
                "get_dispute" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    let dispute_id = args
                        .get("dispute_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("dispute_id is required", None)
                        })?;
                    tools::get_dispute(&client, profile_id, dispute_id).await
                }
                "get_dispute_reasons" => {
                    let args = request.arguments.unwrap_or_default();
                    let profile_id = args
                        .get("profile_id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| {
                            rmcp::ErrorData::invalid_params("profile_id is required", None)
                        })?;
                    tools::get_dispute_reasons(&client, profile_id).await
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

//! MCP Server implementation for X API using RMCP SDK

use crate::client::XClient;
use crate::error::XResult;
use crate::types::SearchTweetsParams;
use rmcp::{
    model::ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{tool::ToolRouter},
        tool::Parameters,
    },
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
    ServiceExt, transport::stdio,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::Future;

/// Tool arguments for getting user information
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetUserArgs {
    /// Username (without @) or user ID
    pub identifier: String,
    /// Whether the identifier is a user ID (true) or username (false)
    #[serde(default)]
    pub is_user_id: bool,
}



/// Tool arguments for searching tweets
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchTweetsArgs {
    /// Search query
    pub query: String,
    /// Maximum number of results (default: 10, max: 100)
    #[serde(default = "default_max_results")]
    pub max_results: u32,
    /// Include user information in results
    #[serde(default)]
    pub include_users: bool,
    /// Include tweet metrics
    #[serde(default)]
    pub include_metrics: bool,
}

/// Tool arguments for getting a specific tweet
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetTweetArgs {
    /// The tweet ID
    pub tweet_id: String,
}

/// Tool arguments for getting user's tweets
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetUserTweetsArgs {
    /// Username or user ID
    pub identifier: String,
    /// Whether the identifier is a user ID (true) or username (false)
    #[serde(default)]
    pub is_user_id: bool,
    /// Maximum number of tweets to retrieve (default: 10)
    #[serde(default = "default_max_results")]
    pub max_results: u32,
}

fn default_max_results() -> u32 {
    10
}

/// X MCP Server
#[derive(Clone)]
pub struct XMcpServer {
    client: XClient,
    tool_router: ToolRouter<XMcpServer>,
}

#[tool_router]
impl XMcpServer {
    /// Create a new X MCP Server
    pub fn new(client: XClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

    /// Create server from environment variables
    pub fn from_env() -> XResult<Self> {
        let client = XClient::from_env()?;
        Ok(Self::new(client))
    }

    /// Run the server with stdio transport
    pub async fn run_stdio(self) -> XResult<()> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    /// Get user information by username or user ID
    #[tool(description = "Get user information by username or user ID")]
    async fn get_user(
        &self,
        Parameters(args): Parameters<GetUserArgs>,
    ) -> Result<CallToolResult, McpError> {
        let user = if args.is_user_id {
            self.client.get_user_by_id(&args.identifier).await
        } else {
            self.client.get_user_by_username(&args.identifier).await
        };

        match user {
            Ok(Some(user)) => {
                let result = json!({
                    "success": true,
                    "user": user
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Ok(None) => {
                let result = json!({
                    "success": false,
                    "error": "User not found"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Err(e) => {
                let result = json!({
                    "success": false,
                    "error": format!("Error: {}", e)
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
        }
    }



    /// Search for tweets
    #[tool(description = "Search for tweets")]
    async fn search_tweets(
        &self,
        Parameters(args): Parameters<SearchTweetsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let mut tweet_fields = vec![
            "id".to_string(),
            "text".to_string(),
            "author_id".to_string(),
            "created_at".to_string(),
        ];

        let mut user_fields = vec![];
        let mut expansions = vec![];

        if args.include_metrics {
            tweet_fields.push("public_metrics".to_string());
        }

        if args.include_users {
            user_fields.extend(vec![
                "id".to_string(),
                "name".to_string(),
                "username".to_string(),
            ]);
            expansions.push("author_id".to_string());
        }

        let search_params = SearchTweetsParams {
            query: args.query,
            max_results: Some(args.max_results.min(100)), // API limit
            tweet_fields: Some(tweet_fields),
            user_fields: if user_fields.is_empty() { None } else { Some(user_fields) },
            expansions: if expansions.is_empty() { None } else { Some(expansions) },
        };

        match self.client.search_tweets(search_params).await {
            Ok(tweets) => {
                let result = json!({
                    "success": true,
                    "tweets": tweets,
                    "count": tweets.len()
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Err(e) => {
                let result = json!({
                    "success": false,
                    "error": format!("Error: {}", e)
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
        }
    }

    /// Get a specific tweet by ID
    #[tool(description = "Get a specific tweet by ID")]
    async fn get_tweet(
        &self,
        Parameters(args): Parameters<GetTweetArgs>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_tweet(&args.tweet_id).await {
            Ok(Some(tweet)) => {
                let result = json!({
                    "success": true,
                    "tweet": tweet
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Ok(None) => {
                let result = json!({
                    "success": false,
                    "error": "Tweet not found"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Err(e) => {
                let result = json!({
                    "success": false,
                    "error": format!("Error: {}", e)
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
        }
    }

    /// Get user's recent tweets
    #[tool(description = "Get user's recent tweets")]
    async fn get_user_tweets(
        &self,
        Parameters(args): Parameters<GetUserTweetsArgs>,
    ) -> Result<CallToolResult, McpError> {
        // First, get the user to get their ID if we have a username
        let user_id = if args.is_user_id {
            args.identifier.clone()
        } else {
            match self.client.get_user_by_username(&args.identifier).await {
                Ok(Some(user)) => user.id,
                Ok(None) => {
                    let result = json!({
                        "success": false,
                        "error": "User not found"
                    });
                    return Ok(CallToolResult::success(vec![Content::text(
                        serde_json::to_string_pretty(&result).unwrap_or_default(),
                    )]));
                }
                Err(e) => {
                    let result = json!({
                        "success": false,
                        "error": format!("Error: {}", e)
                    });
                    return Ok(CallToolResult::success(vec![Content::text(
                        serde_json::to_string_pretty(&result).unwrap_or_default(),
                    )]));
                }
            }
        };

        match self
            .client
            .get_user_tweets(&user_id, Some(args.max_results.min(100)))
            .await
        {
            Ok(tweets) => {
                let result = json!({
                    "success": true,
                    "tweets": tweets,
                    "count": tweets.len(),
                    "user_id": user_id
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Err(e) => {
                let result = json!({
                    "success": false,
                    "error": format!("Error: {}", e)
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for XMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "x-mcp-server".to_string(),
                version: crate::VERSION.to_string(),
            },
            instructions: Some("This server provides X (Twitter) API tools for read-only operations. Available tools: get_user, search_tweets, get_tweet, get_user_tweets.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}

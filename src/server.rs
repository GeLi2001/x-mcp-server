//! MCP Server implementation for X API

use crate::client::XClient;
use crate::error::{XError, XResult};
use crate::tools::{
    get_tweet, get_user, get_user_tweets, post_tweet, search_tweets, GetTweetArgs, GetUserArgs,
    GetUserTweetsArgs, PostTweetArgs, SearchTweetsArgs,
};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as AsyncBufReader};

/// X MCP Server
pub struct XMcpServer {
    client: XClient,
}

impl XMcpServer {
    /// Create a new X MCP Server
    pub fn new(client: XClient) -> Self {
        Self { client }
    }

    /// Create server from environment variables
    pub fn from_env() -> XResult<Self> {
        let client = XClient::from_env()?;
        Ok(Self::new(client))
    }

    /// Run the server with stdio transport
    pub async fn run_stdio(self) -> XResult<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = AsyncBufReader::new(stdin);
        let mut line = String::new();

        // Send server info
        let _server_info = json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "protocol_version": "2024-11-05",
                "capabilities": {
                    "tools": {
                        "list_changed": false
                    }
                },
                "server_info": {
                    "name": "x-mcp-server",
                    "version": crate::VERSION
                }
            }
        });

        tracing::info!("X MCP Server initialized");

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if let Ok(request) = serde_json::from_str::<Value>(&line) {
                        let response = self.handle_request(request).await;
                        if let Ok(response_str) = serde_json::to_string(&response) {
                            stdout.write_all(response_str.as_bytes()).await?;
                            stdout.write_all(b"\n").await?;
                            stdout.flush().await?;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle incoming MCP requests
    async fn handle_request(&self, request: Value) -> Value {
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = request.get("id").cloned();

        match method {
            "initialize" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocol_version": "2024-11-05",
                        "capabilities": {
                            "tools": {
                                "list_changed": false
                            }
                        },
                        "server_info": {
                            "name": "x-mcp-server",
                            "version": crate::VERSION
                        }
                    }
                })
            }
            "tools/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "get_user",
                                "description": "Get user information by username or user ID",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "identifier": {
                                            "type": "string",
                                            "description": "Username (without @) or user ID"
                                        },
                                        "is_user_id": {
                                            "type": "boolean",
                                            "description": "Whether the identifier is a user ID (true) or username (false)",
                                            "default": false
                                        }
                                    },
                                    "required": ["identifier"]
                                }
                            },
                            {
                                "name": "post_tweet",
                                "description": "Post a new tweet",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {
                                            "type": "string",
                                            "description": "The text content of the tweet"
                                        },
                                        "reply_to": {
                                            "type": "string",
                                            "description": "Optional tweet ID to reply to"
                                        }
                                    },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "search_tweets",
                                "description": "Search for tweets",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "query": {
                                            "type": "string",
                                            "description": "Search query"
                                        },
                                        "max_results": {
                                            "type": "integer",
                                            "description": "Maximum number of results (default: 10, max: 100)",
                                            "default": 10,
                                            "minimum": 1,
                                            "maximum": 100
                                        },
                                        "include_users": {
                                            "type": "boolean",
                                            "description": "Include user information in results",
                                            "default": false
                                        },
                                        "include_metrics": {
                                            "type": "boolean",
                                            "description": "Include tweet metrics",
                                            "default": false
                                        }
                                    },
                                    "required": ["query"]
                                }
                            },
                            {
                                "name": "get_tweet",
                                "description": "Get a specific tweet by ID",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "tweet_id": {
                                            "type": "string",
                                            "description": "The tweet ID"
                                        }
                                    },
                                    "required": ["tweet_id"]
                                }
                            },
                            {
                                "name": "get_user_tweets",
                                "description": "Get user's recent tweets",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "identifier": {
                                            "type": "string",
                                            "description": "Username or user ID"
                                        },
                                        "is_user_id": {
                                            "type": "boolean",
                                            "description": "Whether the identifier is a user ID (true) or username (false)",
                                            "default": false
                                        },
                                        "max_results": {
                                            "type": "integer",
                                            "description": "Maximum number of tweets to retrieve (default: 10)",
                                            "default": 10,
                                            "minimum": 1,
                                            "maximum": 100
                                        }
                                    },
                                    "required": ["identifier"]
                                }
                            }
                        ]
                    }
                })
            }
            "tools/call" => {
                let params = request.get("params").unwrap_or(&Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

                let result = self.call_tool(tool_name, arguments).await;

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error serializing result".to_string())
                            }
                        ]
                    }
                })
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": "Method not found"
                    }
                })
            }
        }
    }

    /// Call a specific tool
    async fn call_tool(&self, tool_name: &str, arguments: Value) -> Value {
        let result = match tool_name {
            "get_user" => {
                match serde_json::from_value::<GetUserArgs>(arguments) {
                    Ok(args) => get_user(args, &self.client).await,
                    Err(e) => Err(XError::Json(e)),
                }
            }
            "post_tweet" => {
                match serde_json::from_value::<PostTweetArgs>(arguments) {
                    Ok(args) => post_tweet(args, &self.client).await,
                    Err(e) => Err(XError::Json(e)),
                }
            }
            "search_tweets" => {
                match serde_json::from_value::<SearchTweetsArgs>(arguments) {
                    Ok(args) => search_tweets(args, &self.client).await,
                    Err(e) => Err(XError::Json(e)),
                }
            }
            "get_tweet" => {
                match serde_json::from_value::<GetTweetArgs>(arguments) {
                    Ok(args) => get_tweet(args, &self.client).await,
                    Err(e) => Err(XError::Json(e)),
                }
            }
            "get_user_tweets" => {
                match serde_json::from_value::<GetUserTweetsArgs>(arguments) {
                    Ok(args) => get_user_tweets(args, &self.client).await,
                    Err(e) => Err(XError::Json(e)),
                }
            }
            _ => Err(XError::Generic(format!("Unknown tool: {}", tool_name))),
        };

        match result {
            Ok(content) => content,
            Err(error) => json!({
                "success": false,
                "error": error.to_string()
            }),
        }
    }
}
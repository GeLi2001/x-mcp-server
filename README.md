# X MCP Server

[![Crates.io](https://img.shields.io/crates/v/x-mcp-server.svg)](https://crates.io/crates/x-mcp-server)
[![Documentation](https://docs.rs/x-mcp-server/badge.svg)](https://docs.rs/x-mcp-server)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server that provides access to X (formerly Twitter) API for basic utilities. This server allows AI assistants and other MCP clients to interact with X/Twitter through a standardized interface.

## Features

- 🔐 **Simple Bearer Token Authentication** - Easy setup with just one token
- 👤 **User Information** - Get user profiles by username or ID
- 🐦 **Read-Only Operations** - Get tweets, user information, and search (no posting)
- 🔍 **Search** - Search for tweets with various filters and options
- 📝 **User Timeline** - Retrieve a user's recent tweets
- 🛠️ **MCP Tools** - Standardized tools for AI integration
- ⚡ **Async/Await** - Built with Tokio for high performance

## Installation

### From crates.io

```bash
cargo install x-mcp-server
```

### From source

```bash
git clone https://github.com/yourusername/x-mcp-server
cd x-mcp-server
cargo build --release
```

## Quick Start

### 1. Get X API Credentials

1. Go to the [X Developer Portal](https://developer.twitter.com/en/portal/dashboard)
2. Create a new app or use an existing one
3. Generate your API keys and access tokens
4. Make sure your app has the necessary permissions

### 2. Set Environment Variables

Create a `.env` file or set environment variables:

```bash
export X_CONSUMER_KEY="your_consumer_key"
export X_CONSUMER_SECRET="your_consumer_secret"
export X_ACCESS_TOKEN="your_access_token"
export X_ACCESS_TOKEN_SECRET="your_access_token_secret"
```

Or copy `.env.example` to `.env` and fill in your credentials.

### 3. Run the Server

```bash
x-mcp-server
```

The server will start and listen for MCP requests on stdin/stdout.

## Configuration

The server can be configured using environment variables:

| Variable         | Description                           | Required |
| ---------------- | ------------------------------------- | -------- |
| `X_BEARER_TOKEN` | Your X API Bearer Token               | Yes      |
| `RUST_LOG`       | Logging level (e.g., `info`, `debug`) | No       |

## Available Tools

The server provides the following MCP tools:

### `get_user`

Get user information by username or user ID.

**Parameters:**

- `identifier` (string): Username (without @) or user ID
- `is_user_id` (boolean, optional): Whether the identifier is a user ID (default: false)

**Example:**

```json
{
  "identifier": "elonmusk",
  "is_user_id": false
}
```

### `post_tweet`

Post a new tweet.

**Parameters:**

- `text` (string): The text content of the tweet
- `reply_to` (string, optional): Tweet ID to reply to

**Example:**

```json
{
  "text": "Hello, world! 🌍",
  "reply_to": "1234567890"
}
```

### `search_tweets`

Search for tweets.

**Parameters:**

- `query` (string): Search query
- `max_results` (integer, optional): Maximum number of results (1-100, default: 10)
- `include_users` (boolean, optional): Include user information (default: false)
- `include_metrics` (boolean, optional): Include tweet metrics (default: false)

**Example:**

```json
{
  "query": "MCP OR \"Model Context Protocol\"",
  "max_results": 20,
  "include_users": true,
  "include_metrics": true
}
```

### `get_tweet`

Get a specific tweet by ID.

**Parameters:**

- `tweet_id` (string): The tweet ID

**Example:**

```json
{
  "tweet_id": "1234567890"
}
```

### `get_user_tweets`

Get a user's recent tweets.

**Parameters:**

- `identifier` (string): Username or user ID
- `is_user_id` (boolean, optional): Whether the identifier is a user ID (default: false)
- `max_results` (integer, optional): Maximum number of tweets (1-100, default: 10)

**Example:**

```json
{
  "identifier": "elonmusk",
  "max_results": 5
}
```

## Library Usage

You can also use this as a Rust library:

```toml
[dependencies]
x-mcp-server = "0.1"
```

```rust
use x_mcp_server::{XClient, auth::OAuthCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credentials = OAuthCredentials::from_env()?;
    let client = XClient::new(credentials);

    // Get user info
    let user = client.get_user_by_username("elonmusk").await?;
    println!("{:#?}", user);

    // Post a tweet
    let tweet = client.post_tweet("Hello from Rust! 🦀", None).await?;
    println!("Posted tweet: {}", tweet.id);

    Ok(())
}
```

## MCP Integration

This server implements the [Model Context Protocol](https://modelcontextprotocol.io/) specification. You can integrate it with any MCP-compatible client:

### Claude Desktop

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "x-mcp-server": {
      "command": "x-mcp-server",
      "env": {
        "X_CONSUMER_KEY": "your_key",
        "X_CONSUMER_SECRET": "your_secret",
        "X_ACCESS_TOKEN": "your_token",
        "X_ACCESS_TOKEN_SECRET": "your_token_secret"
      }
    }
  }
}
```

### Other MCP Clients

Any MCP client can connect to this server using stdio transport.

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running with Debug Logs

```bash
RUST_LOG=debug cargo run
```

## API Limits

Please be aware of X API rate limits:

- **User lookup**: 300 requests per 15-minute window
- **Tweet posting**: 300 tweets per 15-minute window
- **Search**: 180 requests per 15-minute window
- **User timeline**: 1500 requests per 15-minute window

The server does not implement rate limiting, so ensure your usage stays within these limits.

## Security

- API credentials are never logged or exposed
- OAuth 1.0a signatures are generated securely
- All HTTP requests use HTTPS

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Related

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [X API Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)

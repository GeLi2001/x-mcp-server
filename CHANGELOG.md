# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-01-XX

### Added

- Initial release of X MCP Server
- OAuth 1.0a authentication for X API
- MCP tools for user information retrieval
- Tweet posting and retrieval functionality
- Tweet search capabilities
- User timeline access
- Comprehensive error handling
- Full async/await support with Tokio
- Integration tests and documentation
- Support for environment-based configuration

### Features

- `get_user` - Get user information by username or ID
- `post_tweet` - Post new tweets with optional reply functionality
- `search_tweets` - Search tweets with filtering options
- `get_tweet` - Retrieve specific tweets by ID
- `get_user_tweets` - Get user's recent timeline

### Security

- Secure OAuth 1.0a signature generation
- Environment variable configuration for credentials
- No credential logging or exposure

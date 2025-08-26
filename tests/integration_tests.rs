//! Integration tests for X MCP Server

use serde_json::json;
use x_mcp_server::{
    auth::OAuthCredentials,
    client::XClient,
    tools::{GetUserArgs, SearchTweetsArgs},
    types::SearchTweetsParams,
};

/// Test that we can create OAuth credentials
#[test]
fn test_oauth_credentials_creation() {
    let creds = OAuthCredentials::new(
        "test_key".to_string(),
        "test_secret".to_string(),
        "test_token".to_string(),
        "test_token_secret".to_string(),
    );
    
    assert_eq!(creds.consumer_key, "test_key");
    assert_eq!(creds.consumer_secret, "test_secret");
    assert_eq!(creds.access_token, "test_token");
    assert_eq!(creds.access_token_secret, "test_token_secret");
}

/// Test that we can create a client
#[test]
fn test_client_creation() {
    let creds = OAuthCredentials::new(
        "test_key".to_string(),
        "test_secret".to_string(),
        "test_token".to_string(),
        "test_token_secret".to_string(),
    );
    
    let _client = XClient::new(creds);
    // Just test that creation works
    assert!(true);
}

/// Test tool argument parsing
#[test]
fn test_tool_args_parsing() {
    let user_args_json = json!({
        "identifier": "testuser",
        "is_user_id": false
    });
    
    let args: GetUserArgs = serde_json::from_value(user_args_json).unwrap();
    assert_eq!(args.identifier, "testuser");
    assert!(!args.is_user_id);
    
    let search_args_json = json!({
        "query": "test query",
        "max_results": 20,
        "include_users": true
    });
    
    let search_args: SearchTweetsArgs = serde_json::from_value(search_args_json).unwrap();
    assert_eq!(search_args.query, "test query");
    assert_eq!(search_args.max_results, 20);
    assert!(search_args.include_users);
}

/// Test search params conversion
#[test]
fn test_search_params() {
    let params = SearchTweetsParams {
        query: "test query".to_string(),
        max_results: Some(50),
        tweet_fields: Some(vec!["id".to_string(), "text".to_string()]),
        user_fields: Some(vec!["username".to_string()]),
        expansions: Some(vec!["author_id".to_string()]),
    };
    
    assert_eq!(params.query, "test query");
    assert_eq!(params.max_results, Some(50));
    assert!(params.tweet_fields.is_some());
    assert!(params.user_fields.is_some());
    assert!(params.expansions.is_some());
}

//! Integration tests for X MCP Server

use serde_json::json;
use x_mcp_server::{
    client::XClient,
    server::{GetUserArgs, SearchTweetsArgs},
    types::SearchTweetsParams,
};

/// Test that we can create a client
#[test]
fn test_client_creation() {
    let _client = XClient::new("test_bearer_token".to_string());
    // Just test that creation works - we can't test much without a real token
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

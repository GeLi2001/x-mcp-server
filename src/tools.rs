//! MCP tools for X API operations

use crate::client::XClient;
use crate::error::XResult;
use crate::types::SearchTweetsParams;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Tool arguments for getting user information
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserArgs {
    /// Username (without @) or user ID
    pub identifier: String,
    /// Whether the identifier is a user ID (true) or username (false)
    #[serde(default)]
    pub is_user_id: bool,
}

/// Tool arguments for posting a tweet
#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetArgs {
    /// The text content of the tweet
    pub text: String,
    /// Optional tweet ID to reply to
    pub reply_to: Option<String>,
}

/// Tool arguments for searching tweets
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTweetArgs {
    /// The tweet ID
    pub tweet_id: String,
}

/// Tool arguments for getting user's tweets
#[derive(Debug, Serialize, Deserialize)]
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

/// Get user information by username or user ID
pub async fn get_user(args: GetUserArgs, client: &XClient) -> XResult<Value> {
    let user = if args.is_user_id {
        client.get_user_by_id(&args.identifier).await?
    } else {
        client.get_user_by_username(&args.identifier).await?
    };

    match user {
        Some(user) => Ok(json!({
            "success": true,
            "user": user
        })),
        None => Ok(json!({
            "success": false,
            "error": "User not found"
        })),
    }
}

/// Post a new tweet
pub async fn post_tweet(args: PostTweetArgs, client: &XClient) -> XResult<Value> {
    let tweet = client
        .post_tweet(&args.text, args.reply_to.as_deref())
        .await?;

    Ok(json!({
        "success": true,
        "tweet": tweet
    }))
}

/// Search for tweets
pub async fn search_tweets(args: SearchTweetsArgs, client: &XClient) -> XResult<Value> {
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

    let tweets = client.search_tweets(search_params).await?;

    Ok(json!({
        "success": true,
        "tweets": tweets,
        "count": tweets.len()
    }))
}

/// Get a specific tweet by ID
pub async fn get_tweet(args: GetTweetArgs, client: &XClient) -> XResult<Value> {
    let tweet = client.get_tweet(&args.tweet_id).await?;

    match tweet {
        Some(tweet) => Ok(json!({
            "success": true,
            "tweet": tweet
        })),
        None => Ok(json!({
            "success": false,
            "error": "Tweet not found"
        })),
    }
}

/// Get user's recent tweets
pub async fn get_user_tweets(args: GetUserTweetsArgs, client: &XClient) -> XResult<Value> {
    // First, get the user to get their ID if we have a username
    let user_id = if args.is_user_id {
        args.identifier.clone()
    } else {
        match client.get_user_by_username(&args.identifier).await? {
            Some(user) => user.id,
            None => {
                return Ok(json!({
                    "success": false,
                    "error": "User not found"
                }))
            }
        }
    };

    let tweets = client
        .get_user_tweets(&user_id, Some(args.max_results.min(100)))
        .await?;

    Ok(json!({
        "success": true,
        "tweets": tweets,
        "count": tweets.len(),
        "user_id": user_id
    }))
}

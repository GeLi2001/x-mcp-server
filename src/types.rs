//! Type definitions for X API responses

use serde::{Deserialize, Serialize};

/// User information from X API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub description: Option<String>,
    pub public_metrics: Option<UserMetrics>,
    pub profile_image_url: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<String>,
}

/// User metrics (followers, following, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetrics {
    pub followers_count: u64,
    pub following_count: u64,
    pub tweet_count: u64,
    pub listed_count: u64,
}

/// Tweet information from X API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub public_metrics: Option<TweetMetrics>,
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
}

/// Tweet metrics (likes, retweets, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweetMetrics {
    pub retweet_count: u64,
    pub like_count: u64,
    pub reply_count: u64,
    pub quote_count: u64,
}

/// Context annotation for tweets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: Domain,
    pub entity: Entity,
}

/// Domain information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// Entity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// Referenced tweet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub tweet_type: String,
    pub id: String,
}

/// Response wrapper for X API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XResponse<T> {
    pub data: Option<T>,
    pub includes: Option<Includes>,
    pub errors: Option<Vec<XApiError>>,
    pub meta: Option<serde_json::Value>,
}

/// Includes section for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Includes {
    pub users: Option<Vec<User>>,
    pub tweets: Option<Vec<Tweet>>,
}

/// X API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiError {
    pub title: String,
    pub detail: Option<String>,
    pub resource_type: Option<String>,
    pub parameter: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}

/// Request to post a tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTweetRequest {
    pub text: String,
    pub reply: Option<ReplySettings>,
}

/// Reply settings for tweets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplySettings {
    pub in_reply_to_tweet_id: Option<String>,
}

/// Search tweets request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTweetsParams {
    pub query: String,
    pub max_results: Option<u32>,
    pub tweet_fields: Option<Vec<String>>,
    pub user_fields: Option<Vec<String>>,
    pub expansions: Option<Vec<String>>,
}

//! X API client implementation

use crate::auth::OAuthCredentials;
use crate::error::{XError, XResult};
use crate::types::{
    PostTweetRequest, SearchTweetsParams, Tweet, User, XResponse,
};
use reqwest::Client;
use serde_json::Value;
use std::collections::BTreeMap;

/// X API client
#[derive(Debug, Clone)]
pub struct XClient {
    client: Client,
    credentials: OAuthCredentials,
    base_url: String,
}

impl XClient {
    /// Create a new X API client
    pub fn new(credentials: OAuthCredentials) -> Self {
        Self {
            client: Client::new(),
            credentials,
            base_url: "https://api.twitter.com/2".to_string(),
        }
    }

    /// Create client from environment variables
    pub fn from_env() -> XResult<Self> {
        let credentials = OAuthCredentials::from_env()?;
        Ok(Self::new(credentials))
    }

    /// Get user information by username
    pub async fn get_user_by_username(&self, username: &str) -> XResult<Option<User>> {
        let url = format!("{}/users/by/username/{}", self.base_url, username);
        let params = BTreeMap::from([
            ("user.fields".to_string(), "id,name,username,description,public_metrics,profile_image_url,verified,created_at".to_string()),
        ]);

        let response: XResponse<User> = self.make_request("GET", &url, &params, None).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(response.data)
    }

    /// Get user information by user ID
    pub async fn get_user_by_id(&self, user_id: &str) -> XResult<Option<User>> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        let params = BTreeMap::from([
            ("user.fields".to_string(), "id,name,username,description,public_metrics,profile_image_url,verified,created_at".to_string()),
        ]);

        let response: XResponse<User> = self.make_request("GET", &url, &params, None).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(response.data)
    }

    /// Post a new tweet
    pub async fn post_tweet(&self, text: &str, reply_to: Option<&str>) -> XResult<Tweet> {
        let url = format!("{}/tweets", self.base_url);
        
        let mut request = PostTweetRequest {
            text: text.to_string(),
            reply: None,
        };

        if let Some(reply_id) = reply_to {
            request.reply = Some(crate::types::ReplySettings {
                in_reply_to_tweet_id: Some(reply_id.to_string()),
            });
        }

        let body = serde_json::to_value(request)?;
        let response: XResponse<Tweet> = self.make_request("POST", &url, &BTreeMap::new(), Some(body)).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        response.data.ok_or_else(|| XError::Api {
            status: 400,
            message: "No data returned from post tweet".to_string(),
        })
    }

    /// Search for tweets
    pub async fn search_tweets(&self, params: SearchTweetsParams) -> XResult<Vec<Tweet>> {
        let url = format!("{}/tweets/search/recent", self.base_url);
        
        let mut query_params = BTreeMap::new();
        query_params.insert("query".to_string(), params.query);
        
        if let Some(max_results) = params.max_results {
            query_params.insert("max_results".to_string(), max_results.to_string());
        }
        
        if let Some(tweet_fields) = params.tweet_fields {
            query_params.insert("tweet.fields".to_string(), tweet_fields.join(","));
        }
        
        if let Some(user_fields) = params.user_fields {
            query_params.insert("user.fields".to_string(), user_fields.join(","));
        }
        
        if let Some(expansions) = params.expansions {
            query_params.insert("expansions".to_string(), expansions.join(","));
        }

        let response: XResponse<Vec<Tweet>> = self.make_request("GET", &url, &query_params, None).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(response.data.unwrap_or_default())
    }

    /// Get a tweet by ID
    pub async fn get_tweet(&self, tweet_id: &str) -> XResult<Option<Tweet>> {
        let url = format!("{}/tweets/{}", self.base_url, tweet_id);
        let params = BTreeMap::from([
            ("tweet.fields".to_string(), "id,text,author_id,created_at,public_metrics,context_annotations,referenced_tweets".to_string()),
            ("expansions".to_string(), "author_id".to_string()),
        ]);

        let response: XResponse<Tweet> = self.make_request("GET", &url, &params, None).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(response.data)
    }

    /// Get user's recent tweets
    pub async fn get_user_tweets(&self, user_id: &str, max_results: Option<u32>) -> XResult<Vec<Tweet>> {
        let url = format!("{}/users/{}/tweets", self.base_url, user_id);
        let mut params = BTreeMap::from([
            ("tweet.fields".to_string(), "id,text,author_id,created_at,public_metrics".to_string()),
        ]);
        
        if let Some(max) = max_results {
            params.insert("max_results".to_string(), max.to_string());
        }

        let response: XResponse<Vec<Tweet>> = self.make_request("GET", &url, &params, None).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(response.data.unwrap_or_default())
    }

    /// Make an authenticated request to the X API
    async fn make_request<T>(
        &self,
        method: &str,
        url: &str,
        params: &BTreeMap<String, String>,
        body: Option<Value>,
    ) -> XResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let auth_header = self.credentials.generate_auth_header(method, url, params)?;
        
        let mut request = self.client.request(
            method.parse().map_err(|_| XError::Generic("Invalid HTTP method".to_string()))?,
            url,
        );

        request = request.header("Authorization", auth_header);
        request = request.header("Content-Type", "application/json");

        // Add query parameters for GET requests
        if method.to_uppercase() == "GET" && !params.is_empty() {
            request = request.query(params);
        }

        // Add body for POST/PUT requests
        if let Some(body_value) = body {
            request = request.json(&body_value);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let response_text = response.text().await?;
        let parsed: T = serde_json::from_str(&response_text).map_err(|e| {
            tracing::error!("Failed to parse response: {}", response_text);
            XError::Json(e)
        })?;

        Ok(parsed)
    }
}

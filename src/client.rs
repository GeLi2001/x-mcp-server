//! X API client implementation using Bearer Token

use crate::error::{XError, XResult};
use crate::types::{SearchTweetsParams, Tweet, User, XResponse};
use reqwest::Client;

/// X API client
#[derive(Debug, Clone)]
pub struct XClient {
    client: Client,
    bearer_token: String,
    base_url: String,
}

impl XClient {
    /// Create a new X API client
    pub fn new(bearer_token: String) -> Self {
        Self {
            client: Client::new(),
            bearer_token,
            base_url: "https://api.twitter.com/2".to_string(),
        }
    }

    /// Create client from environment variables
    pub fn from_env() -> XResult<Self> {
        let bearer_token = std::env::var("X_BEARER_TOKEN")
            .map_err(|_| XError::Config("X_BEARER_TOKEN not found".to_string()))?;
        Ok(Self::new(bearer_token))
    }

    /// Get user information by username
    pub async fn get_user_by_username(&self, username: &str) -> XResult<Option<User>> {
        let url = format!("{}/users/by/username/{}", self.base_url, username);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&[("user.fields", "id,name,username,description,public_metrics,profile_image_url,verified,created_at")])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: XResponse<User> = response.json().await?;
        
        if let Some(errors) = api_response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(api_response.data)
    }

    /// Get user information by user ID
    pub async fn get_user_by_id(&self, user_id: &str) -> XResult<Option<User>> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&[("user.fields", "id,name,username,description,public_metrics,profile_image_url,verified,created_at")])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: XResponse<User> = response.json().await?;
        
        if let Some(errors) = api_response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(api_response.data)
    }

    /// Search for tweets
    pub async fn search_tweets(&self, params: SearchTweetsParams) -> XResult<Vec<Tweet>> {
        let url = format!("{}/tweets/search/recent", self.base_url);
        
        let mut query_params = vec![("query", params.query)];
        
        if let Some(max_results) = params.max_results {
            query_params.push(("max_results", max_results.to_string()));
        }
        
        if let Some(tweet_fields) = params.tweet_fields {
            query_params.push(("tweet.fields", tweet_fields.join(",")));
        }
        
        if let Some(user_fields) = params.user_fields {
            query_params.push(("user.fields", user_fields.join(",")));
        }
        
        if let Some(expansions) = params.expansions {
            query_params.push(("expansions", expansions.join(",")));
        }

        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&query_params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: XResponse<Vec<Tweet>> = response.json().await?;
        
        if let Some(errors) = api_response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(api_response.data.unwrap_or_default())
    }

    /// Get a tweet by ID
    pub async fn get_tweet(&self, tweet_id: &str) -> XResult<Option<Tweet>> {
        let url = format!("{}/tweets/{}", self.base_url, tweet_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&[
                ("tweet.fields", "id,text,author_id,created_at,public_metrics,context_annotations,referenced_tweets"),
                ("expansions", "author_id"),
            ])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: XResponse<Tweet> = response.json().await?;
        
        if let Some(errors) = api_response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(api_response.data)
    }

    /// Get user's recent tweets
    pub async fn get_user_tweets(&self, user_id: &str, max_results: Option<u32>) -> XResult<Vec<Tweet>> {
        let url = format!("{}/users/{}/tweets", self.base_url, user_id);
        
        let mut query_params = vec![
            ("tweet.fields", "id,text,author_id,created_at,public_metrics".to_string()),
        ];
        
        if let Some(max) = max_results {
            query_params.push(("max_results", max.to_string()));
        }

        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&query_params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(XError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: XResponse<Vec<Tweet>> = response.json().await?;
        
        if let Some(errors) = api_response.errors {
            if !errors.is_empty() {
                return Err(XError::Api {
                    status: 400,
                    message: format!("API errors: {:?}", errors),
                });
            }
        }

        Ok(api_response.data.unwrap_or_default())
    }
}
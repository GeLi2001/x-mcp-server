//! Authentication module for X API OAuth 1.0a

use crate::error::{XError, XResult};
use base64::prelude::*;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::BTreeMap;

type HmacSha1 = Hmac<Sha1>;

/// OAuth 1.0a credentials for X API
#[derive(Debug, Clone)]
pub struct OAuthCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl OAuthCredentials {
    /// Create new OAuth credentials
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Self {
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        }
    }

    /// Load credentials from environment variables
    pub fn from_env() -> XResult<Self> {
        let consumer_key = std::env::var("X_CONSUMER_KEY")
            .map_err(|_| XError::Config("X_CONSUMER_KEY not found".to_string()))?;
        let consumer_secret = std::env::var("X_CONSUMER_SECRET")
            .map_err(|_| XError::Config("X_CONSUMER_SECRET not found".to_string()))?;
        let access_token = std::env::var("X_ACCESS_TOKEN")
            .map_err(|_| XError::Config("X_ACCESS_TOKEN not found".to_string()))?;
        let access_token_secret = std::env::var("X_ACCESS_TOKEN_SECRET")
            .map_err(|_| XError::Config("X_ACCESS_TOKEN_SECRET not found".to_string()))?;

        Ok(Self::new(
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        ))
    }

    /// Generate OAuth 1.0a authorization header
    pub fn generate_auth_header(
        &self,
        method: &str,
        url: &str,
        params: &BTreeMap<String, String>,
    ) -> XResult<String> {
        let nonce = generate_nonce();
        let timestamp = chrono::Utc::now().timestamp().to_string();

        let mut oauth_params = BTreeMap::new();
        oauth_params.insert("oauth_consumer_key".to_string(), self.consumer_key.clone());
        oauth_params.insert("oauth_nonce".to_string(), nonce);
        oauth_params.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        oauth_params.insert("oauth_timestamp".to_string(), timestamp);
        oauth_params.insert("oauth_token".to_string(), self.access_token.clone());
        oauth_params.insert("oauth_version".to_string(), "1.0".to_string());

        // Combine OAuth params with request params
        let mut all_params = oauth_params.clone();
        all_params.extend(params.clone());

        // Generate signature
        let signature = self.generate_signature(method, url, &all_params)?;
        oauth_params.insert("oauth_signature".to_string(), signature);

        // Build authorization header
        let auth_header = oauth_params
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, percent_encode(v)))
            .collect::<Vec<_>>()
            .join(", ");

        Ok(format!("OAuth {}", auth_header))
    }

    /// Generate OAuth signature
    fn generate_signature(
        &self,
        method: &str,
        url: &str,
        params: &BTreeMap<String, String>,
    ) -> XResult<String> {
        // Create parameter string
        let param_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        // Create signature base string
        let base_string = format!(
            "{}&{}&{}",
            method.to_uppercase(),
            percent_encode(url),
            percent_encode(&param_string)
        );

        // Create signing key
        let signing_key = format!(
            "{}&{}",
            percent_encode(&self.consumer_secret),
            percent_encode(&self.access_token_secret)
        );

        // Generate HMAC-SHA1 signature
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())
            .map_err(|e| XError::Auth(format!("Failed to create HMAC: {}", e)))?;
        mac.update(base_string.as_bytes());
        let result = mac.finalize();
        let signature = BASE64_STANDARD.encode(result.into_bytes());

        Ok(signature)
    }
}

/// Generate a random nonce for OAuth
fn generate_nonce() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=25 => (b'A' + idx) as char,
                26..=51 => (b'a' + (idx - 26)) as char,
                _ => (b'0' + (idx - 52)) as char,
            }
        })
        .collect()
}

/// Percent encode a string for OAuth
fn percent_encode(s: &str) -> String {
    urlencoding::encode(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_generation() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        
        assert_eq!(nonce1.len(), 32);
        assert_eq!(nonce2.len(), 32);
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_percent_encoding() {
        assert_eq!(percent_encode("hello world"), "hello%20world");
        assert_eq!(percent_encode("test@example.com"), "test%40example.com");
    }
}

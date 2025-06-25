//! Authentication types and utilities

use crate::core::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Authentication method for Hume API
#[derive(Debug, Clone)]
pub enum Auth {
    /// API key authentication
    ApiKey(String),
    /// Access token authentication
    AccessToken(AuthToken),
}

impl Auth {
    /// Create a new API key authentication
    pub fn api_key(key: impl Into<String>) -> Self {
        Self::ApiKey(key.into())
    }

    /// Create a new access token authentication
    pub fn access_token(token: AuthToken) -> Self {
        Self::AccessToken(token)
    }

    /// Get the authorization header value
    pub fn header_value(&self) -> Option<(&'static str, String)> {
        match self {
            Self::ApiKey(key) => Some(("X-Hume-Api-Key", key.clone())),
            Self::AccessToken(token) => Some(("Authorization", format!("Bearer {}", token.access_token))),
        }
    }

    /// Get the query parameter for WebSocket authentication
    pub fn query_param(&self) -> (&'static str, String) {
        match self {
            Self::ApiKey(key) => ("api_key", key.clone()),
            Self::AccessToken(token) => ("access_token", token.access_token.clone()),
        }
    }

    /// Check if the authentication is expired (only applicable for access tokens)
    pub fn is_expired(&self) -> bool {
        match self {
            Self::ApiKey(_) => false,
            Self::AccessToken(token) => token.is_expired(),
        }
    }
}

/// Access token for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// The access token
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Expiration time in seconds
    pub expires_in: u64,
    /// When the token was created
    #[serde(skip)]
    pub created_at: DateTime<Utc>,
}

impl AuthToken {
    /// Create a new auth token
    pub fn new(access_token: String, token_type: String, expires_in: u64) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
            created_at: Utc::now(),
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let expiry = self.created_at + chrono::Duration::seconds(self.expires_in as i64);
        Utc::now() >= expiry
    }

    /// Get the remaining time until expiration in seconds
    pub fn time_until_expiry(&self) -> Option<u64> {
        let expiry = self.created_at + chrono::Duration::seconds(self.expires_in as i64);
        let now = Utc::now();
        if expiry > now {
            Some((expiry - now).num_seconds() as u64)
        } else {
            None
        }
    }
}

/// Request to get an access token
#[derive(Debug, Serialize)]
pub struct AccessTokenRequest {
    /// API key
    pub api_key: String,
    /// Secret key
    pub secret_key: String,
}

/// Response from access token endpoint
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    /// The access token
    pub access_token: String,
    /// Token type
    pub token_type: String,
    /// Expiration time in seconds
    pub expires_in: u64,
}

impl From<AccessTokenResponse> for AuthToken {
    fn from(response: AccessTokenResponse) -> Self {
        AuthToken::new(
            response.access_token,
            response.token_type,
            response.expires_in,
        )
    }
}

/// Generate an access token from API key and secret key
pub async fn generate_access_token(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    secret_key: &str,
) -> Result<AuthToken> {
    let url = format!("{}/oauth2-cc/token", base_url);
    
    let request = AccessTokenRequest {
        api_key: api_key.to_string(),
        secret_key: secret_key.to_string(),
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let token_response: AccessTokenResponse = response.json().await?;
        Ok(token_response.into())
    } else {
        let status = response.status().as_u16();
        let body = response.text().await.ok();
        Err(Error::api(
            status,
            "Failed to generate access token".to_string(),
            None,
            body,
        ))
    }
}
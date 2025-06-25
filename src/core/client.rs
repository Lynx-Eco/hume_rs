//! Main Hume client implementation

use crate::core::{
    auth::{generate_access_token, Auth, AuthToken},
    error::{Error, Result},
    http::{HttpClient, HttpClientBuilder as InternalHttpClientBuilder},
};
use std::sync::Arc;
use std::time::Duration;

/// The main client for interacting with Hume APIs
#[derive(Debug, Clone)]
pub struct HumeClient {
    pub(crate) http: Arc<HttpClient>,
    pub(crate) base_url: String,
}

impl HumeClient {
    /// Create a new client with an API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        HumeClientBuilder::new(api_key).build()
    }

    /// Create a new client from environment variables
    /// 
    /// Reads the API key from the HUME_API_KEY environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("HUME_API_KEY")
            .map_err(|_| Error::config("HUME_API_KEY environment variable not set"))?;
        Self::new(api_key)
    }

    /// Create a new client builder
    pub fn builder() -> HumeClientBuilder {
        HumeClientBuilder::default()
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get a reference to the HTTP client
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Get the authentication method
    pub fn auth(&self) -> Option<&Auth> {
        self.http.auth.as_ref()
    }

    /// Create a TTS client
    pub fn tts(&self) -> crate::tts::TtsClient {
        crate::tts::TtsClient::from(self.clone())
    }

    /// Create an Expression Measurement client
    pub fn expression(&self) -> crate::expression_measurement::ExpressionMeasurementClient {
        crate::expression_measurement::ExpressionMeasurementClient::from(self.clone())
    }

    /// Create an EVI client
    pub fn evi(&self) -> crate::evi::EviClient {
        crate::evi::EviClient::from(self.clone())
    }

    /// Generate an access token using API key and secret key
    pub async fn generate_access_token(&self, api_key: &str, secret_key: &str) -> Result<AuthToken> {
        generate_access_token(&self.http.client, &self.base_url, api_key, secret_key).await
    }
}

/// Builder for creating Hume clients
#[derive(Debug, Default)]
pub struct HumeClientBuilder {
    api_key: Option<String>,
    access_token: Option<AuthToken>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl HumeClientBuilder {
    /// Create a new builder with an API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    /// Set the API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self.access_token = None;
        self
    }

    /// Set the API key (alias for consistency with documentation)
    pub fn with_api_key(self, api_key: impl Into<String>) -> Self {
        self.api_key(api_key)
    }

    /// Set the access token
    pub fn access_token(mut self, token: AuthToken) -> Self {
        self.access_token = Some(token);
        self.api_key = None;
        self
    }

    /// Set the access token (alias for consistency with documentation)
    pub fn with_access_token(self, token: impl Into<String>) -> Self {
        let auth_token = AuthToken {
            access_token: token.into(),
            token_type: "Bearer".to_string(),
            expires_in: 3600, // Default 1 hour
            created_at: chrono::Utc::now(),
        };
        self.access_token(auth_token)
    }

    /// Set the base URL (defaults to https://api.hume.ai)
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the base URL (alias for consistency with documentation)
    pub fn with_base_url(self, base_url: impl Into<String>) -> Self {
        self.base_url(base_url)
    }

    /// Set the default request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries for failed requests
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<HumeClient> {
        let base_url = self.base_url.unwrap_or_else(|| crate::DEFAULT_BASE_URL.to_string());

        let auth = if let Some(token) = self.access_token {
            Some(Auth::access_token(token))
        } else if let Some(api_key) = self.api_key {
            Some(Auth::api_key(api_key))
        } else {
            return Err(Error::config("Either api_key or access_token must be provided"));
        };

        let mut http_builder = InternalHttpClientBuilder::new(base_url.clone());
        
        if let Some(auth) = auth {
            http_builder = http_builder.auth(auth);
        }
        
        if let Some(timeout) = self.timeout {
            http_builder = http_builder.timeout(timeout);
        }
        
        if let Some(max_retries) = self.max_retries {
            http_builder = http_builder.max_retries(max_retries);
        }

        let http = http_builder.build()?;

        Ok(HumeClient {
            http: Arc::new(http),
            base_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_api_key() {
        let client = HumeClientBuilder::new("test-api-key")
            .build()
            .expect("Failed to build client");
        
        assert_eq!(client.base_url(), crate::DEFAULT_BASE_URL);
    }

    #[test]
    fn test_builder_with_custom_base_url() {
        let custom_url = "https://custom.hume.ai";
        let client = HumeClientBuilder::new("test-api-key")
            .base_url(custom_url)
            .build()
            .expect("Failed to build client");
        
        assert_eq!(client.base_url(), custom_url);
    }

    #[test]
    fn test_builder_requires_auth() {
        let result = HumeClientBuilder::default().build();
        assert!(result.is_err());
    }
}
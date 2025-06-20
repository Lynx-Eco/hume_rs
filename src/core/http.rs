//! HTTP client implementation with retry logic

use crate::core::{
    auth::Auth,
    error::{ApiErrorDetails, Error, Result},
    request::RequestOptions,
};
use backoff::{ExponentialBackoff, future::retry, Error as BackoffError};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use reqwest::{header::HeaderMap, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::{pin::Pin, time::Duration};
use tracing::debug;

/// HTTP client with retry logic and error handling
#[derive(Debug, Clone)]
pub struct HttpClient {
    pub(crate) client: reqwest::Client,
    base_url: String,
    pub(crate) auth: Option<Auth>,
    default_timeout: Duration,
    max_retries: u32,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(base_url: String, auth: Option<Auth>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(format!("hume-rust-sdk/{}", crate::SDK_VERSION))
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url,
            auth,
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
        })
    }

    /// Set the authentication method
    pub fn set_auth(&mut self, auth: Auth) {
        self.auth = Some(auth);
    }

    /// Set the default timeout
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Set the maximum number of retries
    pub fn set_max_retries(&mut self, max_retries: u32) {
        self.max_retries = max_retries;
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str, options: Option<RequestOptions>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::GET, path, None::<()>, options).await
    }

    /// Make a POST request
    pub async fn post<B, T>(&self, path: &str, body: B, options: Option<RequestOptions>) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.request(Method::POST, path, Some(body), options).await
    }

    /// Make a PUT request
    pub async fn put<B, T>(&self, path: &str, body: B, options: Option<RequestOptions>) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.request(Method::PUT, path, Some(body), options).await
    }

    /// Make a PATCH request
    pub async fn patch<B, T>(&self, path: &str, body: B, options: Option<RequestOptions>) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.request(Method::PATCH, path, Some(body), options).await
    }

    /// Make a DELETE request
    pub async fn delete<T>(&self, path: &str, options: Option<RequestOptions>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::DELETE, path, None::<()>, options).await
    }

    /// Make a request that returns raw bytes
    pub async fn request_bytes(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
        options: Option<RequestOptions>,
    ) -> Result<Bytes> {
        let response = self.execute_request(method, path, body, options).await?;
        Ok(response.bytes().await?)
    }

    /// Make a request that returns a stream
    pub async fn request_stream(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
        options: Option<RequestOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>> {
        let response = self.execute_request(method, path, body, options).await?;
        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(Error::from));
        Ok(Box::pin(stream))
    }

    /// Make a request with automatic retry
    async fn request<B, T>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
        options: Option<RequestOptions>,
    ) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let response = self.execute_request(method, path, body, options).await?;
        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            response.json::<T>().await.map_err(Error::from)
        } else {
            let body_text = response.text().await.ok();
            self.handle_error_response(status, headers, body_text)
        }
    }

    /// Execute a request with retry logic
    async fn execute_request(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
        options: Option<RequestOptions>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let options = options.unwrap_or_default();
        let max_retries = options.max_retries.unwrap_or(self.max_retries);

        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(Duration::from_secs(60)),
            ..Default::default()
        };

        retry(backoff, || async {
            let mut request = self.client.request(method.clone(), &url);

            // Set auth header
            if let Some(auth) = &self.auth {
                if let Some((header_name, header_value)) = auth.header_value() {
                    request = request.header(header_name, header_value);
                }
            }

            // Set custom headers
            for (key, value) in &options.headers {
                request = request.header(key, value);
            }

            // Set query parameters
            for (key, value) in &options.query {
                request = request.query(&[(key, value)]);
            }

            // Set timeout
            let timeout = options.timeout.unwrap_or(self.default_timeout);
            request = request.timeout(timeout);

            // Set body
            if let Some(body) = &body {
                request = request.json(body);
            }

            let response = request.send().await.map_err(|e| {
                if e.is_timeout() {
                    BackoffError::permanent(Error::Timeout)
                } else if self.should_retry(&e) {
                    debug!("Retrying request due to error: {}", e);
                    BackoffError::transient(Error::from(e))
                } else {
                    BackoffError::permanent(Error::from(e))
                }
            })?;

            let status = response.status();
            if self.should_retry_status(status) {
                if max_retries > 0 {
                    debug!("Retrying request due to status: {}", status);
                    Err(BackoffError::transient(Error::other(format!(
                        "Received retryable status: {}",
                        status
                    ))))
                } else {
                    Ok(response)
                }
            } else {
                Ok(response)
            }
        })
        .await
    }

    /// Check if an error should trigger a retry
    fn should_retry(&self, error: &reqwest::Error) -> bool {
        error.is_connect() || error.is_timeout()
    }

    /// Check if a status code should trigger a retry
    fn should_retry_status(&self, status: StatusCode) -> bool {
        status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
    }

    /// Handle error responses
    fn handle_error_response<T>(&self, status: StatusCode, headers: HeaderMap, body: Option<String>) -> Result<T> {
        let retry_after = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimit { retry_after });
        }

        // Try to parse error details from body
        let (message, code) = if let Some(body_text) = &body {
            if let Ok(error_details) = serde_json::from_str::<ApiErrorDetails>(body_text) {
                (error_details.message, error_details.code)
            } else {
                (format!("HTTP {} error", status.as_u16()), None)
            }
        } else {
            (format!("HTTP {} error", status.as_u16()), None)
        };

        Err(Error::api(status.as_u16(), message, code, body))
    }
}

/// Builder for creating HTTP clients
pub struct HttpClientBuilder {
    base_url: String,
    auth: Option<Auth>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl HttpClientBuilder {
    /// Create a new builder
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            auth: None,
            timeout: None,
            max_retries: None,
        }
    }

    /// Set the authentication method
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Set the default timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the HTTP client
    pub fn build(self) -> Result<HttpClient> {
        let mut client = HttpClient::new(self.base_url, self.auth)?;
        
        if let Some(timeout) = self.timeout {
            client.set_default_timeout(timeout);
        }
        
        if let Some(max_retries) = self.max_retries {
            client.set_max_retries(max_retries);
        }
        
        Ok(client)
    }
}
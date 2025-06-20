//! Error types for the Hume SDK

use thiserror::Error;

/// A type alias for `Result<T, hume::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for Hume SDK operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP client error
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// API error returned by Hume
    #[error("API error (status {status}): {message}")]
    Api {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
        /// Optional error code
        code: Option<String>,
        /// Raw response body
        body: Option<String>,
    },

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Base64 decode error
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// Timeout error
    #[error("Request timed out")]
    Timeout,

    /// Rate limit error
    #[error("Rate limit exceeded")]
    RateLimit {
        /// Optional retry-after header value
        retry_after: Option<u64>,
    },

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a new API error
    pub fn api(status: u16, message: String, code: Option<String>, body: Option<String>) -> Self {
        Self::Api {
            status,
            message,
            code,
            body,
        }
    }

    /// Create a new authentication error
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }

    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new other error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }

    /// Returns true if this is an API error
    pub fn is_api_error(&self) -> bool {
        matches!(self, Self::Api { .. })
    }

    /// Returns true if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimit { .. })
    }

    /// Returns true if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }

    /// Get the status code if this is an API error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// API error details returned by Hume
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiErrorDetails {
    /// Error message
    pub message: String,
    /// Error code
    pub code: Option<String>,
    /// Field-specific errors for validation
    pub errors: Option<Vec<FieldError>>,
}

/// Field-specific error for validation
#[derive(Debug, Clone, serde::Deserialize)]
pub struct FieldError {
    /// Field name
    pub field: String,
    /// Error message
    pub message: String,
    /// Error code
    pub code: Option<String>,
}
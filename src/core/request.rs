//! Request configuration options

use std::collections::HashMap;
use std::time::Duration;

/// Options for customizing HTTP requests
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// Additional headers to include in the request
    pub headers: HashMap<String, String>,
    /// Query parameters to include in the request
    pub query: HashMap<String, String>,
    /// Request timeout
    pub timeout: Option<Duration>,
    /// Maximum number of retries
    pub max_retries: Option<u32>,
}

impl RequestOptions {
    /// Create a new RequestOptions with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Add a query parameter
    pub fn with_query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// Set the timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Merge with another RequestOptions, with other taking precedence
    pub fn merge(mut self, other: RequestOptions) -> Self {
        self.headers.extend(other.headers);
        self.query.extend(other.query);
        if other.timeout.is_some() {
            self.timeout = other.timeout;
        }
        if other.max_retries.is_some() {
            self.max_retries = other.max_retries;
        }
        self
    }
}

/// Builder pattern implementation for RequestOptions
pub struct RequestOptionsBuilder {
    options: RequestOptions,
}

impl RequestOptionsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            options: RequestOptions::new(),
        }
    }

    /// Add a header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.headers.insert(key.into(), value.into());
        self
    }

    /// Add multiple headers
    pub fn headers<I, K, V>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in headers {
            self.options.headers.insert(key.into(), value.into());
        }
        self
    }

    /// Add a query parameter
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.query.insert(key.into(), value.into());
        self
    }

    /// Add multiple query parameters
    pub fn queries<I, K, V>(mut self, queries: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in queries {
            self.options.query.insert(key.into(), value.into());
        }
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.options.max_retries = Some(max_retries);
        self
    }

    /// Build the RequestOptions
    pub fn build(self) -> RequestOptions {
        self.options
    }
}

impl Default for RequestOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
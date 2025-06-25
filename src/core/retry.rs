//! Retry logic for HTTP requests

use crate::core::error::{Error, Result};
use backoff::{backoff::Backoff, ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial retry delay
    pub initial_backoff: Duration,
    /// Maximum retry delay
    pub max_backoff: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Calculate backoff duration for a given retry attempt
    pub fn calculate_backoff(&self, retry_attempt: u32) -> Duration {
        // Create a temporary backoff to calculate the duration
        let mut backoff = create_backoff(self);
        
        // Advance the backoff to the desired retry attempt
        for _ in 0..retry_attempt {
            if backoff.next_backoff().is_none() {
                return self.max_backoff;
            }
        }
        
        // Get the next backoff duration
        backoff.next_backoff().unwrap_or(self.max_backoff)
    }
}

/// Determine if an error is retryable
pub fn is_retryable_error(error: &Error) -> bool {
    match error {
        // Network errors are usually retryable
        Error::Http(e) => {
            // Connection errors, timeouts, etc.
            e.is_timeout() || e.is_connect() || e.status().map_or(false, |s| {
                // Retry on server errors and rate limits
                s.as_u16() >= 500 || s.as_u16() == 429
            })
        }
        // Rate limit errors are retryable
        Error::RateLimit { .. } => true,
        // Timeout errors are retryable
        Error::Timeout => true,
        // WebSocket errors might be retryable
        Error::WebSocket(e) => {
            use tokio_tungstenite::tungstenite::Error as WsError;
            matches!(
                e,
                WsError::ConnectionClosed | WsError::AlreadyClosed | WsError::Io(_)
            )
        }
        // Other errors are not retryable
        _ => false,
    }
}

/// Extract retry-after duration from error
pub fn get_retry_after(error: &Error) -> Option<Duration> {
    match error {
        Error::RateLimit { retry_after } => retry_after.map(Duration::from_secs),
        // For HTTP errors, the retry-after is already extracted in the Error::RateLimit variant
        _ => None,
    }
}

/// Create exponential backoff from config
pub fn create_backoff(config: &RetryConfig) -> ExponentialBackoff {
    ExponentialBackoffBuilder::new()
        .with_initial_interval(config.initial_backoff)
        .with_max_interval(config.max_backoff)
        .with_multiplier(config.backoff_multiplier)
        .with_randomization_factor(0.5) // Default jitter
        .with_max_elapsed_time(None)
        .build()
}

/// Retry a future with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    config: &RetryConfig,
    mut operation: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut backoff = create_backoff(config);
    let mut retries = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                // Check if we should retry
                if retries >= config.max_retries || !is_retryable_error(&error) {
                    return Err(error);
                }

                // Get retry delay
                let delay = if let Some(retry_after) = get_retry_after(&error) {
                    retry_after
                } else if let Some(delay) = backoff.next_backoff() {
                    delay
                } else {
                    return Err(error);
                };

                // Log retry attempt
                tracing::warn!(
                    "Retrying after error: {} (attempt {}/{}), waiting {:?}",
                    error,
                    retries + 1,
                    config.max_retries,
                    delay
                );

                // Wait before retrying
                tokio::time::sleep(delay).await;
                retries += 1;
            }
        }
    }
}

/// Retry policy builder
#[derive(Debug, Clone)]
pub struct RetryPolicyBuilder {
    config: RetryConfig,
}

impl RetryPolicyBuilder {
    /// Create a new retry policy builder
    pub fn new() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }

    /// Set maximum number of retries
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Set initial retry interval
    pub fn initial_interval(mut self, interval: Duration) -> Self {
        self.config.initial_backoff = interval;
        self
    }

    /// Set maximum retry interval
    pub fn max_interval(mut self, interval: Duration) -> Self {
        self.config.max_backoff = interval;
        self
    }

    /// Set backoff multiplier
    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.config.backoff_multiplier = multiplier;
        self
    }

    /// Build the retry configuration
    pub fn build(self) -> RetryConfig {
        self.config
    }
}

impl Default for RetryPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_retryable_error() {
        // Rate limit errors are retryable
        assert!(is_retryable_error(&Error::RateLimit { retry_after: None }));
        
        // Timeout errors are retryable
        assert!(is_retryable_error(&Error::Timeout));
        
        // Validation errors are not retryable
        assert!(!is_retryable_error(&Error::Validation("test".into())));
        
        // Auth errors are not retryable
        assert!(!is_retryable_error(&Error::Auth("test".into())));
    }

    #[test]
    fn test_get_retry_after() {
        // Rate limit with retry_after
        let error = Error::RateLimit {
            retry_after: Some(5),
        };
        assert_eq!(get_retry_after(&error), Some(Duration::from_secs(5)));
        
        // Rate limit without retry_after
        let error = Error::RateLimit { retry_after: None };
        assert_eq!(get_retry_after(&error), None);
        
        // Other errors
        let error = Error::Timeout;
        assert_eq!(get_retry_after(&error), None);
    }

    #[test]
    fn test_retry_config_builder() {
        let config = RetryPolicyBuilder::new()
            .max_retries(5)
            .initial_interval(Duration::from_secs(1))
            .max_interval(Duration::from_secs(60))
            .multiplier(3.0)
            .build();
        
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_backoff, Duration::from_secs(1));
        assert_eq!(config.max_backoff, Duration::from_secs(60));
        assert_eq!(config.backoff_multiplier, 3.0);
    }
}
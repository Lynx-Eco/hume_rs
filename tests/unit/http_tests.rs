//! Unit tests for HTTP client and retry logic

#[cfg(test)]
mod tests {
    use hume::core::{
        auth::Auth,
        error::Error,
        http::{HttpClient, HttpClientBuilder},
        retry::{retry_with_backoff, RetryConfig},
        request::RequestOptions,
    };
    use std::time::Duration;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_http_client_builder() {
        let client = HttpClientBuilder::new("https://api.hume.ai")
            .auth(Auth::ApiKey("test-key".to_string()))
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .build()
            .unwrap();

        assert!(client.auth.is_some());
    }

    #[test]
    fn test_request_options() {
        let mut options = RequestOptions::default();
        options.timeout = Some(Duration::from_secs(30));
        options.max_retries = Some(2);
        options.headers.insert("X-Custom".to_string(), "value".to_string());
        options.query.insert("param".to_string(), "value".to_string());

        assert_eq!(options.timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.max_retries, Some(2));
        assert_eq!(options.headers.get("X-Custom"), Some(&"value".to_string()));
        assert_eq!(options.query.get("param"), Some(&"value".to_string()));
    }

    #[test]
    fn test_auth_header_values() {
        let auth = Auth::ApiKey("test-key-123".to_string());
        let (header_name, header_value) = auth.header_value().unwrap();
        assert_eq!(header_name, "X-Hume-Api-Key");
        assert_eq!(header_value, "test-key-123");

        let auth = Auth::AccessToken(hume::core::auth::AuthToken {
            access_token: "token-456".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            created_at: chrono::Utc::now(),
        });
        let (header_name, header_value) = auth.header_value().unwrap();
        assert_eq!(header_name, "Authorization");
        assert_eq!(header_value, "Bearer token-456");
    }

    #[test]
    fn test_auth_query_params() {
        let auth = Auth::ApiKey("test-key".to_string());
        let (param_name, param_value) = auth.query_param();
        assert_eq!(param_name, "api_key");
        assert_eq!(param_value, "test-key");

        let auth = Auth::AccessToken(hume::core::auth::AuthToken {
            access_token: "token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            created_at: chrono::Utc::now(),
        });
        let (param_name, param_value) = auth.query_param();
        assert_eq!(param_name, "access_token");
        assert_eq!(param_value, "token");
    }

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let config = RetryConfig {
            max_retries: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(&config, || async {
            attempts_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<_, Error>("success")
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_eventual_success() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let config = RetryConfig {
            max_retries: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(&config, || async {
            let attempt = attempts_clone.fetch_add(1, Ordering::SeqCst);
            if attempt < 2 {
                Err(Error::network("Connection failed"))
            } else {
                Ok("success")
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_attempts_exceeded() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let config = RetryConfig {
            max_retries: 2,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(&config, || async {
            attempts_clone.fetch_add(1, Ordering::SeqCst);
            Err::<String, _>(Error::network("Connection failed"))
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let config = RetryConfig {
            max_retries: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(&config, || async {
            attempts_clone.fetch_add(1, Ordering::SeqCst);
            Err::<String, _>(Error::validation("Invalid input"))
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1); // No retries for validation errors
    }

    #[tokio::test]
    async fn test_retry_rate_limit_error() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let config = RetryConfig {
            max_retries: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(&config, || async {
            let attempt = attempts_clone.fetch_add(1, Ordering::SeqCst);
            if attempt == 0 {
                Err(Error::RateLimit { retry_after: Some(1) })
            } else {
                Ok("success")
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_error_types() {
        let err = Error::network("Connection failed");
        assert!(matches!(err, Error::Network(_)));

        let err = Error::validation("Invalid input");
        assert!(matches!(err, Error::Validation(_)));

        let err = Error::api(404, "Not found".to_string(), None, None);
        assert!(matches!(err, Error::Api { status, .. } if status == 404));

        let err = Error::RateLimit { retry_after: Some(60) };
        assert!(matches!(err, Error::RateLimit { retry_after: Some(60) }));
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff, Duration::from_millis(100));
        assert_eq!(config.max_backoff, Duration::from_secs(10));
        assert_eq!(config.backoff_multiplier, 2.0);
    }

    #[test]
    fn test_calculate_backoff() {
        let config = RetryConfig {
            max_retries: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 2.0,
        };

        // Test exponential backoff calculation
        let backoff = config.calculate_backoff(0);
        assert!(backoff >= Duration::from_millis(100) && backoff <= Duration::from_millis(150));

        let backoff = config.calculate_backoff(1);
        assert!(backoff >= Duration::from_millis(200) && backoff <= Duration::from_millis(300));

        let backoff = config.calculate_backoff(2);
        assert!(backoff >= Duration::from_millis(400) && backoff <= Duration::from_millis(600));

        // Test max backoff
        let backoff = config.calculate_backoff(10);
        assert!(backoff <= config.max_backoff);
    }
}
//! Unit tests for HumeClient

#[cfg(test)]
mod tests {
    use hume::core::{
        client::{HumeClient, HumeClientBuilder},
        auth::Auth,
        error::Error,
    };

    #[test]
    fn test_client_builder_with_api_key() {
        let client = HumeClientBuilder::new()
            .with_api_key("test-key-123")
            .build()
            .unwrap();

        assert!(matches!(client.auth(), Some(Auth::ApiKey(_))));
    }

    #[test]
    fn test_client_builder_with_access_token() {
        let client = HumeClientBuilder::new()
            .with_access_token("token-456")
            .build()
            .unwrap();

        assert!(matches!(client.auth(), Some(Auth::AccessToken(_))));
    }

    #[test]
    fn test_client_builder_with_base_url() {
        let client = HumeClientBuilder::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.api.hume.ai")
            .build()
            .unwrap();

        assert!(matches!(client.auth(), Some(Auth::ApiKey(_))));
    }

    #[test]
    fn test_client_builder_no_auth() {
        let result = HumeClientBuilder::new().build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("Authentication is required")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_client_from_env() {
        // Set environment variable for test
        std::env::set_var("HUME_API_KEY", "env-test-key");
        
        let client = HumeClient::from_env().unwrap();
        assert!(matches!(client.auth(), Some(Auth::ApiKey(_))));
        
        // Clean up
        std::env::remove_var("HUME_API_KEY");
    }

    #[test]
    fn test_client_from_env_missing_key() {
        // Ensure the environment variable is not set
        std::env::remove_var("HUME_API_KEY");
        
        let result = HumeClient::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_client_apis() {
        let client = HumeClientBuilder::new()
            .with_api_key("test-key")
            .build()
            .unwrap();

        // Test that all API clients are accessible
        let _ = client.tts();
        let _ = client.expression();
        let _ = client.evi();
    }

    #[test]
    fn test_api_key_validation_in_builder() {
        // Test empty API key
        let result = HumeClientBuilder::new()
            .with_api_key("")
            .build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }

        // Test "dummy" API key
        let result = HumeClientBuilder::new()
            .with_api_key("dummy")
            .build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("'dummy' is not allowed")),
            _ => panic!("Expected validation error"),
        }

        // Test short API key
        let result = HumeClientBuilder::new()
            .with_api_key("short")
            .build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("too short")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_base_url_default() {
        let client = HumeClientBuilder::new()
            .with_api_key("test-key-123")
            .build()
            .unwrap();

        // Can't directly test base_url but we can verify client was created
        assert!(matches!(client.auth(), Some(Auth::ApiKey(_))));
    }

    #[test]
    fn test_client_clone() {
        let client = HumeClientBuilder::new()
            .with_api_key("test-key")
            .build()
            .unwrap();

        let cloned = client.clone();
        
        // Both should have the same auth
        assert!(matches!(client.auth(), Some(Auth::ApiKey(_))));
        assert!(matches!(cloned.auth(), Some(Auth::ApiKey(_))));
    }
}
//! Integration tests for error handling and retry logic

use hume::{HumeClientBuilder, core::error::Error};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_retry_on_server_error() {
    let mock_server = MockServer::start().await;
    
    // First two attempts fail with 500, third succeeds
    for i in 0..3 {
        Mock::given(method("GET"))
            .and(path("/v0/tts/voices"))
            .and(header("X-Hume-Api-Key", "test-key"))
            .respond_with(
                if i < 2 {
                    ResponseTemplate::new(500).set_body_json(serde_json::json!({
                        "message": "Internal server error",
                        "code": "SERVER_ERROR"
                    }))
                } else {
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "voices": [{
                            "id": "voice-1",
                            "name": "Test Voice"
                        }]
                    }))
                }
            )
            .expect(1)
            .mount(&mock_server)
            .await;
    }
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    let result = tts.list_voices(None).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_no_retry_on_client_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "message": "Bad request",
            "code": "BAD_REQUEST"
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    let result = tts.list_voices(None).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rate_limit_with_retry_after() {
    let mock_server = MockServer::start().await;
    
    // First attempt returns 429, second succeeds
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(429)
            .insert_header("retry-after", "1")
            .set_body_json(serde_json::json!({
                "message": "Rate limit exceeded",
                "code": "RATE_LIMITED"
            })))
        .expect(1)
        .mount(&mock_server)
        .await;
        
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "generations": [{
                "data": "base64audiodata",
                "duration_ms": 1000
            }]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = hume::tts::models::TtsRequestBuilder::new()
        .utterance("Test")
        .unwrap()
        .build();
    
    let start = std::time::Instant::now();
    let result = tts.synthesize(request, None).await;
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    // The retry mechanism should have handled the rate limit
}

#[tokio::test]
async fn test_network_error_retry() {
    // This test simulates network errors by using an invalid URL
    let client = HumeClientBuilder::new("test-key")
        .with_base_url("http://localhost:1") // Invalid port
        .build()
        .unwrap();
    
    let tts = client.tts();
    let result = tts.list_voices(None).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Http(_) => {},
        _ => panic!("Expected network error"),
    }
}

#[tokio::test]
async fn test_authentication_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "invalid-key"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "message": "Invalid API key",
            "code": "AUTH_ERROR"
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("invalid-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    let result = tts.list_voices(None).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Api { status, message, .. } => {
            assert_eq!(status, 401);
            assert!(message.contains("Invalid API key"));
        },
        _ => panic!("Expected API error"),
    }
}

#[tokio::test]
async fn test_validation_error_response() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
            "message": "Invalid request: text is too long",
            "code": "VALIDATION_ERROR",
            "details": {
                "field": "utterances[0].text",
                "constraint": "max_length",
                "value": 5001
            }
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = hume::tts::models::TtsRequestBuilder::new()
        .utterance("Very long text")
        .unwrap()
        .build();
    
    let result = tts.synthesize(request, None).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Api { status, message, .. } => {
            assert_eq!(status, 422);
            assert!(message.contains("text is too long"));
        },
        _ => panic!("Expected API error"),
    }
}

#[tokio::test]
async fn test_timeout_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200)
            .set_delay(std::time::Duration::from_secs(5)) // 5 second delay
            .set_body_json(serde_json::json!({
                "voices": []
            })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    // Use short timeout
    let options = hume::core::request::RequestOptions {
        timeout: Some(std::time::Duration::from_millis(100)),
        ..Default::default()
    };
    
    let result = tts.list_voices(Some(options)).await;
    
    assert!(result.is_err());
    // Timeout errors are wrapped as network errors
    match result.unwrap_err() {
        Error::Http(_) => {},
        _ => panic!("Expected network/timeout error"),
    }
}

#[tokio::test]
async fn test_json_decode_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("not json"))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    let result = tts.list_voices(None).await;
    
    assert!(result.is_err());
    // JSON decode errors are wrapped as HTTP errors
    match result.unwrap_err() {
        Error::Http(_) => {},
        _ => panic!("Expected HTTP error"),
    }
}
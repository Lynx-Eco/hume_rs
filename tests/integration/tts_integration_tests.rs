//! Integration tests for TTS API

use hume::{HumeClientBuilder, tts::models::*};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, body_json};

#[tokio::test]
async fn test_tts_with_voice_settings() {
    let mock_server = MockServer::start().await;
    
    let expected_request = serde_json::json!({
        "utterances": [{
            "text": "Hello with custom voice",
            "voice": {
                "name": "Maya Angelou",
                "provider": "HUME"
            },
            "speed": 1.2,
            "trailing_silence": 500
        }],
        "format": "mp3",
        "sample_rate": 44100
    });
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "generations": [{
                "data": "base64audiodata",
                "duration_ms": 2000,
                "voice": "Maya Angelou"
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = TtsRequestBuilder::new()
        .utterance_with_voice("Hello with custom voice", "Maya Angelou")
        .unwrap()
        .format(AudioFormat::Mp3)
        .sample_rate(SampleRate::HZ_44100)
        .build();
    
    let response = tts.synthesize(request, None).await.unwrap();
    assert_eq!(response.generations.len(), 1);
    assert_eq!(response.generations[0].duration_ms, Some(2000));
}

// TODO: Implement TTS streaming in Rust SDK
// #[tokio::test]
// async fn test_tts_streaming() {
//     // Streaming not yet implemented
// }

#[tokio::test]
async fn test_tts_validation_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
            "message": "Text is too long",
            "code": "VALIDATION_ERROR"
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = TtsRequestBuilder::new()
        .utterance("Very long text")
        .unwrap()
        .build();
    
    let result = tts.synthesize(request, None).await;
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.status_code(), Some(422));
}

#[tokio::test]
async fn test_tts_rate_limit() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "60")
                .set_body_json(serde_json::json!({
                    "message": "Rate limit exceeded",
                    "code": "RATE_LIMITED"
                }))
        )
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = TtsRequestBuilder::new()
        .utterance("Test")
        .unwrap()
        .build();
    
    let result = tts.synthesize(request, None).await;
    assert!(result.is_err());
    
    if let hume::core::error::Error::RateLimit { retry_after } = result.unwrap_err() {
        assert_eq!(retry_after, Some(60));
    } else {
        panic!("Expected rate limit error");
    }
}

#[tokio::test]
async fn test_tts_with_context() {
    let mock_server = MockServer::start().await;
    
    let expected_request = serde_json::json!({
        "utterances": [{
            "text": "Hello world"
        }],
        "context": {
            "text": "This is a friendly greeting",
            "voice": "Maya Angelou"
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "generations": [{
                "data": "base64audiodata",
                "duration_ms": 1500
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let tts = client.tts();
    
    let request = TtsRequestBuilder::new()
        .utterance("Hello world")
        .unwrap()
        .context("This is a friendly greeting", Some("Maya Angelou".to_string()))
        .build();
    
    let response = tts.synthesize(request, None).await.unwrap();
    assert_eq!(response.generations.len(), 1);
}
//! Mock server tests for HTTP endpoints

use hume::HumeClientBuilder;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_tts_synthesize() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v0/tts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "generations": [{
                "data": "base64audiodata",
                "duration_ms": 1500,
                "voice": "test-voice"
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    let tts_client = hume::TtsClient::from(client);
    
    let request = hume::tts::models::TtsRequest {
        utterances: vec![hume::tts::models::Utterance {
            text: "Hello, world!".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };
    
    let response = tts_client.synthesize(request, None).await.unwrap();
    assert_eq!(response.generations.len(), 1);
    assert_eq!(response.generations[0].data, "base64audiodata");
}

#[tokio::test]
async fn test_tts_list_voices() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{
                "id": "voice-1",
                "name": "Test Voice",
                "description": "A test voice",
                "gender": "neutral",
                "language": "en-US"
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    let tts_client = hume::TtsClient::from(client);
    let response = tts_client.list_voices(None).await.unwrap();
    
    assert_eq!(response.voices.len(), 1);
    assert_eq!(response.voices[0].id, "voice-1");
    assert_eq!(response.voices[0].name, "Test Voice");
}

#[tokio::test]
async fn test_expression_measurement_create_job() {
    let mock_server = MockServer::start().await;
    
    // Mock the POST response - returns only job_id
    Mock::given(method("POST"))
        .and(path("/v0/batch/jobs"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "job_id": "job-123"
        })))
        .mount(&mock_server)
        .await;
    
    // Mock the GET response - returns full job details
    Mock::given(method("GET"))
        .and(path("/v0/batch/jobs/job-123"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "job_id": "job-123",
            "type": "INFERENCE",
            "state": {
                "status": "QUEUED",
                "created_timestamp_ms": 1234567890
            },
            "request": {
                "models": {},
                "sources": []
            }
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    let em_client = hume::ExpressionMeasurementClient::from(client);
    let batch = em_client.batch();
    
    let request = hume::expression_measurement::models::BatchJobRequest {
        models: Default::default(),
        sources: vec![],
        callback_url: None,
        notify: None,
    };
    
    let response = batch.create_job(request, None).await.unwrap();
    assert_eq!(response.job_id, "job-123");
    match response.state {
        hume::expression_measurement::models::StateInference::Queued { .. } => {},
        _ => panic!("Expected Queued state"),
    }
}

#[tokio::test]
async fn test_evi_list_tools() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/evi/tools"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page_number": 1,
            "page_size": 10,
            "total_pages": 1,
            "tools_page": [{
                "id": "tool-1",
                "name": "test_tool",
                "description": "A test tool",
                "parameters": {}
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    let evi_client = hume::EviClient::from(client);
    let tools = evi_client.tools();
    
    let response = tools.list(None, None, None).await.unwrap();
    assert_eq!(response.tools_page.len(), 1);
    assert_eq!(response.tools_page[0].as_ref().unwrap().id, "tool-1");
}

#[tokio::test]
async fn test_error_handling() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/tts/voices"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "message": "Unauthorized",
            "code": "AUTH_ERROR"
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    let tts_client = hume::TtsClient::from(client);
    let result = tts_client.list_voices(None).await;
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_api_error());
    assert_eq!(error.status_code(), Some(401));
}
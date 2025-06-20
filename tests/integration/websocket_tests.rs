//! WebSocket tests

use hume::evi::chat::{ClientMessage, ServerMessage};

#[test]
fn test_client_message_serialization() {
    let session_settings = ClientMessage::SessionSettings {
        settings: hume::evi::models::SessionSettings {
            audio: None,
            system_prompt: Some("Test prompt".to_string()),
            context: None,
            variables: None,
            tools: None,
            builtin_tools: None,
        },
    };
    
    let json = serde_json::to_string(&session_settings).unwrap();
    assert!(json.contains(r#""type":"session_settings""#));
    assert!(json.contains(r#""system_prompt":"Test prompt""#));
    
    let audio_input = ClientMessage::AudioInput {
        data: "base64data".to_string(),
    };
    
    let json = serde_json::to_string(&audio_input).unwrap();
    assert!(json.contains(r#""type":"audio_input""#));
    assert!(json.contains(r#""data":"base64data""#));
    
    let user_input = ClientMessage::UserInput {
        text: "Hello".to_string(),
    };
    
    let json = serde_json::to_string(&user_input).unwrap();
    assert!(json.contains(r#""type":"user_input""#));
    assert!(json.contains(r#""text":"Hello""#));
}

#[test]
fn test_server_message_deserialization() {
    let session_started_json = r#"{
        "type": "session_started",
        "session_id": "session-123",
        "chat_id": "chat-456",
        "chat_group_id": "group-789",
        "config": {
            "id": "config-1",
            "name": "test-config",
            "version": 1
        }
    }"#;
    
    let message: ServerMessage = serde_json::from_str(session_started_json).unwrap();
    match message {
        ServerMessage::SessionStarted { session_id, chat_id, .. } => {
            assert_eq!(session_id, "session-123");
            assert_eq!(chat_id, "chat-456");
        }
        _ => panic!("Expected SessionStarted message"),
    }
    
    let user_message_json = r#"{
        "type": "user_message",
        "message_id": "msg-123",
        "text": "Hello, assistant"
    }"#;
    
    let message: ServerMessage = serde_json::from_str(user_message_json).unwrap();
    match message {
        ServerMessage::UserMessage { message_id, text } => {
            assert_eq!(message_id, "msg-123");
            assert_eq!(text, "Hello, assistant");
        }
        _ => panic!("Expected UserMessage"),
    }
    
    let error_json = r#"{
        "type": "error",
        "message": "Something went wrong",
        "code": "ERR_001"
    }"#;
    
    let message: ServerMessage = serde_json::from_str(error_json).unwrap();
    match message {
        ServerMessage::Error { message, code, .. } => {
            assert_eq!(message, "Something went wrong");
            assert_eq!(code, "ERR_001");
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_stream_message_serialization() {
    use hume::expression_measurement::stream::{StreamData, StreamMessage};
    
    let text_data = StreamData::Text {
        text: "Analyze this".to_string(),
    };
    
    let json = serde_json::to_string(&text_data).unwrap();
    assert!(json.contains(r#""type":"text""#));
    assert!(json.contains(r#""text":"Analyze this""#));
    
    let audio_data = StreamData::Audio {
        data: "base64audio".to_string(),
    };
    
    let json = serde_json::to_string(&audio_data).unwrap();
    assert!(json.contains(r#""type":"audio""#));
    
    let job_details_json = r#"{
        "type": "job_details",
        "job_id": "stream-job-123"
    }"#;
    
    let message: StreamMessage = serde_json::from_str(job_details_json).unwrap();
    match message {
        StreamMessage::JobDetails { job_id } => {
            assert_eq!(job_id, "stream-job-123");
        }
        _ => panic!("Expected JobDetails message"),
    }
}
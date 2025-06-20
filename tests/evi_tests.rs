//! Tests for Empathic Voice Interface functionality

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::tools::CreateToolRequest;
use hume::evi::prompts::CreatePromptRequest;
use hume::evi::voices::CreateCustomVoiceRequest;
use hume::evi::configs::CreateConfigRequest;

#[test]
fn test_evi_client_creation() {
    let client = HumeClient::new("test-key").unwrap();
    let evi_client = EviClient::from(client);
    
    // Test that we can create sub-clients
    let _chat = evi_client.chat();
    let _tools = evi_client.tools();
    let _prompts = evi_client.prompts();
    let _voices = evi_client.voices();
    let _configs = evi_client.configs();
}

#[test]
fn test_tool_creation() {
    let tool = Tool {
        id: "tool-123".to_string(),
        name: "weather_tool".to_string(),
        description: "Get weather information".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                }
            }
        }),
        required: Some(false),
        version_id: Some("v1".to_string()),
        created_at: None,
        updated_at: None,
    };
    
    assert_eq!(tool.id, "tool-123");
    assert_eq!(tool.name, "weather_tool");
}

#[test]
fn test_create_tool_request_builder() {
    let request = CreateToolRequest::builder("calculator", "Performs calculations")
        .parameters(serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string"
                }
            }
        }))
        .required(true)
        .build();
    
    assert_eq!(request.name, "calculator");
    assert_eq!(request.description, "Performs calculations");
    assert_eq!(request.required, Some(true));
}

#[test]
fn test_prompt_creation() {
    let prompt = Prompt {
        id: "prompt-123".to_string(),
        name: "greeting".to_string(),
        text: "Hello! How can I help you today?".to_string(),
        version: Some(1),
        version_description: Some("Initial version".to_string()),
        created_at: None,
        updated_at: None,
    };
    
    assert_eq!(prompt.id, "prompt-123");
    assert_eq!(prompt.name, "greeting");
}

#[test]
fn test_create_prompt_request_builder() {
    let request = CreatePromptRequest::builder("support", "How may I assist you?")
        .version_description("Customer support prompt")
        .build();
    
    assert_eq!(request.name, "support");
    assert_eq!(request.text, "How may I assist you?");
    assert_eq!(request.version_description, Some("Customer support prompt".to_string()));
}

#[test]
fn test_custom_voice_creation() {
    let voice = CustomVoice {
        id: "voice-123".to_string(),
        name: "friendly-voice".to_string(),
        base_voice_id: "base-voice-1".to_string(),
        parameters: Some(VoiceParameters {
            pitch: Some(1.1),
            rate: Some(0.95),
            volume: Some(1.0),
        }),
        created_at: None,
        updated_at: None,
    };
    
    assert_eq!(voice.id, "voice-123");
    assert_eq!(voice.name, "friendly-voice");
}

#[test]
fn test_create_custom_voice_request_builder() {
    let request = CreateCustomVoiceRequest::builder("energetic", "base-1")
        .pitch(1.2)
        .rate(1.1)
        .volume(0.9)
        .build();
    
    assert_eq!(request.name, "energetic");
    assert_eq!(request.base_voice_id, "base-1");
    assert!(request.parameters.is_some());
    
    let params = request.parameters.unwrap();
    assert_eq!(params.pitch, Some(1.2));
    assert_eq!(params.rate, Some(1.1));
    assert_eq!(params.volume, Some(0.9));
}

#[test]
fn test_config_creation() {
    let config = Config {
        id: "config-123".to_string(),
        name: "customer-support".to_string(),
        version: 1,
        prompt: Some(PromptSpec {
            id: "prompt-1".to_string(),
            version: Some(2),
        }),
        voice: Some(VoiceSpec {
            id: "voice-1".to_string(),
        }),
        language_model: Some(LanguageModelSpec {
            model_provider: "openai".to_string(),
            model_resource: "gpt-4".to_string(),
            temperature: Some(0.7),
        }),
        tools: None,
        event_messages: None,
        timeouts: None,
        created_at: None,
        updated_at: None,
    };
    
    assert_eq!(config.id, "config-123");
    assert_eq!(config.name, "customer-support");
}

#[test]
fn test_create_config_request_builder() {
    let request = CreateConfigRequest::builder("assistant-config")
        .prompt("prompt-1", Some(3))
        .voice("voice-2")
        .language_model("anthropic", "claude-3", Some(0.8))
        .add_tool("tool-1", None)
        .add_tool("tool-2", Some(2))
        .timeouts(Some(300), Some(3600))
        .build();
    
    assert_eq!(request.name, "assistant-config");
    assert!(request.prompt.is_some());
    assert!(request.voice.is_some());
    assert!(request.language_model.is_some());
    assert!(request.tools.is_some());
    assert_eq!(request.tools.as_ref().unwrap().len(), 2);
}

#[test]
fn test_session_settings() {
    let settings = SessionSettings {
        audio: Some(AudioConfig {
            input_encoding: Some(AudioEncoding::Linear16),
            input_sample_rate: Some(16000),
            output_encoding: Some(AudioEncoding::Linear16),
            output_sample_rate: Some(24000),
            output_format: Some(AudioFormat::Wav),
        }),
        system_prompt: Some("You are a helpful assistant".to_string()),
        context: Some(Context {
            context_type: ContextType::Persistent,
            text: "User is a premium customer".to_string(),
        }),
        variables: None,
        tools: Some(vec!["tool-1".to_string(), "tool-2".to_string()]),
        builtin_tools: None,
    };
    
    assert!(settings.audio.is_some());
    assert!(settings.system_prompt.is_some());
    assert!(settings.context.is_some());
}

#[test]
fn test_chat_status_serialization() {
    assert_eq!(serde_json::to_string(&ChatStatus::Active).unwrap(), r#""active""#);
    assert_eq!(serde_json::to_string(&ChatStatus::Ended).unwrap(), r#""ended""#);
    assert_eq!(serde_json::to_string(&ChatStatus::Interrupted).unwrap(), r#""interrupted""#);
    assert_eq!(serde_json::to_string(&ChatStatus::Error).unwrap(), r#""error""#);
}

#[test]
fn test_message_role_serialization() {
    assert_eq!(serde_json::to_string(&MessageRole::User).unwrap(), r#""user""#);
    assert_eq!(serde_json::to_string(&MessageRole::Assistant).unwrap(), r#""assistant""#);
    assert_eq!(serde_json::to_string(&MessageRole::System).unwrap(), r#""system""#);
    assert_eq!(serde_json::to_string(&MessageRole::Tool).unwrap(), r#""tool""#);
}

#[test]
fn test_audio_encoding_serialization() {
    assert_eq!(serde_json::to_string(&AudioEncoding::Linear16).unwrap(), r#""linear16""#);
    assert_eq!(serde_json::to_string(&AudioEncoding::Mulaw).unwrap(), r#""mulaw""#);
}

#[test]
fn test_context_type_serialization() {
    assert_eq!(serde_json::to_string(&ContextType::Persistent).unwrap(), r#""persistent""#);
    assert_eq!(serde_json::to_string(&ContextType::Temporary).unwrap(), r#""temporary""#);
}
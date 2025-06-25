//! Unit tests for EVI models and functionality

#[cfg(test)]
mod tests {
    use hume::evi::models::*;
    use hume::evi::chat::*;
    use hume::evi::configs::*;
    
    #[test]
    fn test_audio_encoding_serialization() {
        use serde_json;
        
        let encoding = AudioEncoding::Linear16;
        let json = serde_json::to_string(&encoding).unwrap();
        assert_eq!(json, r#""linear16""#);
        
        let encoding = AudioEncoding::Mulaw;
        let json = serde_json::to_string(&encoding).unwrap();
        assert_eq!(json, r#""mulaw""#);
    }
    
    #[test]
    fn test_audio_format_serialization() {
        use serde_json;
        
        let format = AudioFormat::Raw;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""raw""#);
        
        let format = AudioFormat::Wav;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""wav""#);
        
        let format = AudioFormat::Mp3;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""mp3""#);
    }
    
    #[test]
    fn test_session_settings_creation() {
        let settings = SessionSettings {
            system_prompt: Some("You are helpful".to_string()),
            audio: Some(AudioConfig {
                input_encoding: Some(AudioEncoding::Linear16),
                input_sample_rate: Some(16000),
                output_encoding: Some(AudioEncoding::Linear16),
                output_sample_rate: Some(24000),
                output_format: Some(AudioFormat::Wav),
            }),
            context: None,
            variables: None,
            tools: None,
            builtin_tools: None,
        };
        
        assert_eq!(settings.system_prompt, Some("You are helpful".to_string()));
        assert!(settings.audio.is_some());
        
        let audio = settings.audio.unwrap();
        assert_eq!(audio.input_encoding, Some(AudioEncoding::Linear16));
        assert_eq!(audio.input_sample_rate, Some(16000));
        assert_eq!(audio.output_sample_rate, Some(24000));
    }
    
    #[test]
    fn test_context_type_serialization() {
        use serde_json;
        
        let context = Context {
            context_type: ContextType::Persistent,
            text: "Remember this".to_string(),
        };
        
        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains(r#""type":"persistent""#));
        assert!(json.contains(r#""text":"Remember this""#));
    }
    
    #[test]
    fn test_chat_session_builder() {
        let builder = ChatSessionBuilder::new()
            .config_id("test-config")
            .config_version(1)
            .resumed_chat_group_id("group-123");
        
        // Can't test the full connection without mocking
        // but we can verify the builder stores values correctly
        assert!(builder.config_id.is_some());
        assert_eq!(builder.config_id.unwrap(), "test-config");
        assert_eq!(builder.config_version, Some(1));
        assert_eq!(builder.resumed_chat_group_id, Some("group-123".to_string()));
    }
    
    #[test]
    fn test_server_message_variants() {
        use serde_json;
        
        // Test SessionStarted
        let msg = ServerMessage::SessionStarted {
            session_id: "sess-123".to_string(),
            chat_id: "chat-456".to_string(),
            chat_group_id: Some("group-789".to_string()),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"session_started""#));
        assert!(json.contains(r#""session_id":"sess-123""#));
        
        // Test AssistantMessage
        let msg = ServerMessage::AssistantMessage {
            message_id: Some("msg-123".to_string()),
            text: "Hello there!".to_string(),
            is_final: true,
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"assistant_message""#));
        assert!(json.contains(r#""text":"Hello there!""#));
        assert!(json.contains(r#""is_final":true"#));
    }
    
    #[test]
    fn test_client_message_variants() {
        use serde_json;
        
        // Test UserInput
        let msg = ClientMessage::UserInput {
            text: "Hello AI".to_string(),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"user_input""#));
        assert!(json.contains(r#""text":"Hello AI""#));
        
        // Test SessionSettings
        let settings = SessionSettings {
            system_prompt: Some("Be concise".to_string()),
            ..Default::default()
        };
        
        let msg = ClientMessage::SessionSettings { settings };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"session_settings""#));
        assert!(json.contains(r#""system_prompt":"Be concise""#));
    }
    
    #[test]
    fn test_create_config_request_builder() {
        let request = CreateConfigRequestBuilder::new("My Assistant")
            .voice("ito")
            .prompt("prompt-123", Some(2))
            .build();
        
        assert_eq!(request.name, "My Assistant");
        assert!(request.voice.is_some());
        assert!(request.prompt.is_some());
        
        let voice = request.voice.unwrap();
        assert_eq!(voice.id, "ito");
        
        let prompt = request.prompt.unwrap();
        assert_eq!(prompt.id, "prompt-123");
        assert_eq!(prompt.version, Some(2));
    }
    
    #[test]
    fn test_builtin_tool() {
        let tool = BuiltinTool {
            name: "web_search".to_string(),
            config: Some(serde_json::json!({
                "max_results": 5
            })),
        };
        
        assert_eq!(tool.name, "web_search");
        assert!(tool.config.is_some());
    }
    
    #[test]
    fn test_return_paged_configs() {
        let configs = vec![
            Config {
                id: "config-1".to_string(),
                name: "Assistant 1".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                voice: None,
                prompt: None,
                language_model: None,
                tools: None,
                version: 1,
            },
            Config {
                id: "config-2".to_string(),
                name: "Assistant 2".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                voice: None,
                prompt: None,
                language_model: None,
                tools: None,
                version: 1,
            },
        ];
        
        let paged = ReturnPagedConfigs {
            page_number: Some(0),
            page_size: Some(10),
            total_pages: Some(1),
            total_items: Some(2),
            configs_page: Some(configs),
        };
        
        assert_eq!(paged.page_number, Some(0));
        assert!(paged.configs_page.is_some());
        let configs = paged.configs_page.unwrap();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].name, "Assistant 1");
    }
}
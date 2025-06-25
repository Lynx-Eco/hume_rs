//! Unit tests for TTS models and functionality

#[cfg(test)]
mod tests {
    use hume::tts::models::*;
    use hume::core::error::Error;

    #[test]
    fn test_tts_request_builder_valid() {
        let request = TtsRequestBuilder::new()
            .utterance("Hello world")
            .unwrap()
            .build();
        
        assert_eq!(request.utterances.len(), 1);
        assert_eq!(request.utterances[0].text, "Hello world");
    }

    #[test]
    fn test_tts_request_builder_text_too_long() {
        let long_text = "a".repeat(5001);
        let result = TtsRequestBuilder::new().utterance(long_text);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => {
                assert!(msg.contains("TTS text"));
                assert!(msg.contains("<= 5000"));
                assert!(msg.contains("5001"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_tts_request_builder_empty_text() {
        let result = TtsRequestBuilder::new().utterance("");
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_tts_request_builder_with_voice() {
        let request = TtsRequestBuilder::new()
            .utterance_with_voice("Hello", "Maya Angelou")
            .unwrap()
            .build();
        
        assert_eq!(request.utterances.len(), 1);
        assert_eq!(request.utterances[0].text, "Hello");
        
        match &request.utterances[0].voice {
            Some(VoiceSpec::Name { name, provider }) => {
                assert_eq!(name, "Maya Angelou");
                assert!(provider.is_none());
            }
            _ => panic!("Expected voice name"),
        }
    }

    #[test]
    fn test_tts_request_builder_multiple_utterances() {
        let request = TtsRequestBuilder::new()
            .utterance("First sentence.")
            .unwrap()
            .utterance("Second sentence.")
            .unwrap()
            .utterance_with_voice("Third sentence.", "Colton Rivers")
            .unwrap()
            .build();
        
        assert_eq!(request.utterances.len(), 3);
        assert_eq!(request.utterances[0].text, "First sentence.");
        assert_eq!(request.utterances[1].text, "Second sentence.");
        assert_eq!(request.utterances[2].text, "Third sentence.");
    }

    #[test]
    fn test_utterance_speed_validation() {
        let mut utterance = Utterance {
            text: "Test".to_string(),
            voice: None,
            description: None,
            speed: Some(1.5),
            trailing_silence: None,
        };
        
        // Valid speed
        assert_eq!(utterance.speed, Some(1.5));
        
        // Test with builder
        let request = TtsRequestBuilder::new()
            .add_utterance(utterance.clone())
            .unwrap()
            .build();
        
        assert_eq!(request.utterances[0].speed, Some(1.5));
        
        // Test speed clamping
        utterance.speed = Some(0.3);
        let request = TtsRequestBuilder::new()
            .add_utterance(utterance.clone())
            .unwrap()
            .build();
        
        assert_eq!(request.utterances[0].speed, Some(0.5)); // Should be clamped
        
        utterance.speed = Some(3.0);
        let request = TtsRequestBuilder::new()
            .add_utterance(utterance)
            .unwrap()
            .build();
        
        assert_eq!(request.utterances[0].speed, Some(2.0)); // Should be clamped
    }

    #[test]
    fn test_audio_format_serialization() {
        use serde_json;
        
        // Test MP3 format
        let format = AudioFormat::Mp3;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"type":"mp3"}"#);
        
        // Test WAV format
        let format = AudioFormat::Wav;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"type":"wav"}"#);
        
        // Test PCM format
        let format = AudioFormat::Pcm;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"type":"pcm"}"#);
    }

    #[test]
    fn test_voice_spec_serialization() {
        use serde_json;
        
        // Test voice by name
        let voice = VoiceSpec::Name {
            name: "Maya Angelou".to_string(),
            provider: None,
        };
        let json = serde_json::to_string(&voice).unwrap();
        assert_eq!(json, r#"{"name":"Maya Angelou"}"#);
        
        // Test voice by ID
        let voice = VoiceSpec::Id {
            id: "voice_123".to_string(),
            provider: Some(VoiceProvider::HumeAi),
        };
        let json = serde_json::to_string(&voice).unwrap();
        assert!(json.contains(r#""id":"voice_123""#));
        assert!(json.contains(r#""provider":"HUME_AI""#));
    }

    #[test]
    fn test_sample_rate() {
        assert_eq!(SampleRate::HZ_8000.as_u32(), 8000);
        assert_eq!(SampleRate::HZ_16000.as_u32(), 16000);
        assert_eq!(SampleRate::HZ_22050.as_u32(), 22050);
        assert_eq!(SampleRate::HZ_24000.as_u32(), 24000);
        assert_eq!(SampleRate::HZ_44100.as_u32(), 44100);
        assert_eq!(SampleRate::HZ_48000.as_u32(), 48000);
        
        // Test custom sample rate
        let custom = SampleRate::custom(12000);
        assert_eq!(custom.as_u32(), 12000);
        
        // Test conversion to u32
        let rate: u32 = SampleRate::HZ_44100.into();
        assert_eq!(rate, 44100);
    }

    #[test]
    fn test_tts_request_context() {
        let request = TtsRequestBuilder::new()
            .utterance("Hello")
            .unwrap()
            .context("This is a friendly greeting", Some("Maya Angelou".to_string()))
            .build();
        
        assert!(request.context.is_some());
        let context = request.context.unwrap();
        assert_eq!(context.text, "This is a friendly greeting");
        assert_eq!(context.voice, Some("Maya Angelou".to_string()));
    }

    #[test]
    fn test_tts_request_format() {
        let request = TtsRequestBuilder::new()
            .utterance("Test")
            .unwrap()
            .format(AudioFormat::Wav)
            .sample_rate(SampleRate::HZ_44100)
            .build();
        
        assert_eq!(request.format, Some(AudioFormat::Wav));
        assert_eq!(request.sample_rate, Some(SampleRate::HZ_44100));
    }
}
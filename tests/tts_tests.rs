//! Tests for Text-to-Speech functionality

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;

#[test]
fn test_tts_client_creation() {
    let client = HumeClient::new("test-key").unwrap();
    let _tts_client = TtsClient::from(client);
    
    // Test that we can create a client successfully
    // (actual functionality would be tested with integration tests)
}

#[test]
fn test_tts_request_builder() {
    let request = TtsRequestBuilder::new()
        .utterance("Hello, world!")
        .unwrap()
        .utterance_with_voice("How are you?", "en-US-1")
        .unwrap()
        .format(AudioFormat::Mp3)
        .sample_rate(SampleRate::HZ_44100)
        .build();
    
    
    assert_eq!(request.utterances.len(), 2);
    assert_eq!(request.utterances[0].text, "Hello, world!");
    assert_eq!(request.utterances[1].text, "How are you?");
    assert!(request.utterances[1].voice.is_some());
    assert_eq!(request.format, Some(AudioFormat::Mp3));
    assert_eq!(request.sample_rate, Some(SampleRate::HZ_44100));
}

#[test]
fn test_utterance_creation() {
    let utterance = Utterance {
        text: "Test utterance".to_string(),
        voice: Some(VoiceSpec::Name {
            name: "test-voice".to_string(),
            provider: None,
        }),
        description: Some("Happy tone".to_string()),
        speed: Some(1.2),
        trailing_silence: Some(500),
    };
    
    assert_eq!(utterance.text, "Test utterance");
    assert!(utterance.voice.is_some());
    assert_eq!(utterance.description, Some("Happy tone".to_string()));
    assert_eq!(utterance.speed, Some(1.2));
    assert_eq!(utterance.trailing_silence, Some(500));
}

#[test]
fn test_audio_format_serialization() {
    let mp3 = serde_json::to_string(&AudioFormat::Mp3).unwrap();
    assert_eq!(mp3, r#"{"type":"mp3"}"#);
    
    let wav = serde_json::to_string(&AudioFormat::Wav).unwrap();
    assert_eq!(wav, r#"{"type":"wav"}"#);
    
    let pcm = serde_json::to_string(&AudioFormat::Pcm).unwrap();
    assert_eq!(pcm, r#"{"type":"pcm"}"#);
}

#[test]
fn test_tts_stream_request() {
    let request = TtsStreamRequest {
        text: "Stream this text".to_string(),
        voice: Some(VoiceSpec::Name {
            name: "stream-voice".to_string(),
            provider: None,
        }),
        description: Some("Excited".to_string()),
        speed: Some(0.9),
        format: Some(AudioFormat::Wav),
        sample_rate: Some(SampleRate::HZ_22050),
        instant: Some(true),
    };
    
    assert_eq!(request.text, "Stream this text");
    assert!(request.voice.is_some());
    assert_eq!(request.instant, Some(true));
}

#[test]
fn test_context_creation() {
    let context = Context {
        text: "Previous conversation context".to_string(),
        voice: Some("context-voice".to_string()),
    };
    
    assert_eq!(context.text, "Previous conversation context");
    assert_eq!(context.voice, Some("context-voice".to_string()));
}

#[test]
fn test_sample_rate() {
    // Test predefined sample rates
    assert_eq!(SampleRate::HZ_8000.as_u32(), 8000);
    assert_eq!(SampleRate::HZ_16000.as_u32(), 16000);
    assert_eq!(SampleRate::HZ_22050.as_u32(), 22050);
    assert_eq!(SampleRate::HZ_24000.as_u32(), 24000);
    assert_eq!(SampleRate::HZ_44100.as_u32(), 44100);
    assert_eq!(SampleRate::HZ_48000.as_u32(), 48000);
    
    // Test custom sample rate
    let custom = SampleRate::custom(32000);
    assert_eq!(custom.as_u32(), 32000);
    
    // Test conversion to u32
    let rate: u32 = SampleRate::HZ_44100.into();
    assert_eq!(rate, 44100);
    
    // Test serialization
    let json = serde_json::to_string(&SampleRate::HZ_44100).unwrap();
    assert_eq!(json, "44100");
}
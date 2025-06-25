//! Comprehensive TTS Example with real audio playback
//! 
//! This example demonstrates all TTS features:
//! - Different voices and providers
//! - Various audio formats and sample rates
//! - Emotional context descriptions
//! - Speed adjustments
//! - Streaming vs batch synthesis
//! - Error handling
//! - Real-time audio playback

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;
use futures_util::StreamExt;
use std::fs;
use std::io::{Write, Cursor};
use rodio::{OutputStream, Sink, Decoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗣️  Comprehensive TTS Example with Audio Playback");
    println!("===============================================\n");
    
    dotenvy::dotenv().ok();
    let has_api_key = std::env::var("HUME_API_KEY")
        .map(|k| !k.is_empty() && k != "dummy")
        .unwrap_or(false);
    
    let api_key = if has_api_key {
        println!("✓ API key found - full example will run\n");
        std::env::var("HUME_API_KEY")?
    } else {
        println!("📋 Running in DEMO MODE (no API key)");
        println!("   Some examples will work locally, others need an API key.\n");
        "dummy".to_string()
    };
    
    let client = HumeClient::new(api_key)?;
    let tts = TtsClient::from(client);
    
    // Initialize audio output
    let (_stream, stream_handle) = OutputStream::try_default()?;
    println!("🔊 Audio output initialized\n");
    
    // Example 1: Basic synthesis with different voices
    example_voices(&tts, &stream_handle).await?;
    
    // Example 2: Different audio formats and sample rates
    example_formats(&tts, &stream_handle).await?;
    
    // Example 3: Emotional context and prosody
    example_emotions(&tts, &stream_handle).await?;
    
    // Example 4: Speed variations
    example_speed(&tts, &stream_handle).await?;
    
    // Example 5: Streaming synthesis
    example_streaming(&tts, &stream_handle).await?;
    
    // Example 6: Batch synthesis with context
    example_batch_with_context(&tts, &stream_handle).await?;
    
    // Example 7: Error handling
    example_error_handling(&tts).await?;
    
    println!("\n✓ All examples completed!");
    Ok(())
}

async fn example_voices(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 1: Voice Selection");
    println!("-----------------------------\n");
    
    // List available voices (requires API key)
    println!("Attempting to list available voices...");
    match tts.list_voices(None).await {
        Ok(voices) => {
            println!("✓ Available voices:");
            for voice in voices.voices.iter().take(5) {
                println!("  - {} ({})", voice.name, voice.id);
                if let Some(desc) = &voice.description {
                    println!("    {}", desc);
                }
                if let Some(tags) = &voice.tags {
                    println!("    Tags: {}", tags.join(", "));
                }
            }
        }
        Err(e) => {
            println!("  Could not list voices: {}", e);
            println!("  (This requires a valid API key)");
        }
    }
    
    // Demo voice specifications
    println!("\nVoice specification examples:");
    
    let voice_examples = vec![
        ("By ID", VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        ("By Name", VoiceSpec::Name {
            name: "ITO".to_string(),
            provider: Some(VoiceProvider::HumeAi),
        }),
        ("Custom Voice", VoiceSpec::Id {
            id: "custom-voice-id".to_string(),
            provider: Some(VoiceProvider::CustomVoice),
        }),
    ];
    
    for (name, voice) in voice_examples {
        println!("  - {}: {:?}", name, voice);
    }
    
    // Try to synthesize with a voice
    println!("\nTrying voice synthesis...");
    match tts.synthesize_simple("Hello from different voices!", None::<String>).await {
        Ok(audio_data) => {
            println!("  ✓ Synthesis successful, playing audio...");
            play_mp3(&audio_data, stream_handle)?;
        }
        Err(e) => {
            println!("  Could not synthesize: {}", e);
        }
    }
    
    Ok(())
}

async fn example_formats(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n📌 Example 2: Audio Formats & Sample Rates");
    println!("------------------------------------------\n");
    
    let formats_and_rates = vec![
        (AudioFormat::Mp3, None, "MP3 (default rate)"),
        (AudioFormat::Wav, Some(SampleRate::HZ_16000), "WAV 16kHz"),
        (AudioFormat::Wav, Some(SampleRate::HZ_44100), "WAV 44.1kHz"),
        (AudioFormat::Pcm, Some(SampleRate::HZ_24000), "PCM 24kHz"),
        (AudioFormat::Pcm, Some(SampleRate::custom(32000)), "PCM 32kHz (custom)"),
    ];
    
    for (format, sample_rate, description) in formats_and_rates {
        println!("Testing {}", description);
        
        let mut builder = TtsRequestBuilder::new()
            .utterance("Testing audio format.")
            .unwrap()
            .format(format);
        
        if let Some(rate) = sample_rate {
            builder = builder.sample_rate(rate);
        }
        
        let request = builder.build();
        
        // This would synthesize with the API
        println!("  Format: {:?}", request.format);
        println!("  Sample rate: {:?}", request.sample_rate);
        
        // Try actual synthesis for supported formats
        if matches!(format, AudioFormat::Mp3 | AudioFormat::Wav) {
            match tts.synthesize(request, None).await {
                Ok(response) => {
                    if let Some(generation) = response.generations.first() {
                        use base64::Engine;
                        let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
                        println!("  ✓ Generated {} bytes", audio_data.len());
                        
                        // Play the audio
                        match format {
                            AudioFormat::Mp3 => play_mp3(&audio_data, stream_handle)?,
                            AudioFormat::Wav => play_wav(&audio_data, stream_handle)?,
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    println!("  Synthesis error: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

async fn example_emotions(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n📌 Example 3: Emotional Context");
    println!("--------------------------------\n");
    
    let emotional_examples = vec![
        ("Happy", "I just got promoted! This is the best day ever!", "Speaking with joy and excitement"),
        ("Sad", "I'm sorry to hear about your loss.", "Speaking with empathy and sadness"),
        ("Angry", "This is completely unacceptable!", "Speaking with frustration and anger"),
        ("Calm", "Take a deep breath. Everything will be okay.", "Speaking in a soothing, calm manner"),
        ("Excited", "We're going to Disneyland!", "Speaking with childlike excitement"),
        ("Professional", "Thank you for calling customer service.", "Speaking in a professional, neutral tone"),
    ];
    
    for (emotion, text, description) in emotional_examples {
        println!("{} example:", emotion);
        println!("  Text: \"{}\"", text);
        println!("  Description: \"{}\"", description);
        
        let utterance = Utterance {
            text: text.to_string(),
            voice: None,
            description: Some(description.to_string()),
            speed: None,
            trailing_silence: Some(500), // 500ms pause after
        };
        
        let request = TtsRequest {
            utterances: vec![utterance],
            context: None,
            format: Some(AudioFormat::Mp3),
            sample_rate: None,
        };
        
        // Try to synthesize and play
        match tts.synthesize(request, None).await {
            Ok(response) => {
                if let Some(generation) = response.generations.first() {
                    use base64::Engine;
                    let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
                    println!("  ✓ Playing {} audio...", emotion.to_lowercase());
                    play_mp3(&audio_data, stream_handle)?;
                }
            }
            Err(_) => {
                println!("  (Would generate {} audio)", emotion.to_lowercase());
            }
        }
        
        println!();
    }
    
    Ok(())
}

async fn example_speed(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📌 Example 4: Speed Variations");
    println!("------------------------------\n");
    
    let speed_examples = vec![
        (0.5, "Very slow speech"),
        (0.75, "Slow speech"),
        (1.0, "Normal speed"),
        (1.25, "Slightly fast"),
        (1.5, "Fast speech"),
        (2.0, "Very fast speech"),
    ];
    
    let text = "The quick brown fox jumps over the lazy dog.";
    
    for (speed, description) in speed_examples {
        println!("{} ({}x):", description, speed);
        
        let request = TtsRequest {
            utterances: vec![Utterance {
                text: text.to_string(),
                voice: None,
                description: None,
                speed: Some(speed),
                trailing_silence: None,
            }],
            context: None,
            format: Some(AudioFormat::Mp3),
            sample_rate: None,
        };
        
        println!("  Text: \"{}\"", text);
        
        // Try to synthesize and play
        match tts.synthesize(request, None).await {
            Ok(response) => {
                if let Some(generation) = response.generations.first() {
                    use base64::Engine;
                    let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
                    println!("  ✓ Playing at {}x speed...", speed);
                    play_mp3(&audio_data, stream_handle)?;
                }
            }
            Err(_) => {
                println!("  (Would generate at {}x speed)", speed);
            }
        }
        
        println!();
    }
    
    Ok(())
}

async fn example_streaming(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📌 Example 5: Streaming Synthesis");
    println!("---------------------------------\n");
    
    println!("Streaming allows real-time synthesis with lower latency.\n");
    
    let stream_request = TtsStreamRequest {
        text: "This is a streaming synthesis example. \
                Audio is generated and sent in chunks as it's being produced. \
                This enables lower latency for real-time applications.".to_string(),
        voice: Some(VoiceSpec::Name {
            name: "ITO".to_string(),
            provider: None,
        }),
        description: Some("Speaking clearly and enthusiastically".to_string()),
        speed: Some(1.0),
        format: Some(AudioFormat::Mp3),
        sample_rate: None,
        instant: Some(true), // Enable instant mode for lowest latency
    };
    
    // Demo streaming with real-time playback
    println!("Streaming synthesis with real-time playback...");
    match tts.stream_file(stream_request.clone(), None).await {
        Ok(mut stream) => {
            let mut chunk_count = 0;
            let mut total_bytes = 0;
            let mut audio_buffer = Vec::new();
            
            print!("Receiving chunks: ");
            while let Some(result) = stream.next().await {
                match result {
                    Ok(data) => {
                        chunk_count += 1;
                        total_bytes += data.len();
                        audio_buffer.extend_from_slice(&data);
                        print!("█");
                        std::io::stdout().flush()?;
                        
                        // Save first chunk as example
                        if chunk_count == 1 {
                            fs::write("stream_chunk_1.mp3", &data)?;
                        }
                    }
                    Err(e) => {
                        println!("\nStream error: {}", e);
                        break;
                    }
                }
            }
            
            println!("\n✓ Received {} chunks ({} bytes total)", chunk_count, total_bytes);
            
            if !audio_buffer.is_empty() {
                println!("🔊 Playing streamed audio...");
                play_mp3(&audio_buffer, stream_handle)?;
                println!("  First chunk saved as stream_chunk_1.mp3");
            }
        }
        Err(e) => {
            println!("Streaming error: {}", e);
        }
    }
    
    Ok(())
}

async fn example_batch_with_context(tts: &TtsClient, stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n📌 Example 6: Batch with Context");
    println!("--------------------------------\n");
    
    println!("Context helps maintain consistency across utterances.\n");
    
    let context = Context {
        text: "Once upon a time, in a magical forest, there lived a wise old owl.".to_string(),
        voice: Some("narrator-voice".to_string()),
    };
    
    let story_parts = vec![
        ("The owl was known throughout the forest for her wisdom.", Some("Speaking as a gentle narrator")),
        ("'Who goes there?' hooted the owl.", Some("Speaking as a wise, elderly owl")),
        ("'It's me, little rabbit,' came the reply.", Some("Speaking as a timid rabbit")),
        ("The owl smiled warmly.", Some("Back to gentle narrator voice")),
    ];
    
    let mut utterances = Vec::new();
    for (text, description) in story_parts {
        utterances.push(Utterance {
            text: text.to_string(),
            voice: None,
            description: description.map(String::from),
            speed: None,
            trailing_silence: Some(300), // Small pause between lines
        });
    }
    
    let request = TtsRequest {
        utterances,
        context: Some(context),
        format: Some(AudioFormat::Wav),
        sample_rate: Some(SampleRate::HZ_44100),
    };
    
    println!("Story synthesis request created:");
    println!("  Context: \"{}...\"", &request.context.as_ref().unwrap().text[..50]);
    println!("  Utterances: {}", request.utterances.len());
    println!("  Format: WAV @ 44.1kHz");
    
    // This would synthesize the story with consistent voice
    match tts.synthesize(request, None).await {
        Ok(response) => {
            println!("\n✓ Synthesis successful!");
            println!("  Generated {} audio segments", response.generations.len());
            
            // Play each part of the story
            for (i, generation) in response.generations.iter().enumerate() {
                use base64::Engine;
                let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
                println!("  🔊 Playing part {}...", i + 1);
                play_wav(&audio_data, stream_handle)?;
            }
        }
        Err(e) => {
            println!("\n  Could not synthesize: {}", e);
            println!("  (This requires a valid API key)");
        }
    }
    
    Ok(())
}

async fn example_error_handling(tts: &TtsClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n📌 Example 7: Error Handling");
    println!("----------------------------\n");
    
    // Example: Empty text
    println!("Testing empty text error:");
    let empty_request = TtsRequest {
        utterances: vec![Utterance {
            text: "".to_string(),
            voice: None,
            description: None,
            speed: None,
            trailing_silence: None,
        }],
        context: None,
        format: None,
        sample_rate: None,
    };
    
    match tts.synthesize(empty_request, None).await {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  ✓ Expected error: {}", e),
    }
    
    // Example: Invalid speed
    println!("\nTesting invalid speed:");
    let _invalid_speed = Utterance {
        text: "Test".to_string(),
        voice: None,
        description: None,
        speed: Some(3.0), // Too fast (max is 2.0)
        trailing_silence: None,
    };
    
    println!("  Speed: 3.0 (exceeds maximum of 2.0)");
    println!("  (Would be rejected by API)");
    
    // Example: Unsupported format combination
    println!("\nTesting format constraints:");
    println!("  PCM format requires sample_rate to be specified");
    println!("  MP3 format ignores sample_rate parameter");
    
    Ok(())
}

// Helper functions for audio playback
fn play_mp3(audio_data: &[u8], stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(audio_data.to_vec());
    let source = Decoder::new(cursor)?;
    
    let sink = Sink::try_new(stream_handle)?;
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}

fn play_wav(wav_data: &[u8], stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(wav_data.to_vec());
    let source = Decoder::new_wav(cursor)?;
    
    let sink = Sink::try_new(stream_handle)?;
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}
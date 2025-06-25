//! Basic Text-to-Speech example with audio playback

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;
use std::fs;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Initialize the client with your API key
    let api_key = std::env::var("HUME_API_KEY")
        .expect("HUME_API_KEY environment variable must be set");
    
    let client = HumeClient::new(api_key)?;
    let tts = TtsClient::from(client);
    
    // Initialize audio output
    let (_stream, stream_handle) = OutputStream::try_default()?;
    
    // Example 1: Simple synthesis with playback
    println!("Example 1: Simple synthesis with audio playback");
    match tts.synthesize_simple(
        "Hello! Welcome to the rust sdk for hume ai.",
        None::<String>,
    ).await {
        Ok(audio_bytes) => {
            // Check if we got valid audio data
            if audio_bytes.len() > 100 { // Minimal check for valid MP3
                fs::write("output_simple.mp3", &audio_bytes)?;
                println!("✓ Saved audio to output_simple.mp3");
                
                // Play the audio
                println!("🔊 Playing audio...");
                match play_audio(&audio_bytes, &stream_handle) {
                    Ok(_) => println!("✓ Playback complete"),
                    Err(e) => println!("  Could not play audio: {}", e)
                }
            } else {
                println!("  Received invalid audio data (too small)");
            }
        }
        Err(e) => {
            eprintln!("Error synthesizing audio: {}", e);
            eprintln!("Note: You need a valid HUME_API_KEY to synthesize audio");
        }
    }
    
    // Example 2: Synthesis with voice and options
    println!("\nExample 2: Synthesis with voice and options");
    let request = TtsRequestBuilder::new()
        .utterance_with_voice_id(
            "I can speak with different voices and emotions!",
            "00aa8842-a5c5-forty-two-8448-8a5cea7b102e"
        )
        .utterance("This is the second sentence.")
        .unwrap()
        .format(AudioFormat::Wav)
        .sample_rate(SampleRate::HZ_44100)
        .build();
    
    match tts.synthesize(request, None).await {
        Ok(response) => {
            for (i, generation) in response.generations.iter().enumerate() {
                use base64::Engine;
                let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
                fs::write(format!("output_voice_{}.wav", i), &audio_data)?;
                
                if let Some(duration) = generation.duration_ms {
                    println!("✓ Saved output_voice_{}.wav ({}ms)", i, duration);
                } else {
                    println!("✓ Saved output_voice_{}.wav", i);
                }
                
                // Play each utterance
                println!("🔊 Playing utterance {}...", i);
                play_wav(&audio_data, &stream_handle)?;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    // Example 3: List available voices
    println!("\nExample 3: Available voices");
    match tts.list_voices(None).await {
        Ok(voices) => {
            println!("Found {} voices:", voices.voices.len());
            for voice in voices.voices.iter().take(5) {
                println!("  - {} ({})", voice.name, voice.id);
                if let Some(desc) = &voice.description {
                    println!("    {}", desc);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not list voices: {}", e);
        }
    }
    
    // Example 4: Streaming synthesis with real-time playback
    println!("\nExample 4: Streaming synthesis with chunked playback");
    let stream_request = TtsStreamRequest {
        text: "This text is being synthesized in real-time streaming mode! Each chunk plays as it arrives.".to_string(),
        voice: Some(VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        instant: Some(true),
        ..Default::default()
    };
    
    use futures_util::StreamExt;
    match tts.stream_file(stream_request, None).await {
        Ok(mut stream) => {
            let mut full_audio = Vec::new();
            let mut chunk_count = 0;
            
            println!("Receiving and playing chunks:");
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(data) => {
                        full_audio.extend_from_slice(&data);
                        chunk_count += 1;
                        print!("█");
                        std::io::Write::flush(&mut std::io::stdout())?;
                        
                        // For real streaming playback, you'd play chunks as they arrive
                        // This is simplified for the example
                    }
                    Err(e) => eprintln!("\nStream error: {}", e),
                }
            }
            println!("\n✓ Received {} chunks", chunk_count);
            
            if !full_audio.is_empty() {
                fs::write("output_stream.mp3", &full_audio)?;
                println!("✓ Saved streamed audio to output_stream.mp3");
                println!("🔊 Playing complete stream...");
                play_audio(&full_audio, &stream_handle)?;
            }
        }
        Err(e) => {
            eprintln!("Streaming error: {}", e);
        }
    }
    
    Ok(())
}

fn play_audio(audio_data: &[u8], stream_handle: &rodio::OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
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
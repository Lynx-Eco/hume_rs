//! TTS Streaming Example with real-time audio playback

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;
use futures_util::StreamExt;
use std::fs::File;
use std::io::{Write, Cursor};
use rodio::{OutputStream, Sink, Decoder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .unwrap_or_else(|_| {
            eprintln!("Warning: HUME_API_KEY not set. Using 'dummy' key for local testing.");
            "dummy".to_string()
        });
    
    let client = HumeClient::new(api_key)?;
    let tts = TtsClient::from(client);
    
    println!("TTS Streaming Example with Real-time Playback");
    println!("============================================\n");
    
    // Initialize audio output
    let (_stream, stream_handle) = OutputStream::try_default()?;
    
    // Example 1: Stream with real-time playback
    println!("Example 1: Real-time streaming playback");
    let realtime_request = TtsStreamRequest {
        text: "Hello! This is a real-time streaming example. \
                As I speak, the audio is being generated and played immediately. \
                This provides the lowest possible latency for voice applications.".to_string(),
        voice: Some(VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        description: Some("Speaking in a friendly, conversational tone".to_string()),
        speed: Some(1.0),
        format: Some(AudioFormat::Mp3),
        instant: Some(true),
        ..Default::default()
    };
    
    println!("→ Starting real-time playback...");
    match tts.stream_file(realtime_request, None).await {
        Ok(mut audio_stream) => {
            let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle)?));
            let mut chunk_buffer = Vec::new();
            let mut chunk_count = 0;
            
            print!("  Playing: ");
            while let Some(chunk_result) = audio_stream.next().await {
                match chunk_result {
                    Ok(data) => {
                        chunk_count += 1;
                        chunk_buffer.extend_from_slice(&data);
                        
                        // Play chunks when we have enough data (every 4KB or so)
                        if chunk_buffer.len() > 4096 {
                            let sink_guard = sink.lock().await;
                            if let Ok(source) = Decoder::new(Cursor::new(chunk_buffer.clone())) {
                                sink_guard.append(source);
                                print!("♪");
                                std::io::stdout().flush()?;
                            }
                            chunk_buffer.clear();
                        }
                    }
                    Err(e) => {
                        println!("\n  ✗ Stream error: {}", e);
                        break;
                    }
                }
            }
            
            // Play any remaining audio
            if !chunk_buffer.is_empty() {
                let sink_guard = sink.lock().await;
                if let Ok(source) = Decoder::new(Cursor::new(chunk_buffer)) {
                    sink_guard.append(source);
                }
            }
            
            // Wait for playback to complete
            let sink_guard = sink.lock().await;
            sink_guard.sleep_until_end();
            
            println!("\n  ✓ Played {} chunks in real-time", chunk_count);
        }
        Err(e) => {
            eprintln!("  Error: {}", e);
            eprintln!("  Note: Real-time streaming requires a valid API key");
        }
    }
    
    // Example 2: Stream with metadata tracking
    println!("\nExample 2: Streaming with metadata tracking");
    let json_request = TtsStreamRequest {
        text: "This example tracks timing and metadata for each audio chunk. \
                Perfect for synchronizing animations or subtitles with speech.".to_string(),
        voice: Some(VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        format: Some(AudioFormat::Mp3),
        instant: Some(true),
        ..Default::default()
    };
    
    match tts.stream_json(json_request, None).await {
        Ok(mut json_stream) => {
            let mut total_duration_ms = 0;
            let mut chunks_metadata = Vec::new();
            
            println!("  Chunk | Duration | Cumulative");
            println!("  ------|----------|------------");
            
            while let Some(chunk_result) = json_stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        let duration = chunk.duration_ms.unwrap_or(0);
                        total_duration_ms += duration;
                        
                        println!("  {:5} | {:8} | {:10}ms", 
                            chunk.index, 
                            format!("{}ms", duration),
                            total_duration_ms
                        );
                        
                        chunks_metadata.push((chunk.index, duration, chunk.is_final));
                        
                        if chunk.is_final {
                            println!("  ✓ Stream complete");
                        }
                    }
                    Err(e) => {
                        println!("  ✗ Error: {}", e);
                        break;
                    }
                }
            }
            
            println!("  Total duration: {}ms ({:.1}s)", total_duration_ms, total_duration_ms as f32 / 1000.0);
        }
        Err(e) => {
            eprintln!("  Error: {}", e);
        }
    }
    
    // Example 3: Parallel streaming with different voices
    println!("\nExample 3: Parallel streaming with multiple voices");
    let voices = vec![
        ("ITO", "Friendly and warm"),
        ("KORA", "Professional and clear"),
    ];
    
    let mut handles = Vec::new();
    
    for (voice_name, description) in voices {
        let tts_clone = tts.clone();
        let stream_handle_clone = stream_handle.clone();
        let voice_name = voice_name.to_string();
        let description = description.to_string();
        
        let handle = tokio::spawn(async move {
            let request = TtsStreamRequest {
                text: format!("Hello, I'm {}. {}", voice_name, description),
                voice: Some(VoiceSpec::Name {
                    name: voice_name.clone(),
                    provider: Some(VoiceProvider::HumeAi),
                }),
                format: Some(AudioFormat::Mp3),
                ..Default::default()
            };
            
            match tts_clone.stream_file(request, None).await {
                Ok(mut stream) => {
                    let mut audio_data = Vec::new();
                    while let Some(Ok(chunk)) = stream.next().await {
                        audio_data.extend_from_slice(&chunk);
                    }
                    
                    if !audio_data.is_empty() {
                        println!("  ✓ {} streamed {} bytes", voice_name, audio_data.len());
                        
                        // Play the voice
                        if let Ok(source) = Decoder::new(Cursor::new(audio_data)) {
                            let sink = Sink::try_new(&stream_handle_clone)?;
                            sink.append(source);
                            sink.sleep_until_end();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  ✗ {} error: {}", voice_name, e);
                }
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });
        
        handles.push(handle);
    }
    
    // Wait for all voices to complete
    for handle in handles {
        let _ = handle.await;
    }
    
    // Example 4: Interactive streaming with speed control
    println!("\nExample 4: Speed variations with streaming");
    
    let speeds = vec![
        (0.5, "Very slow"),
        (0.75, "Slow"),
        (1.0, "Normal"),
        (1.25, "Fast"),
        (1.5, "Very fast"),
    ];
    
    for (speed, label) in speeds {
        println!("\n→ {} speed ({}x)", label, speed);
        
        let request = TtsStreamRequest {
            text: "The quick brown fox jumps over the lazy dog.".to_string(),
            voice: Some(VoiceSpec::Id {
                id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
                provider: None,
            }),
            speed: Some(speed),
            format: Some(AudioFormat::Mp3),
            instant: Some(true),
            ..Default::default()
        };
        
        match tts.stream_file(request, None).await {
            Ok(mut stream) => {
                let mut audio_data = Vec::new();
                let mut chunks = 0;
                
                while let Some(Ok(chunk)) = stream.next().await {
                    audio_data.extend_from_slice(&chunk);
                    chunks += 1;
                }
                
                if !audio_data.is_empty() {
                    println!("  Received {} chunks, playing...", chunks);
                    
                    // Save to file
                    let filename = format!("speed_{}.mp3", label.replace(' ', "_").to_lowercase());
                    File::create(&filename)?.write_all(&audio_data)?;
                    
                    // Play audio
                    if let Ok(source) = Decoder::new(Cursor::new(audio_data)) {
                        let sink = Sink::try_new(&stream_handle)?;
                        sink.append(source);
                        sink.sleep_until_end();
                        println!("  ✓ Playback complete");
                    }
                }
            }
            Err(e) => {
                eprintln!("  Error: {}", e);
            }
        }
    }
    
    // Example 5: Chunked file writing with progress
    println!("\nExample 5: Progressive file writing");
    let progress_request = TtsStreamRequest {
        text: "This is a longer text that demonstrates progressive file writing. \
                As each chunk arrives, it's immediately written to disk. \
                This is useful for very long audio generation where you want to start \
                processing or playing the audio before it's fully generated.".to_string(),
        voice: Some(VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        format: Some(AudioFormat::Wav),
        ..Default::default()
    };
    
    match tts.stream_file(progress_request, None).await {
        Ok(mut stream) => {
            let mut output_file = File::create("progressive_output.wav")?;
            let mut total_bytes = 0;
            let start_time = std::time::Instant::now();
            
            print!("  Writing: ");
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(data) => {
                        total_bytes += data.len();
                        output_file.write_all(&data)?;
                        
                        // Show progress
                        let mb_written = total_bytes as f32 / (1024.0 * 1024.0);
                        print!("\r  Writing: {:.2} MB", mb_written);
                        std::io::stdout().flush()?;
                    }
                    Err(e) => {
                        println!("\n  ✗ Error: {}", e);
                        break;
                    }
                }
            }
            
            let elapsed = start_time.elapsed();
            println!("\n  ✓ Written {} bytes in {:.1}s", total_bytes, elapsed.as_secs_f32());
            println!("  ✓ Saved to progressive_output.wav");
        }
        Err(e) => {
            eprintln!("  Error: {}", e);
        }
    }
    
    println!("\n✓ All streaming examples completed!");
    
    Ok(())
}
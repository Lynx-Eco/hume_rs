//! TTS Streaming Example

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let tts = TtsClient::from(client);
    
    println!("TTS Streaming Example");
    println!("====================\n");
    
    // Example 1: Stream JSON responses
    println!("Example 1: Streaming JSON responses");
    let json_request = TtsStreamRequest {
        text: "Hello! This is a streaming synthesis example. \
                I'm speaking this text in real-time as it's being generated. \
                This allows for very low latency in voice applications.".to_string(),
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
    
    println!("→ Streaming with instant mode enabled...");
    let mut json_stream = tts.stream_json(json_request, None).await?;
    let mut total_chunks = 0;
    let mut total_duration_ms = 0;
    
    print!("  Receiving chunks: ");
    while let Some(chunk_result) = json_stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                total_chunks += 1;
                print!(".");
                std::io::stdout().flush()?;
                
                if let Some(duration) = chunk.duration_ms {
                    total_duration_ms += duration;
                }
                
                // First and last chunks get special handling
                if chunk.index == 0 {
                    println!("\n  ✓ First chunk received (index {})", chunk.index);
                }
                
                if chunk.is_final {
                    println!("\n  ✓ Final chunk received (index {})", chunk.index);
                }
            }
            Err(e) => {
                println!("\n  ✗ Stream error: {}", e);
                break;
            }
        }
    }
    
    println!("  Total chunks: {}", total_chunks);
    println!("  Total duration: {}ms", total_duration_ms);
    
    // Example 2: Stream raw audio file
    println!("\nExample 2: Streaming raw audio file");
    let file_request = TtsStreamRequest {
        text: "This audio is being streamed directly as binary data. \
                Perfect for real-time playback or saving to disk.".to_string(),
        voice: Some(VoiceSpec::Name {
            name: "ITO".to_string(),
            provider: Some(VoiceProvider::HumeAi),
        }),
        format: Some(AudioFormat::Wav),
        instant: Some(true),
        ..Default::default()
    };
    
    println!("→ Streaming WAV audio...");
    let mut audio_stream = tts.stream_file(file_request, None).await?;
    let mut output_file = File::create("streamed_output.wav")?;
    let mut total_bytes = 0;
    
    print!("  Receiving audio data: ");
    while let Some(chunk_result) = audio_stream.next().await {
        match chunk_result {
            Ok(data) => {
                total_bytes += data.len();
                output_file.write_all(&data)?;
                print!("■");
                std::io::stdout().flush()?;
            }
            Err(e) => {
                println!("\n  ✗ Stream error: {}", e);
                break;
            }
        }
    }
    
    println!("\n  ✓ Saved {} bytes to streamed_output.wav", total_bytes);
    
    // Example 3: Different voices and settings
    println!("\nExample 3: Streaming with different settings");
    
    let variations = vec![
        ("Slow speech", 0.8, "Speaking slowly and clearly"),
        ("Fast speech", 1.5, "Speaking quickly with energy"),
        ("Emotional", 1.0, "Speaking with joy and excitement!"),
    ];
    
    for (name, speed, description) in variations {
        println!("\n→ {}", name);
        
        let request = TtsStreamRequest {
            text: "The quick brown fox jumps over the lazy dog.".to_string(),
            voice: Some(VoiceSpec::Id {
                id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
                provider: None,
            }),
            description: Some(description.to_string()),
            speed: Some(speed),
            format: Some(AudioFormat::Mp3),
            instant: Some(true),
            ..Default::default()
        };
        
        let mut stream = tts.stream_file(request, None).await?;
        let filename = format!("stream_{}.mp3", name.replace(' ', "_").to_lowercase());
        let mut file = File::create(&filename)?;
        let mut bytes = 0;
        
        while let Some(Ok(data)) = stream.next().await {
            bytes += data.len();
            file.write_all(&data)?;
        }
        
        println!("  ✓ Saved {} bytes to {}", bytes, filename);
    }
    
    // Example 4: Error handling with streaming
    println!("\nExample 4: Handling streaming errors");
    
    let invalid_request = TtsStreamRequest {
        text: "".to_string(), // Empty text should cause an error
        ..Default::default()
    };
    
    match tts.stream_json(invalid_request, None).await {
        Ok(mut stream) => {
            if let Some(result) = stream.next().await {
                match result {
                    Ok(_) => println!("  Unexpected success"),
                    Err(e) => println!("  ✓ Caught expected error: {}", e),
                }
            }
        }
        Err(e) => {
            println!("  ✓ Request validation error: {}", e);
        }
    }
    
    println!("\n✓ Streaming examples completed!");
    println!("  Check the generated audio files:");
    println!("  - streamed_output.wav");
    println!("  - stream_slow_speech.mp3");
    println!("  - stream_fast_speech.mp3");
    println!("  - stream_emotional.mp3");
    
    Ok(())
}
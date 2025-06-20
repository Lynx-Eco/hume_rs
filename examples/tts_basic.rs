//! Basic Text-to-Speech example

use hume::{HumeClient, TtsClient};
use hume::tts::models::*;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Initialize the client with your API key
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let tts = TtsClient::from(client);
    
    // Example 1: Simple synthesis
    println!("Example 1: Simple synthesis");
    let audio_bytes = tts.synthesize_simple(
        "Hello! Welcome to the rust sdk for hume ai.",
        None::<String>,
    ).await?;
    
    fs::write("output_simple.mp3", &audio_bytes)?;
    println!("✓ Saved audio to output_simple.mp3");
    
    // Example 2: Synthesis with voice and options
    println!("\nExample 2: Synthesis with voice and options");
    let request = TtsRequestBuilder::new()
        .utterance_with_voice_id(
            "I can speak with different voices and emotions!",
            "00aa8842-a5c5-forty-two-8448-8a5cea7b102e"
        )
        .utterance("This is the second sentence.")
        .format(AudioFormat::Wav)
        .build();
    
    let response = tts.synthesize(request, None).await?;
    
    for (i, generation) in response.generations.iter().enumerate() {
        use base64::Engine;
        let audio_data = base64::engine::general_purpose::STANDARD.decode(&generation.data)?;
        fs::write(format!("output_voice_{}.wav", i), &audio_data)?;
        
        if let Some(duration) = generation.duration_ms {
            println!("✓ Saved output_voice_{}.wav ({}ms)", i, duration);
        } else {
            println!("✓ Saved output_voice_{}.wav", i);
        }
    }
    
    // Example 3: List available voices
    println!("\nExample 3: Available voices");
    let voices = tts.list_voices(None).await?;
    
    println!("Found {} voices:", voices.voices.len());
    for voice in voices.voices.iter().take(5) {
        println!("  - {} ({})", voice.name, voice.id);
        if let Some(desc) = &voice.description {
            println!("    {}", desc);
        }
    }
    
    // Example 4: Streaming synthesis
    println!("\nExample 4: Streaming synthesis");
    let stream_request = TtsStreamRequest {
        text: "This text is being synthesized in real-time streaming mode!".to_string(),
        voice: Some(VoiceSpec::Id {
            id: "00aa8842-a5c5-forty-two-8448-8a5cea7b102e".to_string(),
            provider: None,
        }),
        instant: Some(true),
        ..Default::default()
    };
    
    use futures_util::StreamExt;
    let mut stream = tts.stream_file(stream_request, None).await?;
    let mut full_audio = Vec::new();
    
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(data) => {
                full_audio.extend_from_slice(&data);
                print!(".");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            Err(e) => eprintln!("\nStream error: {}", e),
        }
    }
    println!("\n✓ Streaming complete");
    
    fs::write("output_stream.mp3", &full_audio)?;
    println!("✓ Saved streamed audio to output_stream.mp3");
    
    Ok(())
}
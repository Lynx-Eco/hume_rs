//! Real-time Conversation Example for macOS with actual audio I/O
//! 
//! This example demonstrates a real-time voice conversation with EVI on macOS.
//! It uses the microphone for input and plays audio responses through the speakers.

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::chat::{ChatSessionBuilder, ServerMessage};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use rodio::{OutputStream, Sink};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  EVI Real-time Conversation (macOS)");
    println!("=====================================\n");
    
    // Check for API key
    dotenvy::dotenv().ok();
    let api_key = match std::env::var("HUME_API_KEY") {
        Ok(key) if key != "dummy" && !key.is_empty() => key,
        _ => {
            println!("❌ This example requires a valid Hume API key.");
            println!("   Please set HUME_API_KEY environment variable.\n");
            show_audio_setup_guide();
            return Ok(());
        }
    };
    
    println!("✓ API key found");
    println!("📋 Initializing audio system...\n");
    
    // Initialize audio systems
    let host = cpal::default_host();
    
    // Get input device
    let input_device = host.default_input_device()
        .ok_or("No input device available")?;
    let input_config = input_device.default_input_config()?;
    println!("✓ Input device: {}", input_device.name()?);
    println!("  Sample rate: {} Hz", input_config.sample_rate().0);
    println!("  Channels: {}", input_config.channels());
    
    // Set up audio output
    let (_output_stream, output_handle) = OutputStream::try_default()?;
    println!("✓ Output device initialized");
    
    // Initialize Hume client
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    
    // Configure audio for real-time conversation
    let session_settings = SessionSettings {
        system_prompt: Some(
            "You are having a natural voice conversation. \
             Keep responses concise and conversational. \
             Be helpful and friendly.".to_string()
        ),
        audio: Some(AudioConfig {
            // Use Linear16 for direct microphone input
            input_encoding: Some(AudioEncoding::Linear16),
            input_sample_rate: Some(16000), // 16kHz for voice
            output_encoding: Some(AudioEncoding::Linear16),
            output_sample_rate: Some(24000), // 24kHz output
            output_format: Some(AudioFormat::Wav),
        }),
        context: None,
        variables: None,
        tools: None,
        builtin_tools: None,
    };
    
    // Connect to EVI
    println!("\n🔌 Connecting to EVI...");
    let chat = ChatSessionBuilder::new()
        .session_settings(session_settings)
        .connect(&evi.chat())
        .await?;
    
    println!("✓ Connected successfully!\n");
    
    // Set up audio channels
    let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<u8>>(100);
    let (text_tx, mut text_rx) = mpsc::channel::<String>(10);
    
    let chat = Arc::new(Mutex::new(chat));
    let chat_clone = chat.clone();
    
    // Audio buffer for accumulating output
    let output_sink = Arc::new(Mutex::new(Sink::try_new(&output_handle)?));
    let is_recording = Arc::new(AtomicBool::new(false));
    
    // Spawn task to handle incoming messages from EVI
    let output_sink_clone = output_sink.clone();
    let receive_handle = tokio::spawn(async move {
        let mut audio_buffer = Vec::new();
        
        loop {
            let message = {
                let mut chat_guard = chat_clone.lock().await;
                chat_guard.receive().await
            };
            
            match message {
                Ok(Some(ServerMessage::AssistantMessage { message_id: _, text, is_final: _ })) => {
                    println!("\n🤖 Assistant: {}", text);
                }
                Ok(Some(ServerMessage::AudioOutput { message_id: _, data, index: _ })) => {
                    // Decode base64 audio
                    use base64::Engine;
                    if let Ok(audio_data) = base64::engine::general_purpose::STANDARD.decode(&data) {
                        audio_buffer.extend(audio_data);
                        
                        // Play audio chunks as they arrive for lower latency
                        if audio_buffer.len() > 4096 { // Play when we have enough data
                            let sink = output_sink_clone.lock().await;
                            if let Ok(source) = rodio::Decoder::new_wav(Cursor::new(audio_buffer.clone())) {
                                sink.append(source);
                            }
                            audio_buffer.clear();
                        }
                    }
                }
                Ok(Some(ServerMessage::EmotionInference { inference })) => {
                    print!("😊 Emotions: ");
                    let mut emotions: Vec<_> = inference.emotions.iter()
                        .filter(|(_, score)| **score > 0.1)
                        .collect();
                    emotions.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                    
                    for (i, (emotion, score)) in emotions.iter().take(3).enumerate() {
                        if i > 0 { print!(", "); }
                        print!("{} ({:.0}%)", emotion, *score * 100.0);
                    }
                    println!();
                }
                Ok(Some(ServerMessage::SessionEnded { reason, .. })) => {
                    println!("\n📴 Session ended: {}", reason);
                    break;
                }
                Ok(Some(ServerMessage::Error { message, code, .. })) => {
                    eprintln!("\n❌ Error {}: {}", code, message);
                    break;
                }
                Ok(None) => {
                    println!("\n📴 Connection closed");
                    break;
                }
                Err(e) => {
                    eprintln!("\n❌ Receive error: {}", e);
                    eprintln!("   This might be due to unexpected message format from the server");
                    // Don't break on first error, continue to see if we get valid messages
                }
                _ => {} // Handle other message types as needed
            }
        }
        
        // Play any remaining audio
        if !audio_buffer.is_empty() {
            let sink = output_sink_clone.lock().await;
            if let Ok(source) = rodio::Decoder::new_wav(Cursor::new(audio_buffer)) {
                sink.append(source);
            }
        }
    });
    
    // Spawn task to send audio to EVI
    let chat_clone = chat.clone();
    let send_audio_handle = tokio::spawn(async move {
        while let Some(audio_data) = audio_rx.recv().await {
            let mut chat_guard = chat_clone.lock().await;
            if let Err(e) = chat_guard.send_audio(audio_data).await {
                eprintln!("Failed to send audio: {}", e);
                break;
            }
        }
    });
    
    // Spawn task to send text to EVI
    let chat_clone = chat.clone();
    let send_text_handle = tokio::spawn(async move {
        while let Some(text) = text_rx.recv().await {
            let mut chat_guard = chat_clone.lock().await;
            if let Err(e) = chat_guard.send_text(text).await {
                eprintln!("Failed to send text: {}", e);
                break;
            }
        }
    });
    
    // Set up microphone input
    let audio_tx_clone = audio_tx.clone();
    let is_recording_clone = is_recording.clone();
    
    let input_stream = match input_config.sample_format() {
        cpal::SampleFormat::F32 => {
            input_device.build_input_stream(
                &input_config.into(),
                move |data: &[f32], _: &_| {
                    if is_recording_clone.load(Ordering::Relaxed) {
                        // Convert f32 to i16 for EVI
                        let i16_data: Vec<i16> = data.iter()
                            .map(|&sample| (sample * 32767.0) as i16)
                            .collect();
                        
                        // Convert to bytes
                        let bytes: Vec<u8> = i16_data.iter()
                            .flat_map(|&sample| sample.to_le_bytes())
                            .collect();
                        
                        // Send to EVI
                        let _ = audio_tx_clone.try_send(bytes);
                    }
                },
                |err| eprintln!("Audio input error: {}", err),
                None
            )?
        }
        _ => {
            eprintln!("Unsupported sample format");
            return Err("Unsupported audio format".into());
        }
    };
    
    input_stream.play()?;
    
    // Instructions
    println!("🎤 Conversation started!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Instructions:");
    println!("  • Press SPACE to start/stop recording");
    println!("  • Type messages to send text");
    println!("  • Type 'quit' to exit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Send initial greeting
    text_tx.send("Hello! I'm ready to have a conversation.".to_string()).await?;
    
    // Interactive loop
    let stdin = std::io::stdin();
    let mut input = String::new();
    
    // Simple terminal input handling (in production, use a proper TUI library)
    println!("Press ENTER to toggle recording, or type a message:");
    
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        input.clear();
        stdin.read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "quit" | "exit" => {
                println!("Ending conversation...");
                break;
            }
            "" => {
                // Toggle recording
                let was_recording = is_recording.load(Ordering::Relaxed);
                is_recording.store(!was_recording, Ordering::Relaxed);
                
                if !was_recording {
                    println!("🔴 Recording... (press ENTER to stop)");
                } else {
                    println!("⏸️  Stopped recording");
                }
            }
            text => {
                text_tx.send(text.to_string()).await?;
            }
        }
    }
    
    // Clean up
    drop(audio_tx);
    drop(text_tx);
    drop(input_stream);
    
    // The chat connection will be closed when all Arc references are dropped
    
    // Wait for tasks to complete
    let _ = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            let _ = receive_handle.await;
            let _ = send_audio_handle.await;
            let _ = send_text_handle.await;
        }
    ).await;
    
    println!("\n✓ Conversation ended");
    Ok(())
}

fn show_audio_setup_guide() {
    println!("📖 Real-time Audio Setup Guide");
    println!("==============================\n");
    
    println!("This example demonstrates real-time audio conversation with EVI.");
    println!("\nFeatures:");
    println!("  • Live microphone input");
    println!("  • Real-time audio streaming");
    println!("  • Speaker output with low latency");
    println!("  • Emotion inference display");
    
    println!("\nRequirements:");
    println!("  1. Valid HUME_API_KEY environment variable");
    println!("  2. Microphone permissions on macOS");
    println!("  3. Working audio input/output devices");
    
    println!("\nTo grant microphone permissions:");
    println!("  1. Run the app once - macOS will prompt for permission");
    println!("  2. Go to System Preferences > Security & Privacy > Microphone");
    println!("  3. Enable microphone access for your terminal app");
}
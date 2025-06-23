//! EVI WebSocket Connection Test
//! 
//! This example tests the EVI chat connection to ensure:
//! 1. WebSocket connection is established correctly
//! 2. Authentication is handled properly
//! 3. Messages are sent and received correctly
//! 4. Connection stays alive during conversation
//! 5. Proper cleanup on disconnect

use hume::{HumeClientBuilder, EviClient};
use hume::evi::{
    chat::{ChatSessionBuilder, ServerMessage},
    models::{AudioEncoding, AudioConfig, AudioFormat, SessionSettings},
    configs::CreateConfigRequest,
};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("HUME_API_KEY must be set");

    println!("🔗 EVI WebSocket Connection Test");
    println!("{}", "=".repeat(50));

    // Create client
    let client = HumeClientBuilder::new(api_key).build()?;
    let evi = EviClient::from(client);

    // Step 1: List available configurations
    println!("\n📋 Available Configurations:");
    match evi.configs().list(None, None, None).await {
        Ok(configs) => {
            if let Some(configs_page) = configs.configs_page {
                if configs_page.is_empty() {
                    println!("  No custom configurations found");
                } else {
                    for config in &configs_page[..3.min(configs_page.len())] {
                        println!("  - {} (ID: {})", config.name, config.id);
                    }
                }
            } else {
                println!("  No configurations found");
            }
        }
        Err(e) => {
            println!("  ❌ Error listing configs: {}", e);
        }
    }

    // Step 2: Create a test configuration
    println!("\n🔧 Creating Test Configuration:");
    let test_config_name = format!("rust-sdk-test-{}", chrono::Utc::now().timestamp());
    
    let config_request = CreateConfigRequest::builder(test_config_name.clone())
        .voice("ito") // Built-in voice
        .build();
        
    let test_config = match evi.configs().create(config_request, None).await {
        Ok(config) => {
            println!("  ✅ Created config: {} (ID: {})", config.name, config.id);
            Some(config)
        }
        Err(e) => {
            println!("  ❌ Error creating config: {}", e);
            None
        }
    };

    // Step 3: Test WebSocket connection
    println!("\n🌐 Testing WebSocket Connection:");
    
    let session_settings = SessionSettings {
        system_prompt: Some("Test the connection by responding to messages.".to_string()),
        audio: Some(AudioConfig {
            input_encoding: Some(AudioEncoding::Linear16),
            input_sample_rate: Some(16000),
            output_encoding: Some(AudioEncoding::Linear16),
            output_sample_rate: Some(16000),
            output_format: Some(AudioFormat::Wav),
        }),
        context: None,
        variables: None,
        tools: None,
        builtin_tools: None,
    };

    // Create chat session
    let mut socket = match ChatSessionBuilder::new()
        .config_id(test_config.as_ref().map(|c| c.id.clone()).unwrap_or_default())
        .session_settings(session_settings)
        .connect(&evi.chat())
        .await
    {
        Ok(socket) => {
            println!("  ✅ WebSocket connected successfully!");
            socket
        }
        Err(e) => {
            println!("  ❌ Failed to connect: {}", e);
            // Clean up config if created
            if let Some(config) = test_config {
                let _ = evi.configs().delete(&config.id, None).await;
            }
            return Err(e.into());
        }
    };

    // Step 4: Test message exchange
    println!("\n💬 Testing Message Exchange:");
    
    // Send initial message
    println!("  📤 Sending: 'Hello, can you hear me?'");
    if let Err(e) = socket.send_text("Hello, can you hear me?".to_string()).await {
        println!("  ❌ Failed to send message: {}", e);
    }

    // Receive messages with timeout
    let mut message_count = 0;
    let receive_timeout = Duration::from_secs(10);
    
    println!("  ⏳ Waiting for responses (10s timeout)...");
    
    loop {
        match timeout(receive_timeout, socket.receive()).await {
            Ok(Ok(Some(message))) => {
                message_count += 1;
                match message {
                    ServerMessage::SessionStarted { session_id, chat_id, .. } => {
                        println!("  ✅ Session started:");
                        println!("     - Session ID: {}", session_id);
                        println!("     - Chat ID: {}", chat_id);
                    }
                    ServerMessage::AssistantMessage { text, is_final, .. } => {
                        println!("  🤖 Assistant: {}", text);
                        if is_final {
                            println!("     (Message complete)");
                            // Send another message to test continued conversation
                            if message_count < 5 {
                                println!("\n  📤 Sending: 'What's 2+2?'");
                                let _ = socket.send_text("What's 2+2?".to_string()).await;
                            } else {
                                break;
                            }
                        }
                    }
                    ServerMessage::AudioOutput { index, .. } => {
                        println!("  🔊 Audio output (chunk {})", index);
                    }
                    ServerMessage::UserMessage { text, .. } => {
                        println!("  👤 User echo: {}", text);
                    }
                    ServerMessage::Error { message, code, .. } => {
                        println!("  ❌ Error: {} (code: {})", message, code);
                        break;
                    }
                    ServerMessage::Warning { message, .. } => {
                        println!("  ⚠️  Warning: {}", message);
                    }
                    ServerMessage::SessionEnded { reason, .. } => {
                        println!("  🔚 Session ended: {}", reason);
                        break;
                    }
                    _ => {
                        println!("  📨 Other message: {:?}", message);
                    }
                }
            }
            Ok(Ok(None)) => {
                println!("  🔚 Connection closed by server");
                break;
            }
            Ok(Err(e)) => {
                println!("  ❌ Receive error: {}", e);
                break;
            }
            Err(_) => {
                println!("  ⏱️  Timeout - no more messages");
                break;
            }
        }
    }

    // Step 5: Test audio streaming
    println!("\n🎤 Testing Audio Streaming:");
    
    // Send a small audio sample (silence)
    let audio_data = vec![0u8; 1600]; // 100ms of silence at 16kHz
    println!("  📤 Sending audio data (100ms silence)");
    
    if let Err(e) = socket.send_audio(audio_data).await {
        println!("  ❌ Failed to send audio: {}", e);
    } else {
        println!("  ✅ Audio sent successfully");
        
        // Wait for any audio-related responses
        match timeout(Duration::from_secs(3), socket.receive()).await {
            Ok(Ok(Some(message))) => {
                println!("  📨 Response: {:?}", message);
            }
            _ => {
                println!("  ⏱️  No immediate audio response");
            }
        }
    }

    // Step 6: Test graceful disconnect
    println!("\n🔌 Testing Graceful Disconnect:");
    match socket.close().await {
        Ok(_) => println!("  ✅ Connection closed gracefully"),
        Err(e) => println!("  ❌ Error closing connection: {}", e),
    }

    // Step 7: Clean up test configuration
    if let Some(config) = test_config {
        println!("\n🧹 Cleaning Up:");
        match evi.configs().delete(&config.id, None).await {
            Ok(_) => println!("  ✅ Test configuration deleted"),
            Err(e) => println!("  ❌ Error deleting config: {}", e),
        }
    }

    println!("\n✨ Connection test complete!");
    println!("\n📊 Summary:");
    println!("  - WebSocket connection: ✅");
    println!("  - Message exchange: {}", if message_count > 0 { "✅" } else { "❌" });
    println!("  - Received {} messages", message_count);

    Ok(())
}
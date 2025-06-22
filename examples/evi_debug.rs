//! Debug EVI messages to understand the JSON structure

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::chat::ChatSessionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 EVI Message Debug");
    println!("===================\n");
    
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")?;
    
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    
    // Simple session settings
    let session_settings = SessionSettings {
        system_prompt: Some("You are a helpful assistant.".to_string()),
        audio: None,
        context: None,
        variables: None,
        tools: None,
        builtin_tools: None,
    };
    
    println!("Connecting to EVI...");
    
    // Connect using the normal API
    let mut chat = ChatSessionBuilder::new()
        .session_settings(session_settings)
        .connect(&evi.chat())
        .await?;
    
    println!("✓ Connected to EVI\n");
    
    // Listen for messages and debug them
    println!("Listening for messages (sending initial greeting)...");
    println!("------------------------");
    
    // Send initial message
    chat.send_text("Hello, this is a debug test.".to_string()).await?;
    println!("→ Sent: \"Hello, this is a debug test.\"\n");
    
    let mut count = 0;
    loop {
        match chat.receive().await {
            Ok(Some(msg)) => {
                count += 1;
                println!("\n📨 Message {}:", count);
                
                // Print the debug representation
                println!("{:#?}", msg);
                
                // Handle specific message types
                use hume::evi::chat::ServerMessage;
                match msg {
                    ServerMessage::SessionStarted { session_id, chat_id, .. } => {
                        println!("   Type: SessionStarted");
                        println!("   Session ID: {}", session_id);
                        println!("   Chat ID: {}", chat_id);
                    }
                    ServerMessage::AssistantMessage { message_id, text, is_final } => {
                        println!("   Type: AssistantMessage");
                        println!("   Message ID: {:?}", message_id);
                        println!("   Text: {}", text);
                        println!("   Is Final: {}", is_final);
                    }
                    ServerMessage::AudioOutput { message_id, index, .. } => {
                        println!("   Type: AudioOutput");
                        println!("   Message ID: {:?}", message_id);
                        println!("   Index: {}", index);
                    }
                    ServerMessage::EmotionInference { inference } => {
                        println!("   Type: EmotionInference");
                        // Show top 3 emotions
                        let mut emotions: Vec<_> = inference.emotions.iter().collect();
                        emotions.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                        println!("   Top emotions:");
                        for (emotion, score) in emotions.iter().take(3) {
                            println!("     - {}: {:.2}%", emotion, *score * 100.0);
                        }
                    }
                    ServerMessage::Error { message, code, .. } => {
                        println!("   Type: Error");
                        println!("   Code: {}", code);
                        println!("   Message: {}", message);
                    }
                    _ => {
                        println!("   Type: Other");
                    }
                }
                
                // Stop after several messages
                if count >= 10 {
                    println!("\n📊 Received {} messages, stopping...", count);
                    break;
                }
            }
            Ok(None) => {
                println!("\n🔚 Connection closed by server");
                break;
            }
            Err(e) => {
                eprintln!("\n❌ Error receiving message: {}", e);
                eprintln!("   Error details: {:?}", e);
                // Continue listening
            }
        }
    }
    
    println!("\nClosing connection...");
    chat.close().await?;
    println!("✓ Debug session complete");
    
    Ok(())
}
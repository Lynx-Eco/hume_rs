//! Simple EVI test to debug connection issues

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::chat::ChatSessionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Simple EVI Test");
    println!("==================\n");
    
    dotenvy::dotenv().ok();
    let api_key = match std::env::var("HUME_API_KEY") {
        Ok(key) if !key.is_empty() && key != "dummy" => key,
        _ => {
            eprintln!("❌ Valid API key required");
            return Ok(());
        }
    };
    
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    
    // Minimal session settings
    let session_settings = SessionSettings {
        system_prompt: Some("You are a helpful assistant.".to_string()),
        audio: None, // Start without audio config
        context: None,
        variables: None,
        tools: None,
        builtin_tools: None,
    };
    
    println!("Connecting to EVI...");
    let mut chat = ChatSessionBuilder::new()
        .session_settings(session_settings)
        .connect(&evi.chat())
        .await?;
    
    println!("✓ Connected!\n");
    
    // Don't send any message initially, just listen
    println!("Listening for messages...");
    
    let mut message_count = 0;
    loop {
        match chat.receive().await {
            Ok(Some(msg)) => {
                message_count += 1;
                println!("\n📨 Message #{}: {:?}", message_count, msg);
                
                // After receiving initial messages, try sending text
                if message_count == 1 {
                    println!("\nSending test message...");
                    chat.send_text("Hello, this is a test.".to_string()).await?;
                }
                
                // Stop after a few messages
                if message_count >= 5 {
                    break;
                }
            }
            Ok(None) => {
                println!("\n📴 Connection closed");
                break;
            }
            Err(e) => {
                eprintln!("\n❌ Error: {}", e);
                eprintln!("   Continuing to listen...");
            }
        }
    }
    
    println!("\nClosing connection...");
    chat.close().await?;
    println!("✓ Done");
    
    Ok(())
}
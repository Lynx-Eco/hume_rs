//! EVI Chat example

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::chat::{ChatSessionBuilder, ServerMessage};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
  
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    
    // Optional: List available configs
    println!("Available configurations:");
    let configs = evi.configs().list(None, None, None).await?;
    if let Some(configs_page) = configs.configs_page {
        for config in configs_page.iter().take(3) {
            println!("  - {} (v{}): {}", config.id, config.version, config.name);
        }
    }
    
    // Connect to chat
    println!("\nConnecting to EVI chat...");
    
    let session_settings = SessionSettings {
        system_prompt: Some("You are a helpful and friendly AI assistant.".to_string()),
        audio: Some(AudioConfig {
            input_encoding: Some(AudioEncoding::Linear16),
            input_sample_rate: Some(16000),
            output_encoding: Some(AudioEncoding::Linear16),
            output_sample_rate: Some(24000),
            output_format: Some(AudioFormat::Wav),
        }),
        context: None,
        variables: None,
        tools: None,
        builtin_tools: None,
    };
    
    let mut chat = ChatSessionBuilder::new()
        .session_settings(session_settings)
        .connect(&evi.chat())
        .await?;
    
    println!("✓ Connected to EVI chat");
    println!("Type your messages (or 'quit' to exit):\n");
    
    // Start a task to handle incoming messages
    let handle = tokio::spawn(async move {
        loop {
            match chat.receive().await {
                Ok(Some(message)) => {
                    handle_server_message(message);
                }
                Ok(None) => {
                    println!("\n[Chat ended]");
                    break;
                }
                Err(e) => {
                    eprintln!("\n[Error receiving message: {}]", e);
                    break;
                }
            }
        }
    });
    
    // Read user input
    let stdin = io::stdin();
    let mut input = String::new();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        input.clear();
        stdin.read_line(&mut input)?;
        
        let text = input.trim();
        if text == "quit" {
            break;
        }
        
        if !text.is_empty() {
            // Note: In a real application, you would send this through the chat socket
            // For this example, we're just showing the structure
            println!("[Would send: {}]", text);
        }
    }
    
    // Clean up
    drop(handle);
    
    Ok(())
}

fn handle_server_message(message: ServerMessage) {
    match message {
        ServerMessage::SessionStarted { session_id, chat_id, config, .. } => {
            println!("[Session started]");
            println!("  Session ID: {}", session_id);
            println!("  Chat ID: {}", chat_id);
            println!("  Config: {} (v{})", config.name, config.version);
        }
        ServerMessage::UserMessage { text, .. } => {
            println!("\nYou: {}", text);
        }
        ServerMessage::AssistantMessage { text, is_final, .. } => {
            if is_final {
                println!("\nAssistant: {}", text);
            } else {
                print!("{}", text);
                io::stdout().flush().ok();
            }
        }
        ServerMessage::AudioOutput { index, .. } => {
            println!("[Received audio chunk {}]", index);
        }
        ServerMessage::EmotionInference { inference } => {
            println!("\n[Emotion inference:]");
            for (emotion, score) in inference.emotions.iter() {
                if *score > 0.1 {
                    println!("  {}: {:.2}", emotion, score);
                }
            }
        }
        ServerMessage::ToolCall { name, parameters, .. } => {
            println!("\n[Tool call: {} with params: {}]", name, parameters);
        }
        ServerMessage::Error { message, code, .. } => {
            eprintln!("\n[Error {}: {}]", code, message);
        }
        ServerMessage::Warning { message, .. } => {
            println!("\n[Warning: {}]", message);
        }
        ServerMessage::SessionEnded { reason, .. } => {
            println!("\n[Session ended: {}]", reason);
        }
        _ => {}
    }
}
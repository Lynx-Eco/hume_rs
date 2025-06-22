//! EVI Chat Demo - Demonstrates EVI setup and features
//! 
//! This example shows how to use the Empathic Voice Interface (EVI) API.
//! It demonstrates both what works without authentication and what requires a valid API key.

use hume::{HumeClient, EviClient};
use hume::evi::models::*;
use hume::evi::chat::ChatSessionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("EVI Chat Demo");
    println!("=============");
    println!("\nThis demo shows how to use Hume's Empathic Voice Interface (EVI).\n");
    
    // Load API key
    dotenvy::dotenv().ok();
    let has_valid_key = std::env::var("HUME_API_KEY")
        .map(|k| !k.is_empty() && k != "dummy")
        .unwrap_or(false);
    
    if !has_valid_key {
        println!("📋 Running in DEMO MODE (no API key)");
        println!("   This will show you how to set up EVI, but cannot connect to the service.");
        println!("   To run the full example, set HUME_API_KEY environment variable.\n");
        demonstrate_setup();
        return Ok(());
    }
    
    println!("✓ API key found - running full example\n");
    let api_key = std::env::var("HUME_API_KEY")?;
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    
    // Show available resources
    show_available_resources(&evi).await?;
    
    // Demonstrate chat setup
    demonstrate_chat_session(&evi).await?;
    
    Ok(())
}

async fn show_available_resources(evi: &EviClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Available EVI Resources:");
    println!("---------------------------\n");
    
    // List configurations
    println!("1. Configurations:");
    match evi.configs().list(Some(5), None, None).await {
        Ok(configs) => {
            if let Some(list) = configs.configs_page {
                for config in list.iter() {
                    println!("   - {} (v{}): {}", config.id, config.version, config.name);
                    if let Some(prompt) = &config.prompt {
                        println!("     Prompt: {}", prompt.id);
                    }
                }
            } else {
                println!("   No configurations found");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // List tools
    println!("\n2. Available Tools:");
    match evi.tools().list(Some(5), None, None).await {
        Ok(tools) => {
            for tool_opt in tools.tools_page.iter() {
                if let Some(tool) = tool_opt {
                    println!("   - {} ({}): {}", tool.name, tool.id, tool.description);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // List prompts
    println!("\n3. Available Prompts:");
    match evi.prompts().list(Some(5), None, None).await {
        Ok(prompts) => {
            for prompt_opt in prompts.prompts_page.iter() {
                if let Some(prompt) = prompt_opt {
                    println!("   - {} (v{})", prompt.id, prompt.version.unwrap_or(0));
                    if let Some(desc) = &prompt.version_description {
                        println!("     {}", desc);
                    }
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // List voices
    println!("\n4. Custom Voices:");
    match evi.voices().list(Some(5), None, None).await {
        Ok(voices) => {
            for voice in voices.custom_voices_page.iter() {
                println!("   - {} ({})", voice.name, voice.id);
                println!("     Based on: {}", voice.base_voice_id);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    println!();
    Ok(())
}

async fn demonstrate_chat_session(evi: &EviClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("💬 Chat Session Demo:");
    println!("--------------------\n");
    
    // Show how to configure a session
    let session_settings = SessionSettings {
        system_prompt: Some("You are a helpful and empathetic AI assistant.".to_string()),
        audio: Some(AudioConfig {
            input_encoding: Some(AudioEncoding::Linear16),
            input_sample_rate: Some(16000),
            output_encoding: Some(AudioEncoding::Linear16),
            output_sample_rate: Some(24000),
            output_format: Some(AudioFormat::Wav),
        }),
        context: Some(Context {
            context_type: ContextType::Temporary,
            text: "Previous conversation context can go here".to_string(),
        }),
        variables: None,
        tools: None,
        builtin_tools: None,
    };
    
    println!("Session configuration:");
    println!("  - System prompt: Set ✓");
    println!("  - Audio: 16kHz input, 24kHz output (Linear16/WAV)");
    println!("  - Context: Included");
    
    println!("\nAttempting to connect...");
    
    let builder = ChatSessionBuilder::new()
        .session_settings(session_settings);
    
    match builder.connect(&evi.chat()).await {
        Ok(mut chat) => {
            println!("✓ Connected successfully!");
            println!("\nChat features available:");
            println!("  - Send text messages");
            println!("  - Send audio data");
            println!("  - Receive audio responses");
            println!("  - Get emotion inferences");
            println!("  - Handle tool calls");
            
            // Send a test message
            println!("\nSending test message...");
            chat.send_text("Hello! This is a test message.".to_string()).await?;
            
            // Wait for a response
            match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                chat.receive()
            ).await {
                Ok(Ok(Some(message))) => {
                    println!("Received response: {:?}", message);
                }
                Ok(Ok(None)) => {
                    println!("No response received");
                }
                Ok(Err(e)) => {
                    println!("Error receiving response: {}", e);
                }
                Err(_) => {
                    println!("Timeout waiting for response");
                }
            }
            
            // Close the connection
            chat.close().await?;
            println!("\n✓ Connection closed successfully");
        }
        Err(e) => {
            println!("✗ Could not connect: {}", e);
            println!("  This typically means:");
            println!("  - Invalid API key");
            println!("  - No available configurations");
            println!("  - Network issues");
        }
    }
    
    Ok(())
}

fn demonstrate_setup() {
    println!("📖 EVI Setup Guide");
    println!("==================\n");
    
    println!("1. Basic Chat Connection:");
    println!("```rust");
    println!("use hume::{{HumeClient, EviClient}};");
    println!("use hume::evi::chat::ChatSessionBuilder;");
    println!();
    println!("let client = HumeClient::new(api_key)?;");
    println!("let evi = EviClient::from(client);");
    println!();
    println!("let chat = ChatSessionBuilder::new()");
    println!("    .config_id(\"your-config-id\")  // Optional");
    println!("    .build()");
    println!("    .connect(&evi)");
    println!("    .await?;");
    println!("```\n");
    
    println!("2. Session with Audio Configuration:");
    println!("```rust");
    println!("let session = SessionSettings {{");
    println!("    audio: Some(AudioConfig {{");
    println!("        input_encoding: Some(AudioEncoding::Linear16),");
    println!("        input_sample_rate: Some(16000),");
    println!("        output_encoding: Some(AudioEncoding::Linear16),");
    println!("        output_sample_rate: Some(24000),");
    println!("        output_format: Some(AudioFormat::Wav),");
    println!("    }}),");
    println!("    system_prompt: Some(\"Your prompt here\".to_string()),");
    println!("    ..Default::default()");
    println!("}};");
    println!();
    println!("let chat = ChatSessionBuilder::new()");
    println!("    .session_settings(session)");
    println!("    .build()");
    println!("    .connect(&evi)");
    println!("    .await?;");
    println!("```\n");
    
    println!("3. Handling Messages:");
    println!("```rust");
    println!("// Send text");
    println!("chat.send_text(\"Hello!\").await?;");
    println!();
    println!("// Send audio");
    println!("let audio_data: Vec<u8> = // your audio data");
    println!("chat.send_audio(audio_data).await?;");
    println!();
    println!("// Receive messages");
    println!("while let Some(message) = chat.receive().await? {{");
    println!("    match message {{");
    println!("        ServerMessage::AssistantMessage {{ message }} => {{");
    println!("            println!(\"Assistant: {{}}\", message);");
    println!("        }}");
    println!("        ServerMessage::AudioOutput {{ data, .. }} => {{");
    println!("            // Handle audio data");
    println!("        }}");
    println!("        ServerMessage::EmotionInference {{ inference }} => {{");
    println!("            // Handle emotion data");
    println!("        }}");
    println!("        // ... handle other message types");
    println!("    }}");
    println!("}}");
    println!("```\n");
    
    println!("4. Creating Configurations:");
    println!("```rust");
    println!("use hume::evi::configs::CreateConfigRequest;");
    println!();
    println!("let config = CreateConfigRequest::builder(\"my-config\")");
    println!("    .prompt(\"prompt-id\", Some(1))");
    println!("    .voice(\"voice-id\")");
    println!("    .language_model(\"gpt-4\", \"openai\", Some(0.7))");
    println!("    .build();");
    println!();
    println!("let created = evi.configs().create(config, None).await?;");
    println!("```");
}
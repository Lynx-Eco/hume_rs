//! EVI Configuration Management Example

use hume::{HumeClient, EviClient};
use hume::evi::configs::CreateConfigRequest;
use hume::evi::models::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    let configs_client = evi.configs();
    
    // Example 1: List existing configurations
    println!("Example 1: Listing existing configurations");
    let configs = configs_client.list(None, None, None).await?;
    
    if let Some(configs_list) = &configs.configs_page {
        println!("Found {} configurations:", configs_list.len());
        for config in configs_list.iter().take(5) {
            println!("  - {} (v{}): {}", config.id, config.version, config.name);
            if let Some(prompt) = &config.prompt {
                println!("    Prompt: {} (v{})", prompt.id, prompt.version.unwrap_or(0));
            }
        }
    } else {
        println!("No configurations found");
    }
    
    // Example 2: Create a new configuration
    println!("\nExample 2: Creating a new configuration");
    
    // First, let's list available prompts and voices
    let prompts = evi.prompts().list(None, None, None).await?;
    let voices = evi.voices().list(None, None, None).await?;
    
    let first_prompt = prompts.prompts_page.iter().find_map(|p| p.as_ref());
    let first_voice = voices.custom_voices_page.first();
    
    if let (Some(first_prompt), Some(first_voice)) = (first_prompt, first_voice) {
        let new_config = CreateConfigRequest::builder("customer-support-config")
            .prompt(&first_prompt.id, first_prompt.version)
            .voice(&first_voice.id)
            .language_model("openai", "gpt-4", Some(0.7))
            .event_messages(EventMessagesSpec {
                on_new_chat: Some("Hello! How can I assist you today?".to_string()),
                on_inactivity_timeout: Some("Are you still there? Let me know if you need help.".to_string()),
                on_max_duration_timeout: Some("Our session time is up. Thank you for chatting!".to_string()),
            })
            .timeouts(Some(120), Some(1800)) // 2 min inactivity, 30 min max
            .build();
        
        match configs_client.create(new_config, None).await {
            Ok(config) => {
                println!("✓ Created config: {} (v{})", config.id, config.version);
                
                // Example 3: Get config details
                println!("\nExample 3: Getting config details");
                let retrieved = configs_client.get(&config.id, None).await?;
                println!("Config details:");
                println!("  Name: {}", retrieved.name);
                println!("  ID: {}", retrieved.id);
                println!("  Version: {}", retrieved.version);
                if let Some(lm) = &retrieved.language_model {
                    println!("  Language Model: {} - {}", lm.model_provider, lm.model_resource);
                }
                
                // Example 4: Update the config
                println!("\nExample 4: Updating config timeouts");
                use hume::evi::configs::UpdateConfigRequest;
                let update = UpdateConfigRequest {
                    name: Some("customer-support-config-v2".to_string()),
                    timeouts: Some(TimeoutsSpec {
                        inactivity: Some(180), // 3 minutes
                        max_duration: Some(2400), // 40 minutes
                    }),
                    ..Default::default()
                };
                
                let updated = configs_client.update(&config.id, update, None).await?;
                println!("✓ Updated config to version {}", updated.version);
                
                // Example 5: List config versions
                println!("\nExample 5: Listing config versions");
                let versions = configs_client.list_versions(&config.id, None, None, None).await?;
                if let Some(configs_list) = &versions.configs_page {
                    println!("Found {} versions:", configs_list.len());
                    for version in configs_list.iter() {
                        println!("  - Version {}: {}", version.version, version.name);
                        if let Some(created) = &version.created_at {
                            println!("    Created: {}", created);
                        }
                    }
                } else {
                    println!("No versions found");
                }
                
                // Example 6: Get specific version
                if let Some(configs_list) = &versions.configs_page {
                    if configs_list.len() > 1 {
                        let first_version = &configs_list[0];
                        println!("\nExample 6: Getting specific version");
                        let specific = configs_client.get_version(
                            &config.id, 
                            first_version.version, 
                            None
                        ).await?;
                        println!("Retrieved version {} with name: {}", 
                            specific.version, 
                            specific.name
                        );
                    }
                }
                
                // Example 7: Delete the config
                println!("\nExample 7: Deleting the config");
                configs_client.delete(&config.id, None).await?;
                println!("✓ Config deleted successfully");
            }
            Err(e) => {
                println!("Note: Config creation failed: {}", e);
                println!("This might be because no prompts or voices are available.");
            }
        }
    } else {
        println!("Note: No prompts or voices available to create a config");
    }
    
    Ok(())
}
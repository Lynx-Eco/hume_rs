//! Test basic connection to verify fixes

use hume::HumeClient;

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Hume connection...\n");
    
    // Load from .env file
    dotenvy::dotenv().ok();
    
    // Get API key
    let api_key = match std::env::var("HUME_API_KEY") {
        Ok(key) => {
            println!("✓ Found API key: {}...{}", 
                &key[..8.min(key.len())], 
                &key[key.len().saturating_sub(4)..]);
            key
        }
        Err(_) => {
            eprintln!("❌ HUME_API_KEY not found in environment");
            eprintln!("   Please set it or create a .env file");
            return Ok(());
        }
    };
    
    // Test basic client creation
    let client = HumeClient::new(api_key)?;
    println!("✓ Client created successfully");
    
    // Test TTS connection
    println!("\nTesting TTS API...");
    let tts = hume::TtsClient::from(client.clone());
    match tts.list_voices(None).await {
        Ok(voices) => {
            println!("✓ TTS API connected - found {} voices", voices.voices.len());
        }
        Err(e) => {
            println!("✗ TTS API error: {}", e);
        }
    }
    
    // Test EVI connection
    println!("\nTesting EVI API...");
    let evi = hume::EviClient::from(client.clone());
    match evi.configs().list(Some(1), None, None).await {
        Ok(_) => {
            println!("✓ EVI API connected");
        }
        Err(e) => {
            println!("✗ EVI API error: {}", e);
        }
    }
    
    println!("\nConnection test complete!");
    Ok(())
}
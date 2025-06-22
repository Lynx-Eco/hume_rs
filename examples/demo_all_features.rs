//! Demo of all Hume SDK features
//! This example shows what works with and without an API key

use hume::{HumeClient, TtsClient, ExpressionMeasurementClient, EviClient};
use hume::expression_measurement::models::{Models, LanguageModel, SentimentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Hume Rust SDK Feature Demo");
    println!("=============================\n");
    
    dotenvy::dotenv().ok();
    let has_api_key = std::env::var("HUME_API_KEY")
        .map(|k| !k.is_empty() && k != "dummy")
        .unwrap_or(false);
    
    if !has_api_key {
        println!("📋 Running in DEMO MODE (no API key)");
        println!("   Set HUME_API_KEY environment variable for full functionality\n");
    } else {
        println!("✓ API key found - full demo will run\n");
    }
    
    let api_key = std::env::var("HUME_API_KEY").unwrap_or_else(|_| "dummy".to_string());
    let client = HumeClient::new(api_key)?;
    
    // Feature 1: Text-to-Speech
    demo_tts(&client, has_api_key).await?;
    
    // Feature 2: Expression Measurement
    demo_expression_measurement(&client, has_api_key).await?;
    
    // Feature 3: Empathic Voice Interface
    demo_evi(&client, has_api_key).await?;
    
    println!("\n✅ Demo complete!");
    println!("\nNext steps:");
    println!("1. Get your API key from https://platform.hume.ai");
    println!("2. Set HUME_API_KEY environment variable");
    println!("3. Run this demo again to see all features in action");
    
    Ok(())
}

async fn demo_tts(client: &HumeClient, has_api_key: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🗣️  Text-to-Speech (TTS)");
    println!("----------------------");
    
    let tts = TtsClient::from(client.clone());
    
    // Show available features
    println!("Features:");
    println!("  ✓ Multiple voices (ITO, KORA, etc.)");
    println!("  ✓ Emotional context in speech");
    println!("  ✓ Speed control (0.5x - 2.0x)");
    println!("  ✓ Multiple formats (MP3, WAV, PCM)");
    println!("  ✓ Streaming synthesis");
    
    if has_api_key {
        // Try simple synthesis
        match tts.synthesize_simple("Hello from Hume AI!", None::<String>).await {
            Ok(audio) => println!("\n  ✓ Synthesized {} bytes of audio", audio.len()),
            Err(e) => println!("\n  ✗ Synthesis error: {}", e),
        }
        
        // List voices
        match tts.list_voices(None).await {
            Ok(voices) => {
                println!("  ✓ Found {} voices:", voices.voices.len());
                for voice in voices.voices.iter().take(3) {
                    println!("    - {}", voice.name);
                }
            }
            Err(e) => println!("  ✗ Could not list voices: {}", e),
        }
    } else {
        println!("\n  ℹ️  API key required for synthesis");
        println!("  Example usage:");
        println!("    let audio = tts.synthesize_simple(\"Hello!\", None).await?;");
    }
    
    Ok(())
}

async fn demo_expression_measurement(client: &HumeClient, has_api_key: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n🎭 Expression Measurement");
    println!("------------------------");
    
    let em = ExpressionMeasurementClient::from(client.clone());
    
    println!("Features:");
    println!("  ✓ Analyze text for emotions");
    println!("  ✓ Process audio/video files");
    println!("  ✓ Batch processing");
    println!("  ✓ Real-time streaming");
    println!("  ✓ 48+ emotion scores");
    
    if has_api_key {
        // Try text analysis
        let models = Models {
            language: Some(LanguageModel {
                sentiment: Some(SentimentConfig {}),
                toxicity: None,
                granularity: Some("sentence".to_string()),
            }),
            ..Default::default()
        };
        
        let texts = vec![
            "I'm so excited about this new SDK!".to_string(),
        ];
            
        match em.batch().create_job_from_text(models, texts, None, None, None).await {
            Ok(job) => {
                println!("\n  ✓ Created job: {}", job.job_id);
                println!("  Status: {:?}", job.state);
            }
            Err(e) => println!("\n  ✗ Job creation error: {}", e),
        }
    } else {
        println!("\n  ℹ️  API key required for analysis");
        println!("  Example emotions detected:");
        println!("    - Joy: 85%");
        println!("    - Excitement: 78%");
        println!("    - Contentment: 65%");
    }
    
    Ok(())
}

async fn demo_evi(client: &HumeClient, has_api_key: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n🤖 Empathic Voice Interface (EVI)");
    println!("---------------------------------");
    
    let evi = EviClient::from(client.clone());
    
    println!("Features:");
    println!("  ✓ Real-time voice conversations");
    println!("  ✓ Emotion-aware responses");
    println!("  ✓ WebSocket streaming");
    println!("  ✓ Custom voice & prompts");
    println!("  ✓ Tool integration");
    
    if has_api_key {
        // List configurations
        match evi.configs().list(Some(3), None, None).await {
            Ok(configs) => {
                if let Some(list) = configs.configs_page {
                    println!("\n  ✓ Found {} configurations", list.len());
                } else {
                    println!("\n  ℹ️  No configurations found");
                }
            }
            Err(e) => println!("\n  ✗ Could not list configs: {}", e),
        }
    } else {
        println!("\n  ℹ️  API key required for EVI");
        println!("  Example conversation:");
        println!("    User: \"How are you today?\"");
        println!("    EVI: \"I'm doing well, thank you! How can I help?\"");
        println!("    [Detected emotions: friendly, helpful]");
    }
    
    Ok(())
}
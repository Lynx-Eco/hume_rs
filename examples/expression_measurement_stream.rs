//! Expression Measurement Streaming Example

use hume::{HumeClient, ExpressionMeasurementClient};
use hume::expression_measurement::models::*;
use hume::expression_measurement::stream::{StreamBuilder, StreamMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let em = ExpressionMeasurementClient::from(client);
    
    println!("Expression Measurement Streaming Example");
    println!("=======================================\n");
    
    // Configure models for streaming
    let models = StreamBuilder::new()
        .with_language(LanguageModel {
            sentiment: Some(SentimentConfig {}),
            granularity: Some("sentence".to_string()),
            ..Default::default()
        })
        .with_prosody(ProsodyModel {
            granularity: Some("utterance".to_string()),
            window: Some(WindowConfig {
                length: 4.0,
                step: 1.0,
            }),
        })
        .build();
    
    println!("Configured models:");
    println!("  - Language: sentiment analysis at sentence level");
    println!("  - Prosody: utterance-level analysis with 4s window");
    
    // Connect to streaming endpoint
    println!("\nConnecting to streaming endpoint...");
    match em.stream().connect(models).await {
        Ok(mut socket) => {
            println!("✓ Connected successfully");
            
            // Send configuration
            socket.send_config().await?;
            println!("✓ Configuration sent");
            
            // Example 1: Stream text data
            println!("\nExample 1: Streaming text analysis");
            let texts = vec![
                "I'm really excited about this new project!",
                "The weather today is absolutely beautiful.",
                "I'm feeling a bit anxious about the presentation.",
                "This is frustrating, nothing seems to be working.",
                "Wow, that's amazing news! Congratulations!",
            ];
            
            // Send texts and receive results
            for text in texts {
                println!("\n→ Sending: \"{}\"", text);
                socket.send_text(text.to_string()).await?;
                
                // Receive predictions
                tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    async {
                        while let Some(message) = socket.receive().await? {
                            match message {
                                StreamMessage::JobDetails { job_id } => {
                                    println!("  Job ID: {}", job_id);
                                }
                                StreamMessage::Predictions { predictions } => {
                                    if let Some(language) = &predictions.language {
                                        for group in &language.grouped_predictions {
                                            println!("  Text: \"{}\"", group.text);
                                            for pred in &group.predictions {
                                                if let Some(sentiment) = &pred.sentiment {
                                                    println!("    Sentiment scores:");
                                                    println!("      Positive: {:.2}%", sentiment.positive * 100.0);
                                                    println!("      Negative: {:.2}%", sentiment.negative * 100.0);
                                                    println!("      Neutral: {:.2}%", sentiment.neutral * 100.0);
                                                }
                                                
                                                println!("    Top emotions:");
                                                let mut emotions: Vec<_> = pred.emotions.iter().collect();
                                                emotions.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap());
                                                
                                                for (emotion, score) in emotions.iter().take(3) {
                                                    if score.score > 0.1 {
                                                        println!("      {}: {:.2}%", emotion, score.score * 100.0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    break;
                                }
                                StreamMessage::Error { message, code } => {
                                    println!("  Error: {} (code: {:?})", message, code);
                                    break;
                                }
                                StreamMessage::Warning { message } => {
                                    println!("  Warning: {}", message);
                                }
                                _ => {}
                            }
                        }
                        Ok::<(), Box<dyn std::error::Error>>(())
                    }
                ).await??;
                
                // Small delay between messages
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            
            // Example 2: Simulated audio streaming (would use actual audio in practice)
            println!("\n\nExample 2: Simulated audio streaming");
            println!("Note: In a real application, you would send actual audio data");
            
            // Create fake audio data for demonstration
            let fake_audio = vec![0u8; 16000]; // 1 second of silence at 16kHz
            
            println!("→ Sending audio chunk (1s)");
            socket.send_audio(fake_audio).await?;
            
            // Note: Audio processing would return prosody results
            println!("  (Audio would be analyzed for prosody features)");
            
            // Close the connection
            println!("\nClosing connection...");
            socket.close().await?;
            println!("✓ Connection closed");
        }
        Err(e) => {
            println!("✗ Failed to connect: {}", e);
            println!("\nNote: Streaming requires an active WebSocket connection.");
            println!("Make sure your API key has streaming permissions.");
        }
    }
    
    Ok(())
}
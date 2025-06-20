//! Expression Measurement example

use hume::{HumeClient, ExpressionMeasurementClient};
use hume::expression_measurement::models::*;
use hume::expression_measurement::stream::StreamBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Initialize the client
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let em = ExpressionMeasurementClient::from(client);
    
    // Example 1: Batch processing with text
    println!("Example 1: Batch text analysis");
    
    let models = Models {
        language: Some(LanguageModel {
            sentiment: Some(SentimentConfig {}),
            toxicity: None,
            granularity: Some("sentence".to_string()),
        }),
        ..Default::default()
    };
    
    let texts = vec![
        "I'm so excited about this new opportunity!".to_string(),
        "This situation is making me feel anxious.".to_string(),
        "What a beautiful day to be alive!".to_string(),
    ];
    
    let job = em.batch()
        .create_job_from_text(models.clone(), texts, None, None, None)
        .await?;
    
    println!("✓ Created batch job: {}", job.job_id);
    println!("  Status: {:?}", job.state);
    
    // Wait for completion
    println!("\nWaiting for job to complete...");
    let completed_job = em.batch()
        .wait_for_job_completion(
            &job.job_id,
            std::time::Duration::from_secs(2),
            Some(std::time::Duration::from_secs(60)),
        )
        .await?;
    
    println!("✓ Job completed with status: {:?}", completed_job.state);
    
    // Get predictions
    if matches!(completed_job.state, hume::expression_measurement::models::StateInference::Completed { .. }) {
        let predictions = em.batch()
            .get_predictions(&job.job_id, None)
            .await?;
        
        println!("\nPrediction results:");
        for (i, prediction) in predictions.predictions.iter().enumerate() {
            println!("\nText {}: ", i + 1);
            
            if let Some(language) = &prediction.results.language {
                for group in &language.grouped_predictions {
                    println!("  \"{}\"", group.text);
                    
                    for pred in &group.predictions {
                        if let Some(sentiment) = &pred.sentiment {
                            println!("    Sentiment:");
                            println!("      Positive: {:.2}", sentiment.positive);
                            println!("      Negative: {:.2}", sentiment.negative);
                            println!("      Neutral: {:.2}", sentiment.neutral);
                        }
                        
                        println!("    Emotions:");
                        for (emotion, score) in &pred.emotions {
                            if score.score > 0.1 {
                                println!("      {}: {:.2}", emotion, score.score);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Example 2: List recent jobs
    println!("\n\nExample 2: Recent batch jobs");
    let jobs = em.batch().list_jobs(Some(5), None, None).await?;
    
    println!("Found {} total jobs. Recent jobs:", jobs.total);
    for job in jobs.jobs.iter() {
        let timestamp = match &job.state {
            hume::expression_measurement::models::StateInference::Queued { created_timestamp_ms } => *created_timestamp_ms,
            hume::expression_measurement::models::StateInference::InProgress { created_timestamp_ms, .. } => *created_timestamp_ms,
            hume::expression_measurement::models::StateInference::Completed { created_timestamp_ms, .. } => *created_timestamp_ms,
            hume::expression_measurement::models::StateInference::Failed { created_timestamp_ms, .. } => *created_timestamp_ms,
        };
        
        let status = match &job.state {
            hume::expression_measurement::models::StateInference::Queued { .. } => "QUEUED",
            hume::expression_measurement::models::StateInference::InProgress { .. } => "IN_PROGRESS",
            hume::expression_measurement::models::StateInference::Completed { .. } => "COMPLETED",
            hume::expression_measurement::models::StateInference::Failed { .. } => "FAILED",
        };
        
        println!("  - {} ({}): {}", 
            job.job_id, 
            chrono::DateTime::<chrono::Utc>::from_timestamp_millis(timestamp)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Unknown time".to_string()),
            status
        );
    }
    
    // Example 3: Streaming setup (not connected in this example)
    println!("\n\nExample 3: Streaming configuration");
    
    let stream_models = StreamBuilder::new()
        .with_face(FaceModel {
            identify_faces: Some(true),
            min_face_size: Some(60),
            fps_pred: Some(30.0),
            prob_threshold: Some(0.5),
        })
        .with_language(LanguageModel::default())
        .with_prosody(ProsodyModel {
            granularity: Some("utterance".to_string()),
            window: Some(WindowConfig {
                length: 2.0,
                step: 0.5,
            }),
        })
        .build();
    
    println!("✓ Created streaming configuration with:");
    if stream_models.face.is_some() {
        println!("  - Face expression analysis");
    }
    if stream_models.language.is_some() {
        println!("  - Language emotion analysis");
    }
    if stream_models.prosody.is_some() {
        println!("  - Speech prosody analysis");
    }
    
    println!("\nTo use streaming, connect with:");
    println!("  let mut socket = em.stream().connect(stream_models).await?;");
    println!("  socket.send_config().await?;");
    println!("  socket.send_text(\"Analyze this\".to_string()).await?;");
    
    Ok(())
}
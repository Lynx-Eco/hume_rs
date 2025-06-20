//! Expression Measurement File Processing Example

use hume::{HumeClient, ExpressionMeasurementClient};
use hume::expression_measurement::models::*;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let em = ExpressionMeasurementClient::from(client);
    
    println!("Expression Measurement File Processing Example");
    println!("=============================================\n");
    
    // Configure models for analysis
    let models = Models {
        face: Some(FaceModel {
            identify_faces: Some(true),
            min_face_size: Some(60),
            fps_pred: Some(3.0),
            prob_threshold: Some(0.5),
        }),
        language: Some(LanguageModel {
            sentiment: Some(SentimentConfig {}),
            granularity: Some("sentence".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    // Example 1: Process files from disk
    println!("Example 1: Processing local files");
    
    // Create a sample text file for demonstration
    let sample_text = "I am so happy to be here today! This is an amazing opportunity.\n\
                      The presentation went really well, I think.\n\
                      I'm a bit worried about the deadline though.";
    
    let text_path = "sample_text.txt";
    fs::write(text_path, sample_text)?;
    println!("✓ Created sample text file: {}", text_path);
    
    // Read and encode the file
    let file_data = fs::read(text_path)?;
    let file_input = FileInput {
        filename: text_path.to_string(),
        content_type: Some("text/plain".to_string()),
        data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &file_data),
        md5: None,
    };
    
    // Create batch job
    let job = em.batch()
        .create_job_from_files(models.clone(), vec![file_input], None, None, None)
        .await?;
    
    println!("✓ Created batch job: {}", job.job_id);
    match &job.state {
        StateInference::Queued { .. } => println!("  Status: Queued"),
        StateInference::InProgress { .. } => println!("  Status: In Progress"),
        StateInference::Completed { .. } => println!("  Status: Completed"),
        StateInference::Failed { .. } => println!("  Status: Failed"),
    }
    
    // Clean up sample file
    fs::remove_file(text_path)?;
    
    // Example 2: Process URLs
    println!("\nExample 2: Processing URLs");
    let urls = vec![
        "https://hume-tutorials.s3.amazonaws.com/faces.zip".to_string(),
    ];
    
    let url_job = em.batch()
        .create_job_from_urls(
            Models {
                face: Some(FaceModel {
                    identify_faces: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            urls,
            None,
            None,
            None
        )
        .await?;
    
    println!("✓ Created URL processing job: {}", url_job.job_id);
    
    // Example 3: Monitor job progress
    println!("\nExample 3: Monitoring job progress");
    println!("Waiting for job {} to complete...", job.job_id);
    
    let completed_job = match em.batch()
        .wait_for_job_completion(
            &job.job_id,
            std::time::Duration::from_secs(2),
            Some(std::time::Duration::from_secs(30)),
        )
        .await
    {
        Ok(job) => job,
        Err(e) => {
            println!("Job monitoring failed: {}", e);
            return Ok(());
        }
    };
    
    match &completed_job.state {
        StateInference::Completed { created_timestamp_ms, started_timestamp_ms, ended_timestamp_ms } => {
            println!("✓ Job completed successfully!");
            let duration_ms = ended_timestamp_ms - started_timestamp_ms;
            println!("  Processing time: {}ms", duration_ms);
            
            // Example 4: Retrieve predictions
            println!("\nExample 4: Retrieving predictions");
            match em.batch().get_predictions(&job.job_id, None).await {
                Ok(predictions) => {
                    println!("✓ Retrieved predictions");
                    
                    for (i, prediction) in predictions.predictions.iter().enumerate() {
                        println!("\nPrediction {} - Source: {:?}", i + 1, prediction.source.source_type);
                        
                        if let Some(language) = &prediction.results.language {
                            println!("  Language analysis:");
                            for group in &language.grouped_predictions {
                                println!("    Text: \"{}\"", group.text);
                                for pred in &group.predictions {
                                    if let Some(sentiment) = &pred.sentiment {
                                        println!("      Sentiment - Pos: {:.1}%, Neg: {:.1}%, Neu: {:.1}%",
                                            sentiment.positive * 100.0,
                                            sentiment.negative * 100.0,
                                            sentiment.neutral * 100.0
                                        );
                                    }
                                }
                            }
                        }
                        
                        if let Some(face) = &prediction.results.face {
                            println!("  Face analysis:");
                            println!("    Found {} face group(s)", face.grouped_predictions.len());
                        }
                    }
                    
                    if !predictions.errors.is_empty() {
                        println!("\n  Errors encountered:");
                        for error in &predictions.errors {
                            println!("    - {}", error.message);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to retrieve predictions: {}", e);
                }
            }
            
            // Example 5: Get artifacts (if any)
            println!("\nExample 5: Checking for artifacts");
            match em.batch().get_artifacts(&job.job_id, None).await {
                Ok(artifacts) => {
                    if artifacts.artifacts.is_empty() {
                        println!("  No artifacts generated");
                    } else {
                        println!("✓ Found artifacts:");
                        for (artifact_type, urls) in &artifacts.artifacts {
                            println!("  {}: {} file(s)", artifact_type, urls.len());
                            for url in urls.iter().take(2) {
                                println!("    - {}", url);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to retrieve artifacts: {}", e);
                }
            }
        }
        StateInference::Failed { created_timestamp_ms, started_timestamp_ms, ended_timestamp_ms, message } => {
            println!("✗ Job failed: {}", message);
            if let Some(start_ms) = started_timestamp_ms {
                let duration_ms = ended_timestamp_ms - start_ms;
                println!("  Failed after: {}ms", duration_ms);
            }
        }
        _ => {
            println!("Job ended in unexpected state");
        }
    }
    
    // Example 6: List recent jobs
    println!("\nExample 6: Listing recent jobs");
    let recent_jobs = em.batch().list_jobs(Some(5), None, None).await?;
    
    println!("Recent jobs ({} total):", recent_jobs.total);
    for job in recent_jobs.jobs.iter().take(5) {
        let status = match &job.state {
            StateInference::Queued { .. } => "Queued",
            StateInference::InProgress { .. } => "In Progress",
            StateInference::Completed { .. } => "Completed",
            StateInference::Failed { .. } => "Failed",
        };
        
        println!("  - {} ({}): {} model(s), {} source(s)", 
            job.job_id,
            status,
            count_models(&job.request.models),
            job.request.sources.len()
        );
    }
    
    Ok(())
}

fn count_models(models: &Models) -> usize {
    let mut count = 0;
    if models.face.is_some() { count += 1; }
    if models.language.is_some() { count += 1; }
    if models.prosody.is_some() { count += 1; }
    if models.burst.is_some() { count += 1; }
    if models.ner.is_some() { count += 1; }
    count
}
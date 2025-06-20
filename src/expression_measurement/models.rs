//! Data models for Expression Measurement API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for which models to run
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Models {
    /// Face expression model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceModel>,
    
    /// Language emotion model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageModel>,
    
    /// Speech prosody model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prosody: Option<ProsodyModel>,
    
    /// Vocal burst model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst: Option<BurstModel>,
    
    /// Named entity recognition model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ner: Option<NerModel>,
}

/// Face expression model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FaceModel {
    /// Whether to identify faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_faces: Option<bool>,
    
    /// Minimum face size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_face_size: Option<u32>,
    
    /// FPS for video processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_pred: Option<f32>,
    
    /// Probability threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prob_threshold: Option<f32>,
}

/// Language emotion model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LanguageModel {
    /// Sentiment analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<SentimentConfig>,
    
    /// Toxicity detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity: Option<ToxicityConfig>,
    
    /// Granularity level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

/// Sentiment configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SentimentConfig {}

/// Toxicity configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToxicityConfig {}

/// Speech prosody model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProsodyModel {
    /// Granularity level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    
    /// Window configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowConfig>,
}

/// Window configuration for prosody
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window length in seconds
    pub length: f32,
    
    /// Step size in seconds
    pub step: f32,
}

/// Vocal burst model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BurstModel {}

/// Named entity recognition model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NerModel {}

/// Input source for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Source {
    /// File source
    File {
        /// File to process
        #[serde(flatten)]
        file: FileInput,
    },
    /// URL source
    Url {
        /// URL to fetch
        url: String,
    },
    /// Text source
    Text {
        /// Text content
        text: String,
    },
}

/// File input for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInput {
    /// File content type
    pub content_type: Option<String>,
    
    /// File name
    pub filename: String,
    
    /// File data (base64 encoded for JSON)
    pub data: String,
    
    /// MD5 hash
    pub md5: Option<String>,
}

/// Batch job request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJobRequest {
    /// Models to run
    pub models: Models,
    
    /// Input sources
    pub sources: Vec<Source>,
    
    /// Callback URL for notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    
    /// Whether to notify on completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
}

/// Job ID response from create endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobId {
    /// Job ID
    pub job_id: String,
}

/// Batch job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    /// Job is queued
    Queued,
    /// Job is in progress
    InProgress,
    /// Job completed successfully
    Completed,
    /// Job failed
    Failed,
}

/// State information for inference job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StateInference {
    /// Job is queued
    Queued {
        /// Creation timestamp
        created_timestamp_ms: i64,
    },
    /// Job is in progress
    InProgress {
        /// Creation timestamp
        created_timestamp_ms: i64,
        /// Start timestamp
        started_timestamp_ms: i64,
    },
    /// Job completed successfully
    Completed {
        /// Creation timestamp
        created_timestamp_ms: i64,
        /// Start timestamp
        started_timestamp_ms: i64,
        /// End timestamp
        ended_timestamp_ms: i64,
    },
    /// Job failed
    Failed {
        /// Creation timestamp
        created_timestamp_ms: i64,
        /// Start timestamp
        started_timestamp_ms: Option<i64>,
        /// End timestamp
        ended_timestamp_ms: i64,
        /// Error message
        message: String,
    },
}

/// Batch job details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    /// Job ID
    pub job_id: String,
    
    /// Job type
    #[serde(rename = "type")]
    pub job_type: String,
    
    /// Request details
    pub request: BatchJobRequest,
    
    /// State information
    pub state: StateInference,
    
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Job state information (for other job types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobState {
    /// Created jobs
    pub created_jobs: Vec<String>,
    
    /// In-progress jobs
    pub in_progress_jobs: Vec<String>,
    
    /// Completed jobs
    pub completed_jobs: Vec<String>,
    
    /// Failed jobs
    pub failed_jobs: Vec<String>,
}

/// List jobs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// List of jobs
    pub jobs: Vec<BatchJob>,
    
    /// Total count
    pub total: u64,
}

/// Prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResults {
    /// Predictions by source
    pub predictions: Vec<SourcePrediction>,
    
    /// Errors
    pub errors: Vec<PredictionError>,
}

/// Prediction for a single source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePrediction {
    /// Source information
    pub source: SourceInfo,
    
    /// Results by model
    pub results: ModelResults,
}

/// Source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Source type
    #[serde(rename = "type")]
    pub source_type: String,
    
    /// Filename if applicable
    pub filename: Option<String>,
    
    /// URL if applicable
    pub url: Option<String>,
    
    /// Content type
    pub content_type: Option<String>,
}

/// Results from all models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResults {
    /// Face predictions
    pub face: Option<FacePredictions>,
    
    /// Language predictions
    pub language: Option<LanguagePredictions>,
    
    /// Prosody predictions
    pub prosody: Option<ProsodyPredictions>,
    
    /// Burst predictions
    pub burst: Option<BurstPredictions>,
    
    /// NER predictions
    pub ner: Option<NerPredictions>,
}

/// Face prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacePredictions {
    /// Grouped predictions
    pub grouped_predictions: Vec<FaceGroupPrediction>,
}

/// Face group prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceGroupPrediction {
    /// Face ID
    pub id: String,
    
    /// Predictions
    pub predictions: Vec<FacePrediction>,
}

/// Single face prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacePrediction {
    /// Frame number
    pub frame: u32,
    
    /// Time in milliseconds
    pub time_ms: u32,
    
    /// Bounding box
    pub bbox: BoundingBox,
    
    /// Emotion scores
    pub emotions: HashMap<String, EmotionScore>,
}

/// Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Width
    pub w: f32,
    /// Height
    pub h: f32,
}

/// Emotion score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionScore {
    /// Emotion name
    pub name: String,
    /// Score value
    pub score: f32,
}

/// Language prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePredictions {
    /// Grouped predictions
    pub grouped_predictions: Vec<LanguageGroupPrediction>,
}

/// Language group prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageGroupPrediction {
    /// Text
    pub text: String,
    
    /// Predictions
    pub predictions: Vec<LanguagePrediction>,
}

/// Single language prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePrediction {
    /// Emotion scores
    pub emotions: HashMap<String, EmotionScore>,
    
    /// Sentiment scores
    pub sentiment: Option<SentimentScore>,
    
    /// Toxicity scores
    pub toxicity: Option<ToxicityScore>,
}

/// Sentiment score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentScore {
    /// Positive sentiment
    pub positive: f32,
    /// Negative sentiment
    pub negative: f32,
    /// Neutral sentiment
    pub neutral: f32,
}

/// Toxicity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicityScore {
    /// Overall toxicity
    pub toxic: f32,
    /// Severe toxicity
    pub severe_toxic: f32,
    /// Obscene
    pub obscene: f32,
    /// Threat
    pub threat: f32,
    /// Insult
    pub insult: f32,
    /// Identity hate
    pub identity_hate: f32,
}

/// Prosody prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsodyPredictions {
    /// Grouped predictions
    pub grouped_predictions: Vec<ProsodyGroupPrediction>,
}

/// Prosody group prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsodyGroupPrediction {
    /// Predictions
    pub predictions: Vec<ProsodyPrediction>,
}

/// Single prosody prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsodyPrediction {
    /// Time range
    pub time: TimeRange,
    
    /// Emotion scores
    pub emotions: HashMap<String, EmotionScore>,
}

/// Time range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time in milliseconds
    pub start_ms: u32,
    /// End time in milliseconds
    pub end_ms: u32,
}

/// Burst prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstPredictions {
    /// Grouped predictions
    pub grouped_predictions: Vec<BurstGroupPrediction>,
}

/// Burst group prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstGroupPrediction {
    /// Predictions
    pub predictions: Vec<BurstPrediction>,
}

/// Single burst prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstPrediction {
    /// Time range
    pub time: TimeRange,
    
    /// Burst type scores
    pub bursts: HashMap<String, f32>,
}

/// NER prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerPredictions {
    /// Grouped predictions
    pub grouped_predictions: Vec<NerGroupPrediction>,
}

/// NER group prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerGroupPrediction {
    /// Text
    pub text: String,
    
    /// Predictions
    pub predictions: Vec<NerPrediction>,
}

/// Single NER prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerPrediction {
    /// Entity text
    pub entity: String,
    
    /// Entity type
    pub entity_type: String,
    
    /// Position
    pub position: Position,
    
    /// Emotion scores
    pub emotions: HashMap<String, EmotionScore>,
}

/// Text position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Start offset
    pub start: usize,
    /// End offset
    pub end: usize,
}

/// Prediction error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    /// Error message
    pub message: String,
    
    /// Error code
    pub code: Option<String>,
    
    /// File that caused the error
    pub file: Option<String>,
}

/// Job artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobArtifacts {
    /// Artifact URLs by type
    pub artifacts: HashMap<String, Vec<String>>,
}
//! Tests for Expression Measurement functionality

use hume::{HumeClient, ExpressionMeasurementClient};
use hume::expression_measurement::models::*;

#[test]
fn test_expression_measurement_client_creation() {
    let client = HumeClient::new("test-key").unwrap();
    let em_client = ExpressionMeasurementClient::from(client);
    
    // Test that we can create batch and stream clients
    let _batch = em_client.batch();
    let _stream = em_client.stream();
}

#[test]
fn test_models_creation() {
    let models = Models {
        face: Some(FaceModel {
            identify_faces: Some(true),
            min_face_size: Some(60),
            fps_pred: Some(30.0),
            prob_threshold: Some(0.5),
        }),
        language: Some(LanguageModel {
            sentiment: Some(SentimentConfig {}),
            toxicity: Some(ToxicityConfig {}),
            granularity: Some("word".to_string()),
        }),
        prosody: Some(ProsodyModel {
            granularity: Some("utterance".to_string()),
            window: Some(WindowConfig {
                length: 2.0,
                step: 0.5,
            }),
        }),
        burst: Some(BurstModel {}),
        ner: Some(NerModel {}),
    };
    
    assert!(models.face.is_some());
    assert!(models.language.is_some());
    assert!(models.prosody.is_some());
}

#[test]
fn test_source_variants() {
    let file_source = Source::File {
        file: FileInput {
            content_type: Some("image/jpeg".to_string()),
            filename: "test.jpg".to_string(),
            data: "base64data".to_string(),
            md5: Some("hash".to_string()),
        },
    };
    
    let url_source = Source::Url {
        url: "https://example.com/video.mp4".to_string(),
    };
    
    let text_source = Source::Text {
        text: "Analyze this text".to_string(),
    };
    
    // Serialize to test tag behavior
    let file_json = serde_json::to_string(&file_source).unwrap();
    assert!(file_json.contains(r#""type":"file""#));
    
    let url_json = serde_json::to_string(&url_source).unwrap();
    assert!(url_json.contains(r#""type":"url""#));
    
    let text_json = serde_json::to_string(&text_source).unwrap();
    assert!(text_json.contains(r#""type":"text""#));
}

#[test]
fn test_batch_job_request() {
    let request = BatchJobRequest {
        models: Models::default(),
        sources: vec![
            Source::Text {
                text: "Test text".to_string(),
            },
        ],
        callback_url: Some("https://example.com/callback".to_string()),
        notify: Some(true),
    };
    
    assert_eq!(request.sources.len(), 1);
    assert_eq!(request.callback_url, Some("https://example.com/callback".to_string()));
    assert_eq!(request.notify, Some(true));
}

#[test]
fn test_job_status_serialization() {
    assert_eq!(serde_json::to_string(&JobStatus::Queued).unwrap(), r#""QUEUED""#);
    assert_eq!(serde_json::to_string(&JobStatus::InProgress).unwrap(), r#""IN_PROGRESS""#);
    assert_eq!(serde_json::to_string(&JobStatus::Completed).unwrap(), r#""COMPLETED""#);
    assert_eq!(serde_json::to_string(&JobStatus::Failed).unwrap(), r#""FAILED""#);
}

#[test]
fn test_stream_builder() {
    use hume::expression_measurement::stream::StreamBuilder;
    
    let models = StreamBuilder::new()
        .with_face(FaceModel {
            identify_faces: Some(true),
            ..Default::default()
        })
        .with_language(LanguageModel::default())
        .with_prosody(ProsodyModel::default())
        .build();
    
    assert!(models.face.is_some());
    assert!(models.language.is_some());
    assert!(models.prosody.is_some());
    assert!(models.burst.is_none());
    assert!(models.ner.is_none());
}

#[test]
fn test_emotion_score() {
    let score = EmotionScore {
        name: "joy".to_string(),
        score: 0.85,
    };
    
    assert_eq!(score.name, "joy");
    assert_eq!(score.score, 0.85);
}

#[test]
fn test_bounding_box() {
    let bbox = BoundingBox {
        x: 100.0,
        y: 200.0,
        w: 50.0,
        h: 75.0,
    };
    
    assert_eq!(bbox.x, 100.0);
    assert_eq!(bbox.y, 200.0);
    assert_eq!(bbox.w, 50.0);
    assert_eq!(bbox.h, 75.0);
}
//! Unit tests for Expression Measurement models and functionality

#[cfg(test)]
mod tests {
    use hume::expression::models::*;
    use hume::core::error::Error;
    use chrono::Utc;

    #[test]
    fn test_face_prediction() {
        let prediction = FacePrediction {
            box_: BoundingBox {
                x: 10.0,
                y: 20.0,
                w: 100.0,
                h: 150.0,
            },
            emotions: vec![
                EmotionScore {
                    name: "joy".to_string(),
                    score: 0.85,
                },
                EmotionScore {
                    name: "sadness".to_string(),
                    score: 0.15,
                },
            ],
            descriptions: None,
            facs: None,
        };

        assert_eq!(prediction.box_.x, 10.0);
        assert_eq!(prediction.emotions.len(), 2);
        assert_eq!(prediction.emotions[0].name, "joy");
        assert_eq!(prediction.emotions[0].score, 0.85);
    }

    #[test]
    fn test_prosody_prediction() {
        let prediction = ProsodyPrediction {
            emotions: vec![
                EmotionScore {
                    name: "excitement".to_string(),
                    score: 0.75,
                },
            ],
            confidence: Some(0.9),
            speaker_confidence: Some(0.85),
        };

        assert_eq!(prediction.emotions.len(), 1);
        assert_eq!(prediction.confidence, Some(0.9));
        assert_eq!(prediction.speaker_confidence, Some(0.85));
    }

    #[test]
    fn test_language_prediction() {
        let prediction = LanguagePrediction {
            emotions: vec![
                EmotionScore {
                    name: "anger".to_string(),
                    score: 0.3,
                },
                EmotionScore {
                    name: "fear".to_string(),
                    score: 0.7,
                },
            ],
            sentiment: Some(SentimentScore {
                positive: 0.2,
                negative: 0.8,
            }),
            toxicity: None,
        };

        assert_eq!(prediction.emotions.len(), 2);
        assert!(prediction.sentiment.is_some());
        
        let sentiment = prediction.sentiment.unwrap();
        assert_eq!(sentiment.positive, 0.2);
        assert_eq!(sentiment.negative, 0.8);
    }

    #[test]
    fn test_source_location() {
        // Test file source
        let source = SourceLocation::File {
            filename: "test.jpg".to_string(),
        };
        
        if let SourceLocation::File { filename } = source {
            assert_eq!(filename, "test.jpg");
        } else {
            panic!("Expected file source");
        }

        // Test URL source
        let source = SourceLocation::Url {
            url: "https://example.com/image.jpg".to_string(),
        };
        
        if let SourceLocation::Url { url } = source {
            assert_eq!(url, "https://example.com/image.jpg");
        } else {
            panic!("Expected URL source");
        }
    }

    #[test]
    fn test_job_status_serialization() {
        use serde_json;

        let status = JobStatus::Pending;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""pending""#);

        let status = JobStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""running""#);

        let status = JobStatus::Succeeded;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""succeeded""#);

        let status = JobStatus::Failed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""failed""#);
    }

    #[test]
    fn test_batch_request_builder() {
        let request = BatchRequestBuilder::new()
            .add_text("Hello world", None)
            .unwrap()
            .add_text_with_language("Bonjour le monde", Some("fr".to_string()))
            .unwrap()
            .add_image_file("/path/to/image.jpg")
            .unwrap()
            .add_image_url("https://example.com/image.jpg")
            .unwrap()
            .add_audio_file("/path/to/audio.wav")
            .unwrap()
            .add_video_file("/path/to/video.mp4")
            .unwrap()
            .with_models(&[ModelType::Face, ModelType::Language, ModelType::Prosody])
            .with_transcription_config(TranscriptionConfig {
                language: Some("en".to_string()),
                identify_speakers: Some(true),
                confidence_threshold: None,
            })
            .build();

        assert_eq!(request.models.len(), 3);
        assert!(request.models.contains(&"face".to_string()));
        assert!(request.models.contains(&"language".to_string()));
        assert!(request.models.contains(&"prosody".to_string()));
        
        assert!(request.transcription.is_some());
        let transcription = request.transcription.unwrap();
        assert_eq!(transcription.language, Some("en".to_string()));
        assert_eq!(transcription.identify_speakers, Some(true));

        // Check media items
        assert_eq!(request.text.len(), 2);
        assert_eq!(request.text[0].text, "Hello world");
        assert_eq!(request.text[1].text, "Bonjour le monde");
        assert_eq!(request.text[1].language, Some("fr".to_string()));

        assert_eq!(request.files.len(), 3);
        assert_eq!(request.urls.len(), 1);
    }

    #[test]
    fn test_batch_request_empty_text_validation() {
        let result = BatchRequestBuilder::new()
            .add_text("", None);

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_batch_request_long_text_validation() {
        let long_text = "a".repeat(10001);
        let result = BatchRequestBuilder::new()
            .add_text(&long_text, None);

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => {
                assert!(msg.contains("must be <= 10000"));
                assert!(msg.contains("10001"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_stream_request_builder() {
        let request = StreamRequestBuilder::new()
            .add_file("/path/to/stream.mp4")
            .unwrap()
            .with_models(&[ModelType::Face, ModelType::Prosody])
            .with_stream_config(StreamConfig {
                window_ms: Some(500),
                step_ms: Some(250),
            })
            .build();

        assert_eq!(request.models.len(), 2);
        assert_eq!(request.files.len(), 1);
        
        assert!(request.stream_config.is_some());
        let config = request.stream_config.unwrap();
        assert_eq!(config.window_ms, Some(500));
        assert_eq!(config.step_ms, Some(250));
    }

    #[test]
    fn test_job_prediction_response() {
        let response = JobPredictionResponse {
            job_id: "job-123".to_string(),
            status: JobStatus::Succeeded,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            predictions: Some(vec![
                PredictionItem {
                    source: SourceLocation::File {
                        filename: "test.jpg".to_string(),
                    },
                    results: serde_json::json!({
                        "face": {
                            "predictions": []
                        }
                    }),
                    errors: vec![],
                },
            ]),
            errors: vec![],
        };

        assert_eq!(response.job_id, "job-123");
        assert_eq!(response.status, JobStatus::Succeeded);
        assert!(response.predictions.is_some());
        
        let predictions = response.predictions.unwrap();
        assert_eq!(predictions.len(), 1);
        
        if let SourceLocation::File { filename } = &predictions[0].source {
            assert_eq!(filename, "test.jpg");
        } else {
            panic!("Expected file source");
        }
    }

    #[test]
    fn test_emotion_score_validation() {
        // Valid scores
        let score = EmotionScore {
            name: "joy".to_string(),
            score: 0.5,
        };
        assert_eq!(score.score, 0.5);

        // Test edge cases
        let score = EmotionScore {
            name: "sadness".to_string(),
            score: 0.0,
        };
        assert_eq!(score.score, 0.0);

        let score = EmotionScore {
            name: "anger".to_string(),
            score: 1.0,
        };
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn test_model_type_to_string() {
        assert_eq!(ModelType::Face.as_str(), "face");
        assert_eq!(ModelType::Language.as_str(), "language");
        assert_eq!(ModelType::Prosody.as_str(), "prosody");
        assert_eq!(ModelType::Burst.as_str(), "burst");
        assert_eq!(ModelType::NER.as_str(), "ner");
        assert_eq!(ModelType::Facemesh.as_str(), "facemesh");
    }

    #[test]
    fn test_transcription_config() {
        let config = TranscriptionConfig {
            language: Some("en-US".to_string()),
            identify_speakers: Some(true),
            confidence_threshold: Some(0.8),
        };

        assert_eq!(config.language, Some("en-US".to_string()));
        assert_eq!(config.identify_speakers, Some(true));
        assert_eq!(config.confidence_threshold, Some(0.8));
    }

    #[test]
    fn test_error_item() {
        let error = ErrorItem {
            error: "File not found".to_string(),
            code: Some("FILE_NOT_FOUND".to_string()),
        };

        assert_eq!(error.error, "File not found");
        assert_eq!(error.code, Some("FILE_NOT_FOUND".to_string()));
    }

    #[test]
    fn test_grouped_predictions() {
        use std::collections::HashMap;

        let mut predictions = HashMap::new();
        predictions.insert(
            "face".to_string(),
            serde_json::json!({
                "predictions": [{
                    "box": {"x": 10, "y": 20, "w": 100, "h": 150},
                    "emotions": [{"name": "joy", "score": 0.9}]
                }]
            }),
        );

        let grouped = GroupedPredictions {
            predictions,
        };

        assert!(grouped.predictions.contains_key("face"));
        assert_eq!(grouped.predictions.len(), 1);
    }

    #[test]
    fn test_stream_config_validation() {
        // Valid configs
        let config = StreamConfig {
            window_ms: Some(1000),
            step_ms: Some(500),
        };
        assert_eq!(config.window_ms, Some(1000));
        assert_eq!(config.step_ms, Some(500));

        // Minimum values
        let config = StreamConfig {
            window_ms: Some(10),
            step_ms: Some(10),
        };
        assert_eq!(config.window_ms, Some(10));
        assert_eq!(config.step_ms, Some(10));
    }
}
//! Text-to-Speech API client and types

pub mod models;

use crate::core::{client::HumeClient, error::Result, request::RequestOptions};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use std::{pin::Pin, sync::Arc};

/// Client for the Text-to-Speech API
#[derive(Debug, Clone)]
pub struct TtsClient {
    client: Arc<HumeClient>,
}

impl TtsClient {
    /// Create a new TTS client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// Synthesize speech from text and return audio data
    pub async fn synthesize(
        &self,
        request: models::TtsRequest,
        options: Option<RequestOptions>,
    ) -> Result<models::TtsResponse> {
        self.client
            .http
            .post("/v0/tts", request, options)
            .await
    }

    /// Synthesize speech and return as raw audio bytes
    pub async fn synthesize_file(
        &self,
        request: models::TtsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Bytes> {
        self.client
            .http
            .request_bytes(
                reqwest::Method::POST,
                "/v0/tts/file",
                Some(request),
                options,
            )
            .await
    }

    /// Stream synthesis response as JSON chunks
    pub async fn stream_json(
        &self,
        request: models::TtsStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<models::TtsStreamResponse>> + Send>>> {
        let stream = self
            .client
            .http
            .request_stream(
                reqwest::Method::POST,
                "/v0/tts/stream/json",
                Some(request),
                options,
            )
            .await?;

        let mapped_stream = stream.map(|result| {
            result.and_then(|bytes| {
                serde_json::from_slice(&bytes).map_err(crate::core::error::Error::from)
            })
        });

        Ok(Box::pin(mapped_stream))
    }

    /// Stream synthesis response as raw audio chunks
    pub async fn stream_file(
        &self,
        request: models::TtsStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>> {
        self.client
            .http
            .request_stream(
                reqwest::Method::POST,
                "/v0/tts/stream/file",
                Some(request),
                options,
            )
            .await
    }

    /// List available voices
    pub async fn list_voices(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<models::VoicesResponse> {
        self.client.http.get("/v0/tts/voices", options).await
    }

    /// Convenience method to synthesize with default settings
    pub async fn synthesize_simple(
        &self,
        text: impl Into<String>,
        voice_name: Option<impl Into<String>>,
    ) -> Result<Bytes> {
        let request = models::TtsRequest {
            utterances: vec![models::Utterance {
                text: text.into(),
                voice: voice_name.map(|v| models::VoiceSpec::Name {
                    name: v.into(),
                    provider: None,
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        self.synthesize_file(request, None).await
    }
}

impl From<HumeClient> for TtsClient {
    fn from(client: HumeClient) -> Self {
        Self::new(Arc::new(client))
    }
}

impl From<Arc<HumeClient>> for TtsClient {
    fn from(client: Arc<HumeClient>) -> Self {
        Self::new(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HumeClientBuilder;

    #[test]
    fn test_tts_client_creation() {
        let client = HumeClientBuilder::new("test-key")
            .build()
            .expect("Failed to build client");
        
        let tts_client = TtsClient::new(Arc::new(client));
        assert!(!tts_client.client.base_url().is_empty());
    }
}
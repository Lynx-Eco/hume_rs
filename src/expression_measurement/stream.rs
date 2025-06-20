//! Streaming client for Expression Measurement API

use crate::{
    core::{client::HumeClient, error::Result},
    expression_measurement::models::*,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

/// Client for streaming expression measurement
#[derive(Debug, Clone)]
pub struct StreamClient {
    client: Arc<HumeClient>,
}

impl StreamClient {
    /// Create a new stream client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// Connect to the streaming WebSocket
    pub async fn connect(&self, models: Models) -> Result<StreamSocket> {
        let auth = self
            .client
            .http
            .auth
            .as_ref()
            .ok_or_else(|| crate::core::error::Error::auth("No authentication configured"))?;

        let (param_name, param_value) = auth.query_param();
        let ws_url = format!(
            "{}/v0/stream/models?{}={}",
            self.client.base_url().replace("https://", "wss://"),
            param_name,
            param_value
        );

        let (ws_stream, _) = connect_async(&ws_url).await?;

        Ok(StreamSocket::new(ws_stream, models))
    }
}

/// WebSocket connection for streaming
pub struct StreamSocket {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    models: Models,
}

impl StreamSocket {
    /// Create a new stream socket
    fn new(ws: WebSocketStream<MaybeTlsStream<TcpStream>>, models: Models) -> Self {
        Self { ws, models }
    }

    /// Send the initial configuration
    pub async fn send_config(&mut self) -> Result<()> {
        let config = StreamConfig {
            models: self.models.clone(),
            stream_window_ms: None,
        };

        let message = serde_json::to_string(&config)?;
        self.ws.send(Message::Text(message)).await?;
        Ok(())
    }

    /// Send data for processing
    pub async fn send_data(&mut self, data: StreamData) -> Result<()> {
        let message = serde_json::to_string(&data)?;
        self.ws.send(Message::Text(message)).await?;
        Ok(())
    }

    /// Send text for processing
    pub async fn send_text(&mut self, text: String) -> Result<()> {
        self.send_data(StreamData::Text { text }).await
    }

    /// Send audio data for processing
    pub async fn send_audio(&mut self, data: Vec<u8>) -> Result<()> {
        use base64::Engine;
        self.send_data(StreamData::Audio {
            data: base64::engine::general_purpose::STANDARD.encode(&data),
        })
        .await
    }

    /// Send video frame for processing
    pub async fn send_video_frame(&mut self, data: Vec<u8>) -> Result<()> {
        use base64::Engine;
        self.send_data(StreamData::VideoFrame {
            data: base64::engine::general_purpose::STANDARD.encode(&data),
        })
        .await
    }

    /// Receive the next message
    pub async fn receive(&mut self) -> Result<Option<StreamMessage>> {
        match self.ws.next().await {
            Some(Ok(Message::Text(text))) => {
                let message = serde_json::from_str(&text)?;
                Ok(Some(message))
            }
            Some(Ok(Message::Close(_))) => Ok(None),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
            _ => Ok(Some(StreamMessage::Unknown)),
        }
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        self.ws.close(None).await?;
        Ok(())
    }
}

/// Stream configuration
#[derive(Debug, Clone, Serialize)]
struct StreamConfig {
    /// Models to run
    models: Models,
    /// Stream window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_window_ms: Option<u32>,
}

/// Data to send for streaming
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamData {
    /// Text data
    Text {
        /// Text content
        text: String,
    },
    /// Audio data
    Audio {
        /// Base64 encoded audio
        data: String,
    },
    /// Video frame
    VideoFrame {
        /// Base64 encoded frame
        data: String,
    },
}

/// Messages received from the stream
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamMessage {
    /// Job details
    JobDetails {
        /// Job ID
        job_id: String,
    },
    /// Predictions
    Predictions {
        /// Prediction results
        predictions: StreamPredictions,
    },
    /// Error
    Error {
        /// Error message
        message: String,
        /// Error code
        code: Option<String>,
    },
    /// Warning
    Warning {
        /// Warning message
        message: String,
    },
    /// Unknown message type
    #[serde(other)]
    Unknown,
}

/// Streaming predictions
#[derive(Debug, Clone, Deserialize)]
pub struct StreamPredictions {
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

/// Builder for streaming connections
pub struct StreamBuilder {
    models: Models,
}

impl StreamBuilder {
    /// Create a new stream builder
    pub fn new() -> Self {
        Self {
            models: Models::default(),
        }
    }

    /// Enable face model
    pub fn with_face(mut self, config: FaceModel) -> Self {
        self.models.face = Some(config);
        self
    }

    /// Enable language model
    pub fn with_language(mut self, config: LanguageModel) -> Self {
        self.models.language = Some(config);
        self
    }

    /// Enable prosody model
    pub fn with_prosody(mut self, config: ProsodyModel) -> Self {
        self.models.prosody = Some(config);
        self
    }

    /// Enable burst model
    pub fn with_burst(mut self, config: BurstModel) -> Self {
        self.models.burst = Some(config);
        self
    }

    /// Enable NER model
    pub fn with_ner(mut self, config: NerModel) -> Self {
        self.models.ner = Some(config);
        self
    }

    /// Build the models configuration
    pub fn build(self) -> Models {
        self.models
    }
}

impl Default for StreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}
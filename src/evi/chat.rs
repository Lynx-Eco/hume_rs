//! WebSocket chat client for EVI

use crate::{
    core::{client::HumeClient, error::Result},
    evi::models::*,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

/// Client for EVI chat functionality
#[derive(Debug, Clone)]
pub struct ChatClient {
    client: Arc<HumeClient>,
}

impl ChatClient {
    /// Create a new chat client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// Connect to a chat session
    pub async fn connect(
        &self,
        config_id: Option<String>,
        config_version: Option<u32>,
        resumed_chat_group_id: Option<String>,
    ) -> Result<ChatSocket> {
        let auth = self
            .client
            .http
            .auth
            .as_ref()
            .ok_or_else(|| crate::core::error::Error::auth("No authentication configured"))?;

        let (param_name, param_value) = auth.query_param();
        let mut ws_url = format!(
            "{}/v0/evi/chat?{}={}",
            self.client.base_url().replace("https://", "wss://"),
            param_name,
            param_value
        );

        // Add optional parameters
        if let Some(id) = &config_id {
            ws_url.push_str(&format!("&config_id={}", id));
        }
        if let Some(version) = config_version {
            ws_url.push_str(&format!("&config_version={}", version));
        }
        if let Some(group_id) = &resumed_chat_group_id {
            ws_url.push_str(&format!("&resumed_chat_group_id={}", group_id));
        }

        let (ws_stream, _) = connect_async(&ws_url).await?;

        Ok(ChatSocket::new(ws_stream))
    }

    /// List chat history
    pub async fn list_chats(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        ascending_order: Option<bool>,
    ) -> Result<ReturnPagedChats> {
        let mut req_options = crate::core::request::RequestOptions::new();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }
        if let Some(ascending) = ascending_order {
            req_options = req_options.with_query("ascending_order", ascending.to_string());
        }

        self.client
            .http
            .get("/v0/evi/chats", Some(req_options))
            .await
    }

    /// Get a specific chat
    pub async fn get_chat(&self, chat_id: &str) -> Result<Chat> {
        let path = format!("/v0/evi/chats/{}", chat_id);
        self.client.http.get(&path, None).await
    }

    /// List chat groups
    pub async fn list_chat_groups(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        ascending_order: Option<bool>,
    ) -> Result<ReturnPagedChatGroups> {
        let mut req_options = crate::core::request::RequestOptions::new();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }
        if let Some(ascending) = ascending_order {
            req_options = req_options.with_query("ascending_order", ascending.to_string());
        }

        self.client
            .http
            .get("/v0/evi/chat_groups", Some(req_options))
            .await
    }

    /// Get chat messages
    pub async fn get_chat_messages(
        &self,
        chat_id: &str,
        page_number: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PagedResponse<ChatMessage>> {
        let path = format!("/v0/evi/chats/{}/messages", chat_id);
        let mut req_options = crate::core::request::RequestOptions::new();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client.http.get(&path, Some(req_options)).await
    }
}

/// WebSocket connection for EVI chat
pub struct ChatSocket {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl ChatSocket {
    /// Create a new chat socket
    fn new(ws: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws }
    }

    /// Send session settings
    pub async fn send_session_settings(&mut self, settings: SessionSettings) -> Result<()> {
        let message = ClientMessage::SessionSettings { settings };
        self.send_message(message).await
    }

    /// Send audio input
    pub async fn send_audio(&mut self, data: Vec<u8>) -> Result<()> {
        use base64::Engine;
        let message = ClientMessage::AudioInput {
            data: base64::engine::general_purpose::STANDARD.encode(&data),
        };
        self.send_message(message).await
    }

    /// Send text input
    pub async fn send_text(&mut self, text: String) -> Result<()> {
        let message = ClientMessage::UserInput { text };
        self.send_message(message).await
    }

    /// Send assistant input
    pub async fn send_assistant_input(&mut self, text: String) -> Result<()> {
        let message = ClientMessage::AssistantInput { text };
        self.send_message(message).await
    }

    /// Send tool response
    pub async fn send_tool_response(
        &mut self,
        tool_call_id: String,
        content: String,
        tool_name: Option<String>,
    ) -> Result<()> {
        let message = ClientMessage::ToolResponse {
            tool_call_id,
            content,
            tool_name,
        };
        self.send_message(message).await
    }

    /// Send tool error
    pub async fn send_tool_error(
        &mut self,
        tool_call_id: String,
        error: String,
        code: Option<String>,
        tool_name: Option<String>,
    ) -> Result<()> {
        let message = ClientMessage::ToolError {
            tool_call_id,
            error,
            code,
            tool_name,
        };
        self.send_message(message).await
    }

    /// Pause the assistant
    pub async fn pause_assistant(&mut self) -> Result<()> {
        let message = ClientMessage::PauseAssistant {};
        self.send_message(message).await
    }

    /// Resume the assistant
    pub async fn resume_assistant(&mut self) -> Result<()> {
        let message = ClientMessage::ResumeAssistant {};
        self.send_message(message).await
    }

    /// Send a message
    async fn send_message(&mut self, message: ClientMessage) -> Result<()> {
        let json = serde_json::to_string(&message)?;
        self.ws.send(Message::Text(json)).await?;
        Ok(())
    }

    /// Receive the next message
    pub async fn receive(&mut self) -> Result<Option<ServerMessage>> {
        match self.ws.next().await {
            Some(Ok(Message::Text(text))) => {
                let message = serde_json::from_str(&text)?;
                Ok(Some(message))
            }
            Some(Ok(Message::Close(_))) => Ok(None),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
            _ => Ok(Some(ServerMessage::Unknown)),
        }
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        self.ws.close(None).await?;
        Ok(())
    }
}

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Session settings
    SessionSettings {
        /// Settings for the session
        settings: SessionSettings,
    },
    /// Audio input
    AudioInput {
        /// Base64 encoded audio data
        data: String,
    },
    /// User text input
    UserInput {
        /// Text from the user
        text: String,
    },
    /// Assistant input
    AssistantInput {
        /// Text to inject as assistant
        text: String,
    },
    /// Tool response
    ToolResponse {
        /// Tool call ID
        tool_call_id: String,
        /// Response content
        content: String,
        /// Tool name
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_name: Option<String>,
    },
    /// Tool error
    ToolError {
        /// Tool call ID
        tool_call_id: String,
        /// Error message
        error: String,
        /// Error code
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        /// Tool name
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_name: Option<String>,
    },
    /// Pause assistant
    PauseAssistant {},
    /// Resume assistant
    ResumeAssistant {},
}

/// Messages received from server
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Session started
    SessionStarted {
        /// Session ID
        session_id: String,
        /// Chat ID
        chat_id: String,
        /// Chat group ID
        chat_group_id: String,
        /// Config applied
        config: Config,
    },
    /// User message
    UserMessage {
        /// Message ID
        message_id: String,
        /// User text
        text: String,
    },
    /// Assistant message
    AssistantMessage {
        /// Message ID
        message_id: String,
        /// Assistant text
        text: String,
        /// Whether message is complete
        is_final: bool,
    },
    /// Audio output
    AudioOutput {
        /// Message ID
        message_id: String,
        /// Base64 encoded audio
        data: String,
        /// Audio index
        index: u32,
    },
    /// Tool call
    ToolCall {
        /// Tool call ID
        tool_call_id: String,
        /// Tool name
        name: String,
        /// Tool parameters
        parameters: serde_json::Value,
    },
    /// Tool response
    ToolResponse {
        /// Tool call ID
        tool_call_id: String,
        /// Response content
        content: String,
    },
    /// Tool error
    ToolError {
        /// Tool call ID
        tool_call_id: String,
        /// Error message
        error: String,
        /// Error code
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
    },
    /// Emotion inference
    EmotionInference {
        /// Inference results
        inference: EmotionInference,
    },
    /// Error
    Error {
        /// Error message
        message: String,
        /// Error code
        code: String,
        /// Error details
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// Warning
    Warning {
        /// Warning message
        message: String,
        /// Warning code
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
    },
    /// Session ended
    SessionEnded {
        /// Reason for ending
        reason: String,
        /// Additional info
        #[serde(skip_serializing_if = "Option::is_none")]
        info: Option<serde_json::Value>,
    },
    /// Unknown message type
    #[serde(other)]
    Unknown,
}

/// Builder for chat sessions
pub struct ChatSessionBuilder {
    config_id: Option<String>,
    config_version: Option<u32>,
    resumed_chat_group_id: Option<String>,
    session_settings: Option<SessionSettings>,
}

impl ChatSessionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config_id: None,
            config_version: None,
            resumed_chat_group_id: None,
            session_settings: None,
        }
    }

    /// Set the config ID
    pub fn config_id(mut self, id: impl Into<String>) -> Self {
        self.config_id = Some(id.into());
        self
    }

    /// Set the config version
    pub fn config_version(mut self, version: u32) -> Self {
        self.config_version = Some(version);
        self
    }

    /// Resume a chat group
    pub fn resume_chat_group(mut self, group_id: impl Into<String>) -> Self {
        self.resumed_chat_group_id = Some(group_id.into());
        self
    }

    /// Set session settings
    pub fn session_settings(mut self, settings: SessionSettings) -> Self {
        self.session_settings = Some(settings);
        self
    }

    /// Connect to the chat
    pub async fn connect(self, client: &ChatClient) -> Result<ChatSocket> {
        let mut socket = client
            .connect(self.config_id, self.config_version, self.resumed_chat_group_id)
            .await?;

        if let Some(settings) = self.session_settings {
            socket.send_session_settings(settings).await?;
        }

        Ok(socket)
    }
}

impl Default for ChatSessionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
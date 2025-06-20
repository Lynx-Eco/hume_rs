//! Empathic Voice Interface (EVI) API client and types

pub mod chat;
pub mod configs;
pub mod models;
pub mod prompts;
pub mod tools;
pub mod voices;

use crate::core::client::HumeClient;
use std::sync::Arc;

/// Client for the Empathic Voice Interface API
#[derive(Debug, Clone)]
pub struct EviClient {
    client: Arc<HumeClient>,
}

impl EviClient {
    /// Create a new EVI client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// Access chat functionality
    pub fn chat(&self) -> chat::ChatClient {
        chat::ChatClient::new(self.client.clone())
    }

    /// Access tools management
    pub fn tools(&self) -> tools::ToolsClient {
        tools::ToolsClient::new(self.client.clone())
    }

    /// Access prompts management
    pub fn prompts(&self) -> prompts::PromptsClient {
        prompts::PromptsClient::new(self.client.clone())
    }

    /// Access custom voices management
    pub fn voices(&self) -> voices::VoicesClient {
        voices::VoicesClient::new(self.client.clone())
    }

    /// Access configuration management
    pub fn configs(&self) -> configs::ConfigsClient {
        configs::ConfigsClient::new(self.client.clone())
    }
}

impl From<HumeClient> for EviClient {
    fn from(client: HumeClient) -> Self {
        Self::new(Arc::new(client))
    }
}

impl From<Arc<HumeClient>> for EviClient {
    fn from(client: Arc<HumeClient>) -> Self {
        Self::new(client)
    }
}
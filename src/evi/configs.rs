//! Configuration management client for EVI

use crate::{
    core::{client::HumeClient, error::Result, request::RequestOptions},
    evi::models::*,
};
use serde::Serialize;
use std::sync::Arc;

/// Client for managing EVI configurations
#[derive(Debug, Clone)]
pub struct ConfigsClient {
    client: Arc<HumeClient>,
}

impl ConfigsClient {
    /// Create a new configs client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// List all configurations
    pub async fn list(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedConfigs> {
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client
            .http
            .get("/v0/evi/configs", Some(req_options))
            .await
    }

    /// Create a new configuration
    pub async fn create(
        &self,
        request: CreateConfigRequest,
        options: Option<RequestOptions>,
    ) -> Result<Config> {
        self.client
            .http
            .post("/v0/evi/configs", request, options)
            .await
    }

    /// Get a specific configuration
    pub async fn get(&self, config_id: &str, options: Option<RequestOptions>) -> Result<Config> {
        let path = format!("/v0/evi/configs/{}", config_id);
        self.client.http.get(&path, options).await
    }

    /// Update a configuration
    pub async fn update(
        &self,
        config_id: &str,
        request: UpdateConfigRequest,
        options: Option<RequestOptions>,
    ) -> Result<Config> {
        let path = format!("/v0/evi/configs/{}", config_id);
        self.client.http.patch(&path, request, options).await
    }

    /// Delete a configuration
    pub async fn delete(&self, config_id: &str, options: Option<RequestOptions>) -> Result<()> {
        let path = format!("/v0/evi/configs/{}", config_id);
        let _: serde_json::Value = self.client.http.delete(&path, options).await?;
        Ok(())
    }

    /// List configuration versions
    pub async fn list_versions(
        &self,
        config_id: &str,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedConfigs> {
        let path = format!("/v0/evi/configs/{}/versions", config_id);
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client.http.get(&path, Some(req_options)).await
    }

    /// Get a specific configuration version
    pub async fn get_version(
        &self,
        config_id: &str,
        version: u32,
        options: Option<RequestOptions>,
    ) -> Result<Config> {
        let path = format!("/v0/evi/configs/{}/versions/{}", config_id, version);
        self.client.http.get(&path, options).await
    }
}

/// Request to create a new configuration
#[derive(Debug, Clone, Serialize)]
pub struct CreateConfigRequest {
    /// Configuration name
    pub name: String,
    
    /// Prompt specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptSpec>,
    
    /// Voice specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSpec>,
    
    /// Language model specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model: Option<LanguageModelSpec>,
    
    /// Tools specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolSpec>>,
    
    /// Event messages configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_messages: Option<EventMessagesSpec>,
    
    /// Timeouts configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<TimeoutsSpec>,
}

/// Request to update a configuration
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateConfigRequest {
    /// Updated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Updated prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptSpec>,
    
    /// Updated voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSpec>,
    
    /// Updated language model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model: Option<LanguageModelSpec>,
    
    /// Updated tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolSpec>>,
    
    /// Updated event messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_messages: Option<EventMessagesSpec>,
    
    /// Updated timeouts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<TimeoutsSpec>,
}

impl CreateConfigRequest {
    /// Create a new config request builder
    pub fn builder(name: impl Into<String>) -> CreateConfigRequestBuilder {
        CreateConfigRequestBuilder::new(name)
    }
}

/// Builder for creating config requests
#[derive(Debug)]
pub struct CreateConfigRequestBuilder {
    request: CreateConfigRequest,
}

impl CreateConfigRequestBuilder {
    /// Create a new builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            request: CreateConfigRequest {
                name: name.into(),
                prompt: None,
                voice: None,
                language_model: None,
                tools: None,
                event_messages: None,
                timeouts: None,
            },
        }
    }

    /// Set the prompt
    pub fn prompt(mut self, prompt_id: impl Into<String>, version: Option<u32>) -> Self {
        self.request.prompt = Some(PromptSpec {
            id: prompt_id.into(),
            version,
        });
        self
    }

    /// Set the voice
    pub fn voice(mut self, voice_id: impl Into<String>) -> Self {
        self.request.voice = Some(VoiceSpec {
            id: voice_id.into(),
        });
        self
    }

    /// Set the language model
    pub fn language_model(
        mut self,
        provider: impl Into<String>,
        resource: impl Into<String>,
        temperature: Option<f32>,
    ) -> Self {
        self.request.language_model = Some(LanguageModelSpec {
            model_provider: provider.into(),
            model_resource: resource.into(),
            temperature,
        });
        self
    }

    /// Add a tool
    pub fn add_tool(mut self, tool_id: impl Into<String>, version: Option<u32>) -> Self {
        let tools = self.request.tools.get_or_insert_with(Vec::new);
        tools.push(ToolSpec {
            id: tool_id.into(),
            version,
        });
        self
    }

    /// Set event messages
    pub fn event_messages(mut self, messages: EventMessagesSpec) -> Self {
        self.request.event_messages = Some(messages);
        self
    }

    /// Set timeouts
    pub fn timeouts(mut self, inactivity: Option<u32>, max_duration: Option<u32>) -> Self {
        self.request.timeouts = Some(TimeoutsSpec {
            inactivity,
            max_duration,
        });
        self
    }

    /// Build the request
    pub fn build(self) -> CreateConfigRequest {
        self.request
    }
}
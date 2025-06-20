//! Custom voices management client for EVI

use crate::{
    core::{client::HumeClient, error::Result, request::RequestOptions},
    evi::models::{CustomVoice, ReturnPagedCustomVoices, VoiceParameters},
};
use serde::Serialize;
use std::sync::Arc;

/// Client for managing EVI custom voices
#[derive(Debug, Clone)]
pub struct VoicesClient {
    client: Arc<HumeClient>,
}

impl VoicesClient {
    /// Create a new voices client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// List all custom voices
    pub async fn list(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedCustomVoices> {
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client
            .http
            .get("/v0/evi/custom_voices", Some(req_options))
            .await
    }

    /// Create a new custom voice
    pub async fn create(
        &self,
        request: CreateCustomVoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomVoice> {
        self.client
            .http
            .post("/v0/evi/custom_voices", request, options)
            .await
    }

    /// Get a specific custom voice
    pub async fn get(&self, voice_id: &str, options: Option<RequestOptions>) -> Result<CustomVoice> {
        let path = format!("/v0/evi/custom_voices/{}", voice_id);
        self.client.http.get(&path, options).await
    }

    /// Update a custom voice
    pub async fn update(
        &self,
        voice_id: &str,
        request: UpdateCustomVoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomVoice> {
        let path = format!("/v0/evi/custom_voices/{}", voice_id);
        self.client.http.patch(&path, request, options).await
    }

    /// Delete a custom voice
    pub async fn delete(&self, voice_id: &str, options: Option<RequestOptions>) -> Result<()> {
        let path = format!("/v0/evi/custom_voices/{}", voice_id);
        let _: serde_json::Value = self.client.http.delete(&path, options).await?;
        Ok(())
    }
}

/// Request to create a new custom voice
#[derive(Debug, Clone, Serialize)]
pub struct CreateCustomVoiceRequest {
    /// Voice name
    pub name: String,
    
    /// Base voice ID to customize
    pub base_voice_id: String,
    
    /// Voice parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<VoiceParameters>,
}

/// Request to update a custom voice
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomVoiceRequest {
    /// Updated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Updated parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<VoiceParameters>,
}

impl CreateCustomVoiceRequest {
    /// Create a new custom voice request builder
    pub fn builder(name: impl Into<String>, base_voice_id: impl Into<String>) -> CreateCustomVoiceRequestBuilder {
        CreateCustomVoiceRequestBuilder::new(name, base_voice_id)
    }
}

/// Builder for creating custom voice requests
pub struct CreateCustomVoiceRequestBuilder {
    request: CreateCustomVoiceRequest,
}

impl CreateCustomVoiceRequestBuilder {
    /// Create a new builder
    pub fn new(name: impl Into<String>, base_voice_id: impl Into<String>) -> Self {
        Self {
            request: CreateCustomVoiceRequest {
                name: name.into(),
                base_voice_id: base_voice_id.into(),
                parameters: None,
            },
        }
    }

    /// Set voice parameters
    pub fn parameters(mut self, params: VoiceParameters) -> Self {
        self.request.parameters = Some(params);
        self
    }

    /// Set pitch adjustment
    pub fn pitch(mut self, pitch: f32) -> Self {
        let params = self.request.parameters.get_or_insert(VoiceParameters {
            pitch: None,
            rate: None,
            volume: None,
        });
        params.pitch = Some(pitch);
        self
    }

    /// Set rate adjustment
    pub fn rate(mut self, rate: f32) -> Self {
        let params = self.request.parameters.get_or_insert(VoiceParameters {
            pitch: None,
            rate: None,
            volume: None,
        });
        params.rate = Some(rate);
        self
    }

    /// Set volume adjustment
    pub fn volume(mut self, volume: f32) -> Self {
        let params = self.request.parameters.get_or_insert(VoiceParameters {
            pitch: None,
            rate: None,
            volume: None,
        });
        params.volume = Some(volume);
        self
    }

    /// Build the request
    pub fn build(self) -> CreateCustomVoiceRequest {
        self.request
    }
}
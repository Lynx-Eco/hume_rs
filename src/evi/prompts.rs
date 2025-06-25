//! Prompts management client for EVI

use crate::{
    core::{client::HumeClient, error::Result, request::RequestOptions},
    evi::models::{ReturnPagedPrompts, Prompt},
};
use serde::Serialize;
use std::sync::Arc;

/// Client for managing EVI prompts
#[derive(Debug, Clone)]
pub struct PromptsClient {
    client: Arc<HumeClient>,
}

impl PromptsClient {
    /// Create a new prompts client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// List all prompts
    pub async fn list(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedPrompts> {
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client
            .http
            .get("/v0/evi/prompts", Some(req_options))
            .await
    }

    /// Create a new prompt
    pub async fn create(
        &self,
        request: CreatePromptRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt> {
        self.client
            .http
            .post("/v0/evi/prompts", request, options)
            .await
    }

    /// Get a specific prompt
    pub async fn get(&self, prompt_id: &str, options: Option<RequestOptions>) -> Result<Prompt> {
        let path = format!("/v0/evi/prompts/{}", prompt_id);
        self.client.http.get(&path, options).await
    }

    /// Update a prompt
    pub async fn update(
        &self,
        prompt_id: &str,
        request: UpdatePromptRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt> {
        let path = format!("/v0/evi/prompts/{}", prompt_id);
        self.client.http.patch(&path, request, options).await
    }

    /// Delete a prompt
    pub async fn delete(&self, prompt_id: &str, options: Option<RequestOptions>) -> Result<()> {
        let path = format!("/v0/evi/prompts/{}", prompt_id);
        let _: serde_json::Value = self.client.http.delete(&path, options).await?;
        Ok(())
    }

    /// List prompt versions
    pub async fn list_versions(
        &self,
        prompt_id: &str,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedPrompts> {
        let path = format!("/v0/evi/prompts/{}/versions", prompt_id);
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client.http.get(&path, Some(req_options)).await
    }

    /// Get a specific prompt version
    pub async fn get_version(
        &self,
        prompt_id: &str,
        version: u32,
        options: Option<RequestOptions>,
    ) -> Result<Prompt> {
        let path = format!("/v0/evi/prompts/{}/versions/{}", prompt_id, version);
        self.client.http.get(&path, options).await
    }

    /// Create a new version of a prompt
    pub async fn create_version(
        &self,
        prompt_id: &str,
        request: CreatePromptVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt> {
        let path = format!("/v0/evi/prompts/{}/versions", prompt_id);
        self.client.http.post(&path, request, options).await
    }
}

/// Request to create a new prompt
#[derive(Debug, Clone, Serialize)]
pub struct CreatePromptRequest {
    /// Prompt name
    pub name: String,
    
    /// Prompt text
    pub text: String,
    
    /// Version description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

/// Request to update a prompt
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdatePromptRequest {
    /// Updated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Updated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Request to create a new prompt version
#[derive(Debug, Clone, Serialize)]
pub struct CreatePromptVersionRequest {
    /// New prompt text
    pub text: String,
    
    /// Version description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

impl CreatePromptRequest {
    /// Create a new prompt request builder
    pub fn builder(name: impl Into<String>, text: impl Into<String>) -> CreatePromptRequestBuilder {
        CreatePromptRequestBuilder::new(name, text)
    }
}

/// Builder for creating prompt requests
#[derive(Debug)]
pub struct CreatePromptRequestBuilder {
    request: CreatePromptRequest,
}

impl CreatePromptRequestBuilder {
    /// Create a new builder
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            request: CreatePromptRequest {
                name: name.into(),
                text: text.into(),
                version_description: None,
            },
        }
    }

    /// Set the version description
    pub fn version_description(mut self, description: impl Into<String>) -> Self {
        self.request.version_description = Some(description.into());
        self
    }

    /// Build the request
    pub fn build(self) -> CreatePromptRequest {
        self.request
    }
}
//! Tools management client for EVI

use crate::{
    core::{client::HumeClient, error::Result, request::RequestOptions},
    evi::models::{ReturnPagedUserDefinedTools, Tool},
};
use serde::Serialize;
use std::sync::Arc;

/// Client for managing EVI tools
#[derive(Debug, Clone)]
pub struct ToolsClient {
    client: Arc<HumeClient>,
}

impl ToolsClient {
    /// Create a new tools client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// List all tools
    pub async fn list(
        &self,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedUserDefinedTools> {
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client
            .http
            .get("/v0/evi/tools", Some(req_options))
            .await
    }

    /// Create a new tool
    pub async fn create(
        &self,
        request: CreateToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<Tool> {
        self.client
            .http
            .post("/v0/evi/tools", request, options)
            .await
    }

    /// Get a specific tool
    pub async fn get(&self, tool_id: &str, options: Option<RequestOptions>) -> Result<Tool> {
        let path = format!("/v0/evi/tools/{}", tool_id);
        self.client.http.get(&path, options).await
    }

    /// Update a tool
    pub async fn update(
        &self,
        tool_id: &str,
        request: UpdateToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<Tool> {
        let path = format!("/v0/evi/tools/{}", tool_id);
        self.client.http.patch(&path, request, options).await
    }

    /// Delete a tool
    pub async fn delete(&self, tool_id: &str, options: Option<RequestOptions>) -> Result<()> {
        let path = format!("/v0/evi/tools/{}", tool_id);
        let _: serde_json::Value = self.client.http.delete(&path, options).await?;
        Ok(())
    }

    /// List tool versions
    pub async fn list_versions(
        &self,
        tool_id: &str,
        page_number: Option<u32>,
        page_size: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ReturnPagedUserDefinedTools> {
        let path = format!("/v0/evi/tools/{}/versions", tool_id);
        let mut req_options = options.unwrap_or_default();
        
        if let Some(page) = page_number {
            req_options = req_options.with_query("page_number", page.to_string());
        }
        
        if let Some(size) = page_size {
            req_options = req_options.with_query("page_size", size.to_string());
        }

        self.client.http.get(&path, Some(req_options)).await
    }

    /// Get a specific tool version
    pub async fn get_version(
        &self,
        tool_id: &str,
        version_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Tool> {
        let path = format!("/v0/evi/tools/{}/versions/{}", tool_id, version_id);
        self.client.http.get(&path, options).await
    }
}

/// Request to create a new tool
#[derive(Debug, Clone, Serialize)]
pub struct CreateToolRequest {
    /// Tool name
    pub name: String,
    
    /// Tool description
    pub description: String,
    
    /// Tool parameters schema (JSON Schema)
    pub parameters: serde_json::Value,
    
    /// Whether the tool is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Request to update a tool
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateToolRequest {
    /// Updated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Updated description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Updated parameters schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    
    /// Updated required status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CreateToolRequest {
    /// Create a new tool request builder
    pub fn builder(name: impl Into<String>, description: impl Into<String>) -> CreateToolRequestBuilder {
        CreateToolRequestBuilder::new(name, description)
    }
}

/// Builder for creating tool requests
pub struct CreateToolRequestBuilder {
    request: CreateToolRequest,
}

impl CreateToolRequestBuilder {
    /// Create a new builder
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            request: CreateToolRequest {
                name: name.into(),
                description: description.into(),
                parameters: serde_json::json!({}),
                required: None,
            },
        }
    }

    /// Set the parameters schema
    pub fn parameters(mut self, params: serde_json::Value) -> Self {
        self.request.parameters = params;
        self
    }

    /// Set whether the tool is required
    pub fn required(mut self, required: bool) -> Self {
        self.request.required = Some(required);
        self
    }

    /// Build the request
    pub fn build(self) -> CreateToolRequest {
        self.request
    }
}
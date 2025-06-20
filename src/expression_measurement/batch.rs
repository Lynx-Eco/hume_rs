//! Batch processing client for Expression Measurement API

use crate::{
    core::{client::HumeClient, error::Result, request::RequestOptions},
    expression_measurement::models::*,
};
use std::sync::Arc;

/// Client for batch expression measurement operations
#[derive(Debug, Clone)]
pub struct BatchClient {
    client: Arc<HumeClient>,
}

impl BatchClient {
    /// Create a new batch client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// List batch jobs
    pub async fn list_jobs(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        options: Option<RequestOptions>,
    ) -> Result<ListJobsResponse> {
        let mut req_options = options.unwrap_or_default();
        
        if let Some(limit) = limit {
            req_options = req_options.with_query("limit", limit.to_string());
        }
        
        if let Some(offset) = offset {
            req_options = req_options.with_query("offset", offset.to_string());
        }

        self.client
            .http
            .get("/v0/batch/jobs", Some(req_options))
            .await
    }

    /// Create a new batch job
    pub async fn create_job(
        &self,
        request: BatchJobRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob> {
        let job_id_response: JobId = self.client
            .http
            .post("/v0/batch/jobs", request, options.clone())
            .await?;
        
        // Fetch the full job details after creation
        self.get_job(&job_id_response.job_id, options).await
    }

    /// Get job details
    pub async fn get_job(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob> {
        let path = format!("/v0/batch/jobs/{}", job_id);
        self.client.http.get(&path, options).await
    }

    /// Get job predictions
    pub async fn get_predictions(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PredictionResults> {
        let path = format!("/v0/batch/jobs/{}/predictions", job_id);
        self.client.http.get(&path, options).await
    }

    /// Get job artifacts
    pub async fn get_artifacts(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<JobArtifacts> {
        let path = format!("/v0/batch/jobs/{}/artifacts", job_id);
        self.client.http.get(&path, options).await
    }

    /// Create a job from files
    pub async fn create_job_from_files(
        &self,
        models: Models,
        files: Vec<FileInput>,
        callback_url: Option<String>,
        notify: Option<bool>,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob> {
        let sources = files
            .into_iter()
            .map(|file| Source::File { file })
            .collect();

        let request = BatchJobRequest {
            models,
            sources,
            callback_url,
            notify,
        };

        self.create_job(request, options).await
    }

    /// Create a job from URLs
    pub async fn create_job_from_urls(
        &self,
        models: Models,
        urls: Vec<String>,
        callback_url: Option<String>,
        notify: Option<bool>,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob> {
        let sources = urls.into_iter().map(|url| Source::Url { url }).collect();

        let request = BatchJobRequest {
            models,
            sources,
            callback_url,
            notify,
        };

        self.create_job(request, options).await
    }

    /// Create a job from text
    pub async fn create_job_from_text(
        &self,
        models: Models,
        texts: Vec<String>,
        callback_url: Option<String>,
        notify: Option<bool>,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob> {
        let sources = texts
            .into_iter()
            .map(|text| Source::Text { text })
            .collect();

        let request = BatchJobRequest {
            models,
            sources,
            callback_url,
            notify,
        };

        self.create_job(request, options).await
    }

    /// Wait for a job to complete
    pub async fn wait_for_job_completion(
        &self,
        job_id: &str,
        poll_interval: std::time::Duration,
        max_wait: Option<std::time::Duration>,
    ) -> Result<BatchJob> {
        let start = std::time::Instant::now();

        loop {
            let job = self.get_job(job_id, None).await?;

            match &job.state {
                StateInference::Completed { .. } | StateInference::Failed { .. } => return Ok(job),
                _ => {
                    if let Some(max_wait) = max_wait {
                        if start.elapsed() > max_wait {
                            return Err(crate::core::error::Error::Timeout);
                        }
                    }

                    tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HumeClientBuilder;

    #[tokio::test]
    async fn test_batch_client_creation() {
        let client = HumeClientBuilder::new("test-key")
            .build()
            .expect("Failed to build client");
        
        let batch_client = BatchClient::new(Arc::new(client));
        assert!(!batch_client.client.base_url().is_empty());
    }
}
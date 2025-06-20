//! Expression Measurement API client and types

pub mod batch;
pub mod models;
pub mod stream;

use crate::core::client::HumeClient;
use std::sync::Arc;

/// Client for the Expression Measurement API
#[derive(Debug, Clone)]
pub struct ExpressionMeasurementClient {
    client: Arc<HumeClient>,
}

impl ExpressionMeasurementClient {
    /// Create a new Expression Measurement client
    pub fn new(client: Arc<HumeClient>) -> Self {
        Self { client }
    }

    /// Access batch processing functionality
    pub fn batch(&self) -> batch::BatchClient {
        batch::BatchClient::new(self.client.clone())
    }

    /// Access streaming functionality
    pub fn stream(&self) -> stream::StreamClient {
        stream::StreamClient::new(self.client.clone())
    }
}

impl From<HumeClient> for ExpressionMeasurementClient {
    fn from(client: HumeClient) -> Self {
        Self::new(Arc::new(client))
    }
}

impl From<Arc<HumeClient>> for ExpressionMeasurementClient {
    fn from(client: Arc<HumeClient>) -> Self {
        Self::new(client)
    }
}
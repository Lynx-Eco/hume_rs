//! Response handling utilities

use serde::{Deserialize, Serialize};

/// A paginated response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page
    pub data: Vec<T>,
    /// The total number of items
    pub total: Option<u64>,
    /// The number of items per page
    pub limit: Option<u64>,
    /// The offset of this page
    pub offset: Option<u64>,
    /// Whether there are more pages
    pub has_more: Option<bool>,
    /// Cursor for the next page
    pub next_cursor: Option<String>,
}

impl<T> PaginatedResponse<T> {
    /// Check if there are more pages
    pub fn has_next_page(&self) -> bool {
        self.has_more.unwrap_or(false) || self.next_cursor.is_some()
    }

    /// Get the next offset for pagination
    pub fn next_offset(&self) -> Option<u64> {
        if self.has_next_page() {
            if let (Some(offset), Some(limit)) = (self.offset, self.limit) {
                Some(offset + limit)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// A response that may contain a continuation token for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse<T> {
    /// The data in this response
    pub data: T,
    /// Continuation token for the next chunk
    pub continuation_token: Option<String>,
    /// Whether this is the final chunk
    pub is_final: Option<bool>,
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The response data
    pub data: T,
    /// Response metadata
    pub meta: Option<ResponseMetadata>,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Request ID for tracking
    pub request_id: Option<String>,
    /// Response timestamp
    pub timestamp: Option<String>,
    /// API version
    pub version: Option<String>,
}

/// Empty response for endpoints that return no data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

/// Response for operations that return an ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdResponse {
    /// The ID of the created/updated resource
    pub id: String,
}

/// Response for operations that return a status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// The status of the operation
    pub status: String,
    /// Optional message
    pub message: Option<String>,
}
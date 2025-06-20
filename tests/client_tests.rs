//! Tests for the main Hume client

use hume::{HumeClient, HumeClientBuilder};
use std::time::Duration;

#[test]
fn test_client_builder_with_api_key() {
    let client = HumeClientBuilder::new("test-api-key")
        .build()
        .expect("Failed to build client");
    
    assert_eq!(client.base_url(), "https://api.hume.ai");
}

#[test]
fn test_client_builder_with_custom_base_url() {
    let custom_url = "https://custom.hume.ai";
    let client = HumeClientBuilder::new("test-api-key")
        .base_url(custom_url)
        .build()
        .expect("Failed to build client");
    
    assert_eq!(client.base_url(), custom_url);
}

#[test]
fn test_client_builder_with_timeout() {
    let client = HumeClientBuilder::new("test-api-key")
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to build client");
    
    assert_eq!(client.base_url(), "https://api.hume.ai");
}

#[test]
fn test_client_builder_with_max_retries() {
    let client = HumeClientBuilder::new("test-api-key")
        .max_retries(5)
        .build()
        .expect("Failed to build client");
    
    assert_eq!(client.base_url(), "https://api.hume.ai");
}

#[test]
fn test_client_builder_requires_auth() {
    let result = HumeClientBuilder::default().build();
    assert!(result.is_err());
}

#[test]
fn test_client_new_shorthand() {
    let client = HumeClient::new("test-api-key")
        .expect("Failed to create client");
    
    assert_eq!(client.base_url(), "https://api.hume.ai");
}
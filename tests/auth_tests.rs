//! Tests for authentication

use chrono::Utc;
use hume::core::auth::{Auth, AuthToken};

#[test]
fn test_auth_api_key() {
    let auth = Auth::api_key("test-key");
    
    let (header_name, header_value) = auth.header_value().unwrap();
    assert_eq!(header_name, "X-Hume-Api-Key");
    assert_eq!(header_value, "test-key");
    
    let (param_name, param_value) = auth.query_param();
    assert_eq!(param_name, "apiKey");
    assert_eq!(param_value, "test-key");
    
    assert!(!auth.is_expired());
}

#[test]
fn test_auth_access_token() {
    let token = AuthToken::new(
        "test-token".to_string(),
        "Bearer".to_string(),
        3600,
    );
    
    let auth = Auth::access_token(token.clone());
    
    let (header_name, header_value) = auth.header_value().unwrap();
    assert_eq!(header_name, "Authorization");
    assert_eq!(header_value, "Bearer test-token");
    
    let (param_name, param_value) = auth.query_param();
    assert_eq!(param_name, "accessToken");
    assert_eq!(param_value, "test-token");
    
    assert!(!auth.is_expired());
}

#[test]
fn test_auth_token_expiry() {
    let mut token = AuthToken::new(
        "test-token".to_string(),
        "Bearer".to_string(),
        60, // 60 seconds
    );
    
    assert!(!token.is_expired());
    assert!(token.time_until_expiry().is_some());
    assert!(token.time_until_expiry().unwrap() <= 60);
    
    // Simulate expired token
    token.created_at = Utc::now() - chrono::Duration::seconds(120);
    assert!(token.is_expired());
    assert!(token.time_until_expiry().is_none());
}

#[test]
fn test_auth_token_from_response() {
    use hume::core::auth::AccessTokenResponse;
    
    let response = AccessTokenResponse {
        access_token: "new-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 7200,
    };
    
    let token: AuthToken = response.into();
    assert_eq!(token.access_token, "new-token");
    assert_eq!(token.token_type, "Bearer");
    assert_eq!(token.expires_in, 7200);
    assert!(!token.is_expired());
}
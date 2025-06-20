//! Tests for error handling

use hume::core::error::{Error, Result};

#[test]
fn test_error_creation() {
    let api_error = Error::api(404, "Not found".to_string(), None, None);
    assert!(api_error.is_api_error());
    assert_eq!(api_error.status_code(), Some(404));

    let auth_error = Error::auth("Invalid API key");
    assert!(!auth_error.is_api_error());

    let rate_limit_error = Error::RateLimit { retry_after: Some(60) };
    assert!(rate_limit_error.is_rate_limit());

    let timeout_error = Error::Timeout;
    assert!(timeout_error.is_timeout());
}

#[test]
fn test_error_display() {
    let error = Error::api(400, "Bad request".to_string(), Some("INVALID_INPUT".to_string()), None);
    let error_string = error.to_string();
    assert!(error_string.contains("400"));
    assert!(error_string.contains("Bad request"));
}

#[test]
fn test_result_type() {
    fn returns_result() -> Result<String> {
        Ok("success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_error_from_conversions() {
    let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
    let error: Error = json_err.into();
    assert!(matches!(error, Error::Json(_)));

    let url_err = url::Url::parse("not a url").unwrap_err();
    let error: Error = url_err.into();
    assert!(matches!(error, Error::UrlParse(_)));
}
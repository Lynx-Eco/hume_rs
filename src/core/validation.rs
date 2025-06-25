//! Input validation utilities for the Hume SDK

use crate::core::error::{Error, Result};

/// Maximum text length for TTS
pub const MAX_TTS_TEXT_LENGTH: usize = 5000;

/// Maximum text length for expression measurement
pub const MAX_EXPRESSION_TEXT_LENGTH: usize = 10000;

/// Valid speaking rate range
pub const MIN_SPEAKING_RATE: f32 = 0.5;
pub const MAX_SPEAKING_RATE: f32 = 2.0;

/// Valid pitch range
pub const MIN_PITCH: f32 = 0.5;
pub const MAX_PITCH: f32 = 2.0;

/// Valid sample rates
pub const VALID_SAMPLE_RATES: &[u32] = &[8000, 16000, 22050, 24000, 44100, 48000];

/// Maximum file size for uploads (10MB)
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Validate text length
pub fn validate_text_length(text: &str, max_length: usize, field_name: &str) -> Result<()> {
    if text.is_empty() {
        return Err(Error::validation(format!("{} cannot be empty", field_name)));
    }
    
    if text.len() > max_length {
        return Err(Error::validation(format!(
            "{} must be <= {} characters, got {}",
            field_name, max_length, text.len()
        )));
    }
    
    Ok(())
}

/// Validate speaking rate
pub fn validate_speaking_rate(rate: f32) -> Result<f32> {
    if rate < MIN_SPEAKING_RATE {
        Ok(MIN_SPEAKING_RATE)
    } else if rate > MAX_SPEAKING_RATE {
        Ok(MAX_SPEAKING_RATE)
    } else {
        Ok(rate)
    }
}

/// Validate pitch
pub fn validate_pitch(pitch: f32) -> Result<f32> {
    if pitch < MIN_PITCH {
        Ok(MIN_PITCH)
    } else if pitch > MAX_PITCH {
        Ok(MAX_PITCH)
    } else {
        Ok(pitch)
    }
}

/// Validate sample rate
pub fn validate_sample_rate(rate: u32) -> Result<()> {
    if !VALID_SAMPLE_RATES.contains(&rate) {
        return Err(Error::validation(format!(
            "Invalid sample rate {}. Valid rates are: {:?}",
            rate, VALID_SAMPLE_RATES
        )));
    }
    Ok(())
}

/// Validate file size
pub fn validate_file_size(size: usize, field_name: &str) -> Result<()> {
    if size > MAX_FILE_SIZE {
        return Err(Error::validation(format!(
            "{} size exceeds maximum of {} bytes, got {} bytes",
            field_name, MAX_FILE_SIZE, size
        )));
    }
    Ok(())
}

/// Validate API key format
pub fn validate_api_key(api_key: &str) -> Result<()> {
    if api_key.is_empty() {
        return Err(Error::validation("API key cannot be empty"));
    }
    
    if api_key == "dummy" {
        return Err(Error::validation("Invalid API key: 'dummy' is not allowed"));
    }
    
    // Basic format check - adjust based on actual Hume API key format
    if api_key.len() < 20 {
        return Err(Error::validation("API key appears to be invalid (too short)"));
    }
    
    Ok(())
}

/// Validate voice name
pub fn validate_voice_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(Error::validation("Voice name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(Error::validation("Voice name too long (max 100 characters)"));
    }
    
    Ok(())
}

/// Validate language code (BCP-47)
pub fn validate_language_code(code: &str) -> Result<()> {
    // Basic validation - could be enhanced with full BCP-47 parsing
    if code.is_empty() {
        return Err(Error::validation("Language code cannot be empty"));
    }
    
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(Error::validation("Invalid language code format"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_text_length() {
        assert!(validate_text_length("Hello", 10, "text").is_ok());
        assert!(validate_text_length("", 10, "text").is_err());
        assert!(validate_text_length("12345678901", 10, "text").is_err());
    }

    #[test]
    fn test_validate_speaking_rate() {
        assert_eq!(validate_speaking_rate(1.0).unwrap(), 1.0);
        assert_eq!(validate_speaking_rate(0.3).unwrap(), 0.5);
        assert_eq!(validate_speaking_rate(3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_validate_api_key() {
        assert!(validate_api_key("hume_abcdefghijklmnopqrstuvwxyz").is_ok());
        assert!(validate_api_key("").is_err());
        assert!(validate_api_key("dummy").is_err());
        assert!(validate_api_key("short").is_err());
    }
}
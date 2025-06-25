//! Unit tests for input validation

#[cfg(test)]
mod tests {
    use hume::core::validation::*;
    use hume::core::error::Error;

    #[test]
    fn test_validate_text_length_valid() {
        assert!(validate_text_length("Hello", 10, "test").is_ok());
        assert!(validate_text_length("12345", 5, "test").is_ok());
    }

    #[test]
    fn test_validate_text_length_empty() {
        let result = validate_text_length("", 10, "test");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_text_length_too_long() {
        let result = validate_text_length("12345678901", 10, "test");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => {
                assert!(msg.contains("must be <= 10"));
                assert!(msg.contains("got 11"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_speaking_rate() {
        // Valid range
        assert_eq!(validate_speaking_rate(1.0).unwrap(), 1.0);
        assert_eq!(validate_speaking_rate(0.5).unwrap(), 0.5);
        assert_eq!(validate_speaking_rate(2.0).unwrap(), 2.0);
        
        // Below minimum - should clamp
        assert_eq!(validate_speaking_rate(0.3).unwrap(), 0.5);
        assert_eq!(validate_speaking_rate(0.0).unwrap(), 0.5);
        
        // Above maximum - should clamp
        assert_eq!(validate_speaking_rate(3.0).unwrap(), 2.0);
        assert_eq!(validate_speaking_rate(10.0).unwrap(), 2.0);
    }

    #[test]
    fn test_validate_pitch() {
        // Valid range
        assert_eq!(validate_pitch(1.0).unwrap(), 1.0);
        assert_eq!(validate_pitch(0.5).unwrap(), 0.5);
        assert_eq!(validate_pitch(2.0).unwrap(), 2.0);
        
        // Below minimum - should clamp
        assert_eq!(validate_pitch(0.3).unwrap(), 0.5);
        
        // Above maximum - should clamp
        assert_eq!(validate_pitch(3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_validate_sample_rate() {
        // Valid rates
        assert!(validate_sample_rate(8000).is_ok());
        assert!(validate_sample_rate(16000).is_ok());
        assert!(validate_sample_rate(22050).is_ok());
        assert!(validate_sample_rate(24000).is_ok());
        assert!(validate_sample_rate(44100).is_ok());
        assert!(validate_sample_rate(48000).is_ok());
        
        // Invalid rates
        let result = validate_sample_rate(12000);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => {
                assert!(msg.contains("Invalid sample rate 12000"));
                assert!(msg.contains("Valid rates are"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_file_size() {
        // Valid sizes
        assert!(validate_file_size(1000, "test").is_ok());
        assert!(validate_file_size(MAX_FILE_SIZE, "test").is_ok());
        
        // Too large
        let result = validate_file_size(MAX_FILE_SIZE + 1, "test");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => {
                assert!(msg.contains("exceeds maximum"));
                assert!(msg.contains(&MAX_FILE_SIZE.to_string()));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_api_key() {
        // Valid keys
        assert!(validate_api_key("hume_abcdefghijklmnopqrstuvwxyz").is_ok());
        assert!(validate_api_key("12345678901234567890").is_ok());
        
        // Empty key
        let result = validate_api_key("");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
        
        // Dummy key
        let result = validate_api_key("dummy");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("'dummy' is not allowed")),
            _ => panic!("Expected validation error"),
        }
        
        // Too short
        let result = validate_api_key("short");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("too short")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_voice_name() {
        // Valid names
        assert!(validate_voice_name("Maya Angelou").is_ok());
        assert!(validate_voice_name("test-voice").is_ok());
        
        // Empty name
        let result = validate_voice_name("");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
        
        // Too long
        let long_name = "a".repeat(101);
        let result = validate_voice_name(&long_name);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("too long")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_validate_language_code() {
        // Valid codes
        assert!(validate_language_code("en").is_ok());
        assert!(validate_language_code("en-US").is_ok());
        assert!(validate_language_code("zh-Hans-CN").is_ok());
        
        // Empty code
        let result = validate_language_code("");
        assert!(result.is_err());
        
        // Invalid characters
        let result = validate_language_code("en_US");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("Invalid language code format")),
            _ => panic!("Expected validation error"),
        }
    }
}
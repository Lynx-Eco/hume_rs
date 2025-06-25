//! # Hume Rust SDK
//!
//! Official Rust SDK for Hume AI APIs - Text-to-Speech, Expression Measurement, and Empathic Voice Interface.
//!
//! ## Features
//!
//! - **Text-to-Speech (TTS)**: Generate natural-sounding speech with emotional expressiveness
//! - **Expression Measurement**: Analyze emotions from facial expressions, speech prosody, and language
//! - **Empathic Voice Interface (EVI)**: Build conversational AI with emotional intelligence
//! - **Async/await support**: Built on tokio for efficient async operations
//! - **Type-safe**: Leverages Rust's type system for safety and correctness
//! - **Comprehensive error handling**: Detailed error types with automatic retry logic
//!
//! ## Quick Start
//!
//! ```no_run
//! use hume::{HumeClient, tts::models::*};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client from environment variable HUME_API_KEY
//!     let client = HumeClient::from_env()?;
//!     
//!     // Use Text-to-Speech
//!     let tts = client.tts();
//!     let request = TtsRequestBuilder::new()
//!         .utterance("Hello, world!")
//!         .unwrap()
//!         .build();
//!     
//!     let response = tts.synthesize(request, None).await?;
//!     
//!     // Save audio to file
//!     std::fs::write("output.mp3", &response.generations[0].data)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The SDK supports multiple authentication methods:
//!
//! ### API Key
//! ```no_run
//! # use hume::HumeClientBuilder;
//! let client = HumeClientBuilder::new()
//!     .with_api_key("your-api-key")
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Access Token
//! ```no_run
//! # use hume::HumeClientBuilder;
//! let client = HumeClientBuilder::new()
//!     .with_access_token("your-access-token")
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Environment Variable
//! ```no_run
//! # use hume::HumeClient;
//! // Reads from HUME_API_KEY environment variable
//! let client = HumeClient::from_env()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Error Handling
//!
//! All SDK methods return `Result<T, Error>` where `Error` is a comprehensive error type:
//!
//! ```no_run
//! # use hume::{HumeClient, Error};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = HumeClient::from_env()?;
//! # let tts = client.tts();
//! match tts.list_voices(None).await {
//!     Ok(voices) => println!("Found {} voices", voices.voices.len()),
//!     Err(Error::Api { status, message, .. }) => {
//!         eprintln!("API error {}: {}", status, message);
//!     }
//!     Err(Error::RateLimit { retry_after }) => {
//!         eprintln!("Rate limited. Retry after: {:?}", retry_after);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Modules
//!
//! - [`core`]: Core functionality including authentication, HTTP client, and error types
//! - [`tts`]: Text-to-Speech API
//! - [`expression`]: Expression Measurement API (also available as `expression_measurement`)
//! - [`evi`]: Empathic Voice Interface API
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive examples:
//! - `tts_basic.rs` - Simple text-to-speech
//! - `expression_measurement.rs` - Emotion analysis
//! - `evi_chat.rs` - Interactive chat session
//!
//! ## Environment Variables
//!
//! - `HUME_API_KEY` - Your Hume API key
//! - `HUME_BASE_URL` - Custom API base URL (optional)

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod core;
pub mod evi;
pub mod expression_measurement;
pub mod tts;

/// Alias for expression_measurement module for convenience
pub use expression_measurement as expression;

// Re-export main types
pub use crate::core::{
    client::{HumeClient, HumeClientBuilder},
    error::{Error, Result},
};

pub use crate::evi::EviClient;
pub use crate::expression_measurement::ExpressionMeasurementClient;
pub use crate::tts::TtsClient;

/// The version of this SDK
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for Hume API
pub const DEFAULT_BASE_URL: &str = "https://api.hume.ai";
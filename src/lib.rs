//! Hume AI Rust SDK
//!
//! This SDK provides Rust bindings for Hume AI's APIs including:
//! - Empathic Voice Interface (EVI)
//! - Text-to-Speech (TTS)
//! - Expression Measurement
//!
//! # Example
//!
//! ```no_run
//! use hume::{HumeClient, HumeClientBuilder};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = HumeClientBuilder::new("your-api-key")
//!         .build()?;
//!
//!     // Use the client to interact with Hume APIs
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod core;
pub mod evi;
pub mod expression_measurement;
pub mod tts;

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
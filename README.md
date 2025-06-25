# Hume Rust SDK

[![Crates.io](https://img.shields.io/crates/v/hume.svg)](https://crates.io/crates/hume)
[![Documentation](https://docs.rs/hume/badge.svg)](https://docs.rs/hume)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Official Rust SDK for Hume AI APIs - Text-to-Speech, Expression Measurement, and Empathic Voice Interface.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hume = "0.9.0"
```

## Quick Start

```rust
use hume::{HumeClient, TtsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with API key
    let client = HumeClient::new("your-api-key")?;
    
    // Use TTS to synthesize speech
    let tts = TtsClient::from(client);
    let audio = tts.synthesize_simple("Hello, world!", None).await?;
    
    // Save the audio
    std::fs::write("output.mp3", audio)?;
    
    Ok(())
}
```

## Authentication

The SDK supports two authentication methods:

### API Key

```rust
let client = HumeClient::new("your-api-key")?;
```

### Access Token

```rust
use hume::core::auth::AuthToken;

let token = AuthToken::new("access-token".to_string(), "Bearer".to_string(), 3600);
let client = HumeClientBuilder::default()
    .access_token(token)
    .build()?;
```

## API Clients

### Text-to-Speech (TTS)

```rust
use hume::{TtsClient, tts::models::*};

let tts = TtsClient::from(client);

// Simple synthesis
let audio = tts.synthesize_simple("Hello!", Some("voice-id")).await?;

// Advanced synthesis with options
let request = TtsRequestBuilder::new()
    .utterance_with_voice("Hello!", "voice-id")
    .format(AudioFormat::Wav)
    .return_durations(true)
    .build();

let response = tts.synthesize(request, None).await?;

// List available voices
let voices = tts.list_voices(None).await?;
```

### Expression Measurement

```rust
use hume::{ExpressionMeasurementClient, expression_measurement::models::*};

let em = ExpressionMeasurementClient::from(client);

// Batch processing
let batch = em.batch();
let job = batch.create_job_from_text(
    models,
    vec!["Analyze this text"],
    None,
    None,
    None
).await?;

// Wait for completion
let completed = batch.wait_for_job_completion(
    &job.job_id,
    Duration::from_secs(2),
    Some(Duration::from_secs(60))
).await?;

// Get results
let predictions = batch.get_predictions(&job.job_id, None).await?;
```

### Empathic Voice Interface (EVI)

```rust
use hume::{EviClient, evi::chat::ChatSessionBuilder};

let evi = EviClient::from(client);

// Manage tools, prompts, voices, and configs
let tools = evi.tools();
let prompts = evi.prompts();
let voices = evi.voices();
let configs = evi.configs();

// Connect to chat
let mut chat = ChatSessionBuilder::new()
    .config_id("config-id")
    .connect(&evi.chat())
    .await?;

// Send messages
chat.send_text("Hello!".to_string()).await?;

// Receive responses
while let Some(message) = chat.receive().await? {
    // Handle server messages
}
```

## Error Handling

The SDK uses a unified `Error` type:

```rust
use hume::Error;

match tts.list_voices(None).await {
    Ok(voices) => println!("Found {} voices", voices.voices.len()),
    Err(Error::Api { status, message, .. }) => {
        eprintln!("API error ({}): {}", status, message);
    }
    Err(Error::RateLimit { retry_after }) => {
        eprintln!("Rate limited. Retry after: {:?}", retry_after);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Advanced Configuration

```rust
use std::time::Duration;

let client = HumeClientBuilder::new("api-key")
    .base_url("https://custom.hume.ai")
    .timeout(Duration::from_secs(60))
    .max_retries(5)
    .build()?;
```

## Examples

See the `examples/` directory for complete examples:
- `tts_basic.rs` - Text-to-speech synthesis
- `evi_chat.rs` - EVI chat session
- `expression_measurement.rs` - Expression analysis

Run examples with:
```bash
HUME_API_KEY=your-key cargo run --example tts_basic
```

## Features

- **Async/await** - Built on Tokio for async operations
- **Type-safe** - Strongly typed request/response models
- **Retry logic** - Automatic retries with exponential backoff
- **WebSocket support** - For EVI chat and streaming
- **Comprehensive error handling** - Detailed error types
- **Builder patterns** - Convenient request construction

## Requirements

- Rust 1.70+
- Tokio runtime

## License

MIT
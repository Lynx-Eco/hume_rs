//! Data models for Text-to-Speech API

use serde::{Deserialize, Serialize};
use crate::core::validation::{validate_text_length, validate_speaking_rate, MAX_TTS_TEXT_LENGTH};
use crate::core::error::Result;

/// TTS synthesis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    /// List of utterances to synthesize
    pub utterances: Vec<Utterance>,
    
    /// Global context for consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    
    /// Audio format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,
    
    /// Sample rate (for PCM format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<SampleRate>,
}

impl Default for TtsRequest {
    fn default() -> Self {
        Self {
            utterances: Vec::new(),
            context: None,
            format: None,
            sample_rate: None,
        }
    }
}

/// Single utterance to synthesize
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Utterance {
    /// Text to synthesize
    pub text: String,
    
    /// Voice specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSpec>,
    
    /// Description for emotional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Speech speed (0.5 to 2.0, default 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    
    /// Trailing silence in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_silence: Option<u32>,
}

/// Voice specification for utterances
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceSpec {
    /// Voice specified by ID
    Id {
        /// Voice ID
        id: String,
        /// Voice provider
        #[serde(skip_serializing_if = "Option::is_none")]
        provider: Option<VoiceProvider>,
    },
    /// Voice specified by name
    Name {
        /// Voice name
        name: String,
        /// Voice provider
        #[serde(skip_serializing_if = "Option::is_none")]
        provider: Option<VoiceProvider>,
    },
}

/// Voice provider
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceProvider {
    /// Hume AI voice
    HumeAi,
    /// Custom voice
    CustomVoice,
}

/// Context for maintaining consistency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Previous text for context
    pub text: String,
    
    /// Voice used for previous text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

/// Audio format specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AudioFormat {
    /// MP3 format (default)
    Mp3,
    /// WAV format
    Wav,
    /// Raw PCM format
    Pcm,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::Mp3
    }
}

/// Common sample rates for audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct SampleRate(u32);

impl SampleRate {
    /// 8 kHz - Telephone quality
    pub const HZ_8000: Self = Self(8000);
    
    /// 16 kHz - Wideband audio
    pub const HZ_16000: Self = Self(16000);
    
    /// 22.05 kHz - Half of CD quality
    pub const HZ_22050: Self = Self(22050);
    
    /// 24 kHz - Professional audio
    pub const HZ_24000: Self = Self(24000);
    
    /// 44.1 kHz - CD quality
    pub const HZ_44100: Self = Self(44100);
    
    /// 48 kHz - Professional/DVD quality
    pub const HZ_48000: Self = Self(48000);
    
    /// Create a custom sample rate
    pub const fn custom(rate: u32) -> Self {
        Self(rate)
    }
    
    /// Get the raw sample rate value
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<SampleRate> for u32 {
    fn from(rate: SampleRate) -> Self {
        rate.0
    }
}

impl Default for SampleRate {
    fn default() -> Self {
        Self::HZ_24000 // Default to 24kHz as per API docs
    }
}

/// TTS synthesis response
#[derive(Debug, Clone, Deserialize)]
pub struct TtsResponse {
    /// List of generated audio segments
    pub generations: Vec<Generation>,
}

/// Single generation result
#[derive(Debug, Clone, Deserialize)]
pub struct Generation {
    /// Base64 encoded audio data
    pub data: String,
    
    /// Duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    
    /// Voice used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

/// Request for streaming TTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsStreamRequest {
    /// Text to synthesize
    pub text: String,
    
    /// Voice specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSpec>,
    
    /// Description for emotional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Speech speed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    
    /// Audio format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,
    
    /// Sample rate (for PCM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<SampleRate>,
    
    /// Enable instant streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<bool>,
}

impl Default for TtsStreamRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            voice: None,
            description: None,
            speed: None,
            format: None,
            sample_rate: None,
            instant: None,
        }
    }
}

/// Streaming TTS response chunk
#[derive(Debug, Clone, Deserialize)]
pub struct TtsStreamResponse {
    /// Chunk index
    pub index: u32,
    
    /// Base64 encoded audio chunk
    pub data: String,
    
    /// Duration of this chunk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    
    /// Whether this is the final chunk
    pub is_final: bool,
}

/// Available voices response
#[derive(Debug, Clone, Deserialize)]
pub struct VoicesResponse {
    /// List of available voices
    pub voices: Vec<Voice>,
}

/// Voice information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Voice {
    /// Voice ID
    pub id: String,
    
    /// Voice name
    pub name: String,
    
    /// Voice description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Voice gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    
    /// Voice age
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    
    /// Voice language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    /// Whether this is a custom voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    
    /// Voice tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Builder for TTS requests
/// 
/// # Example
/// 
/// ```no_run
/// use hume::tts::models::{TtsRequestBuilder, AudioFormat, SampleRate};
/// 
/// let request = TtsRequestBuilder::new()
///     .utterance("Hello, world!")
///     .format(AudioFormat::Wav)
///     .sample_rate(SampleRate::HZ_44100)
///     .build();
/// ```
#[derive(Debug)]
pub struct TtsRequestBuilder {
    request: TtsRequest,
}

impl TtsRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            request: TtsRequest::default(),
        }
    }

    /// Add an utterance
    pub fn utterance(mut self, text: impl Into<String>) -> Result<Self> {
        let text = text.into();
        validate_text_length(&text, MAX_TTS_TEXT_LENGTH, "TTS text")?;
        self.request.utterances.push(Utterance {
            text,
            ..Default::default()
        });
        Ok(self)
    }

    /// Add an utterance with voice name
    pub fn utterance_with_voice(
        mut self,
        text: impl Into<String>,
        voice_name: impl Into<String>,
    ) -> Result<Self> {
        let text = text.into();
        validate_text_length(&text, MAX_TTS_TEXT_LENGTH, "TTS text")?;
        self.request.utterances.push(Utterance {
            text,
            voice: Some(VoiceSpec::Name {
                name: voice_name.into(),
                provider: None,
            }),
            ..Default::default()
        });
        Ok(self)
    }
    
    /// Add an utterance with voice ID
    pub fn utterance_with_voice_id(
        mut self,
        text: impl Into<String>,
        voice_id: impl Into<String>,
    ) -> Self {
        self.request.utterances.push(Utterance {
            text: text.into(),
            voice: Some(VoiceSpec::Id {
                id: voice_id.into(),
                provider: None,
            }),
            ..Default::default()
        });
        self
    }

    /// Add a full utterance
    pub fn add_utterance(mut self, mut utterance: Utterance) -> Result<Self> {
        // Validate text
        validate_text_length(&utterance.text, MAX_TTS_TEXT_LENGTH, "TTS text")?;
        
        // Validate and clamp speed if provided
        if let Some(speed) = utterance.speed {
            utterance.speed = Some(validate_speaking_rate(speed)?);
        }
        
        self.request.utterances.push(utterance);
        Ok(self)
    }

    /// Set context
    pub fn context(mut self, text: impl Into<String>, voice: Option<String>) -> Self {
        self.request.context = Some(Context {
            text: text.into(),
            voice,
        });
        self
    }

    /// Set audio format
    pub fn format(mut self, format: AudioFormat) -> Self {
        self.request.format = Some(format);
        self
    }

    /// Set sample rate
    pub fn sample_rate(mut self, rate: SampleRate) -> Self {
        self.request.sample_rate = Some(rate);
        self
    }

    /// Build the request
    pub fn build(self) -> TtsRequest {
        self.request
    }
}

impl Default for TtsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
//! Data models for Empathic Voice Interface API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool ID
    pub id: String,
    
    /// Tool name
    pub name: String,
    
    /// Tool description
    pub description: String,
    
    /// Tool parameters schema
    pub parameters: serde_json::Value,
    
    /// Whether the tool is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    
    /// Tool version ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Prompt definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    /// Prompt ID
    pub id: String,
    
    /// Prompt name
    pub name: String,
    
    /// Prompt text
    pub text: String,
    
    /// Prompt version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    
    /// Version description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Custom voice definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomVoice {
    /// Voice ID
    pub id: String,
    
    /// Voice name
    pub name: String,
    
    /// Base voice ID
    pub base_voice_id: String,
    
    /// Voice parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<VoiceParameters>,
    
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Voice parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceParameters {
    /// Pitch adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    
    /// Rate adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    
    /// Volume adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
}

/// EVI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Config ID
    pub id: String,
    
    /// Config name
    pub name: String,
    
    /// Config version
    pub version: u32,
    
    /// Prompt specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptSpec>,
    
    /// Voice specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSpec>,
    
    /// Language model specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model: Option<LanguageModelSpec>,
    
    /// Tools specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolSpec>>,
    
    /// Event messages configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_messages: Option<EventMessagesSpec>,
    
    /// Timeouts configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<TimeoutsSpec>,
    
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Prompt specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSpec {
    /// Prompt ID
    pub id: String,
    
    /// Prompt version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// Voice specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSpec {
    /// Voice ID
    pub id: String,
}

/// Language model specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModelSpec {
    /// Model provider
    pub model_provider: String,
    
    /// Model resource
    pub model_resource: String,
    
    /// Temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

/// Tool specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    /// Tool ID
    pub id: String,
    
    /// Tool version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// Event messages specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagesSpec {
    /// On new chat message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_new_chat: Option<String>,
    
    /// On inactivity timeout message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_inactivity_timeout: Option<String>,
    
    /// On max duration timeout message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_max_duration_timeout: Option<String>,
}

/// Timeouts specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutsSpec {
    /// Inactivity timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactivity: Option<u32>,
    
    /// Max duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,
}

/// Chat session settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Audio configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,
    
    /// System prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    
    /// Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    
    /// Variable values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
    
    /// Tool IDs to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    
    /// Built-in tools configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builtin_tools: Option<Vec<BuiltinTool>>,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Input encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_encoding: Option<AudioEncoding>,
    
    /// Input sample rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_sample_rate: Option<u32>,
    
    /// Output encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encoding: Option<AudioEncoding>,
    
    /// Output sample rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sample_rate: Option<u32>,
    
    /// Output format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<AudioFormat>,
}

/// Audio encoding
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioEncoding {
    /// Linear PCM 16-bit
    Linear16,
    /// μ-law
    Mulaw,
}

/// Audio format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    /// Raw audio
    Raw,
    /// WAV container
    Wav,
    /// MP3 format
    Mp3,
}

/// Context for the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Context type
    #[serde(rename = "type")]
    pub context_type: ContextType,
    
    /// Context text
    pub text: String,
}

/// Context type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextType {
    /// Persistent context
    Persistent,
    /// Temporary context
    Temporary,
}

/// Built-in tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTool {
    /// Tool name
    pub name: String,
    
    /// Tool configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// Chat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    /// Chat ID
    pub id: String,
    
    /// Chat group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_group_id: Option<String>,
    
    /// Config ID used
    pub config_id: String,
    
    /// Config version used
    pub config_version: u32,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<Utc>>,
    
    /// Chat status
    pub status: ChatStatus,
    
    /// Metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Chat status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatStatus {
    /// Chat is active
    Active,
    /// Chat has ended
    Ended,
    /// Chat was interrupted
    Interrupted,
    /// Chat had an error
    Error,
}

/// Chat group information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGroup {
    /// Group ID
    pub id: String,
    
    /// First chat ID
    pub first_chat_id: String,
    
    /// Most recent chat ID
    pub most_recent_chat_id: String,
    
    /// Number of chats
    pub num_chats: u32,
    
    /// Is currently active
    pub is_active: bool,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message ID
    pub id: String,
    
    /// Message role
    pub role: MessageRole,
    
    /// Message content
    pub content: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Tool calls made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    
    /// Emotion inference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion_inference: Option<EmotionInference>,
}

/// Message role
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// System message
    System,
    /// Tool message
    Tool,
}

/// Tool call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name
    pub tool_name: String,
    
    /// Tool parameters
    pub parameters: serde_json::Value,
    
    /// Tool response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
    
    /// Error if tool failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Emotion inference result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionInference {
    /// Inferred emotions
    pub emotions: HashMap<String, f32>,
    
    /// Prosody analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prosody: Option<Prosody>,
}

/// Prosody information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prosody {
    /// Pitch
    pub pitch: f32,
    
    /// Energy
    pub energy: f32,
    
    /// Speech rate
    pub speech_rate: f32,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// Total items
    pub total_items: u64,
    
    /// Page items
    pub items: Vec<T>,
}

/// Paginated response for configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedConfigs {
    /// Page number
    pub page_number: Option<u32>,
    
    /// Page size
    pub page_size: Option<u32>,
    
    /// Total pages
    pub total_pages: u32,
    
    /// List of configs
    pub configs_page: Option<Vec<Config>>,
}

/// Paginated response for user defined tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedUserDefinedTools {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// List of tools
    pub tools_page: Vec<Option<Tool>>,
}

/// Paginated response for prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedPrompts {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// List of prompts
    pub prompts_page: Vec<Option<Prompt>>,
}

/// Paginated response for custom voices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedCustomVoices {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// List of custom voices
    pub custom_voices_page: Vec<CustomVoice>,
}

/// Pagination direction for chats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaginationDirection {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Paginated response for chats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedChats {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// Pagination direction
    pub pagination_direction: PaginationDirection,
    
    /// List of chats
    pub chats_page: Vec<Chat>,
}

/// Paginated response for chat groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPagedChatGroups {
    /// Page number
    pub page_number: u32,
    
    /// Page size
    pub page_size: u32,
    
    /// Total pages
    pub total_pages: u32,
    
    /// Pagination direction
    pub pagination_direction: PaginationDirection,
    
    /// List of chat groups
    pub chat_groups_page: Vec<ChatGroup>,
}
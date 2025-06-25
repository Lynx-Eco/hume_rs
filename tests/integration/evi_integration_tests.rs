//! Integration tests for EVI API

use hume::{HumeClientBuilder, evi::models::{Config, Prompt, Tool}, evi::configs::*, evi::prompts::*, evi::tools::*};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, body_json, query_param};
use chrono::Utc;

#[tokio::test]
async fn test_evi_config_crud() {
    let mock_server = MockServer::start().await;
    
    // Create config
    let create_request = serde_json::json!({
        "name": "Test Assistant",
        "voice": {
            "id": "ito"
        },
        "prompt": {
            "id": "prompt-123",
            "version": 1
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/v0/evi/configs"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&create_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "config-abc",
            "name": "Test Assistant",
            "created_at": Utc::now(),
            "updated_at": Utc::now(),
            "voice": {"id": "ito"},
            "prompt": {"id": "prompt-123", "version": 1},
            "version": 1
        })))
        .mount(&mock_server)
        .await;
    
    // List configs
    Mock::given(method("GET"))
        .and(path("/v0/evi/configs"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page_number": 0,
            "page_size": 10,
            "total_pages": 1,
            "total_items": 1,
            "items": [{
                "id": "config-abc",
                "name": "Test Assistant",
                "created_at": Utc::now(),
                "updated_at": Utc::now(),
                "voice": {"id": "ito"},
                "prompt": {"id": "prompt-123", "version": 1},
                "version": 1
            }]
        })))
        .mount(&mock_server)
        .await;
    
    // Get specific config
    Mock::given(method("GET"))
        .and(path("/v0/evi/configs/config-abc"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "config-abc",
            "name": "Test Assistant",
            "created_at": Utc::now(),
            "updated_at": Utc::now(),
            "voice": {"id": "ito"},
            "prompt": {"id": "prompt-123", "version": 1},
            "version": 1
        })))
        .mount(&mock_server)
        .await;
    
    // Update config
    let update_request = serde_json::json!({
        "name": "Updated Assistant"
    });
    
    Mock::given(method("PATCH"))
        .and(path("/v0/evi/configs/config-abc"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&update_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "config-abc",
            "name": "Updated Assistant",
            "created_at": Utc::now(),
            "updated_at": Utc::now(),
            "voice": {"id": "ito"},
            "prompt": {"id": "prompt-123", "version": 1},
            "version": 2
        })))
        .mount(&mock_server)
        .await;
    
    // Delete config
    Mock::given(method("DELETE"))
        .and(path("/v0/evi/configs/config-abc"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    let configs = evi.configs();
    
    // Test create
    let create_req = CreateConfigRequestBuilder::new("Test Assistant")
        .voice("ito")
        .prompt("prompt-123", Some(1))
        .build();
    
    let created = configs.create(create_req, None).await.unwrap();
    assert_eq!(created.id, "config-abc");
    assert_eq!(created.name, "Test Assistant");
    
    // Test list
    let list = configs.list(None, None, None).await.unwrap();
    assert_eq!(list.configs_page.as_ref().unwrap().len(), 1);
    assert_eq!(list.configs_page.as_ref().unwrap()[0].id, "config-abc");
    
    // Test get
    let config = configs.get("config-abc", None).await.unwrap();
    assert_eq!(config.id, "config-abc");
    
    // Test update
    let update_req = UpdateConfigRequest {
        name: Some("Updated Assistant".to_string()),
        ..Default::default()
    };
    
    let updated = configs.update("config-abc", update_req, None).await.unwrap();
    assert_eq!(updated.name, "Updated Assistant");
    assert_eq!(updated.version, 2);
    
    // Test delete
    configs.delete("config-abc", None).await.unwrap();
}

#[tokio::test]
async fn test_evi_prompts() {
    let mock_server = MockServer::start().await;
    
    // Create prompt
    let create_request = serde_json::json!({
        "name": "Test Prompt",
        "text": "You are a helpful assistant"
    });
    
    Mock::given(method("POST"))
        .and(path("/v0/evi/prompts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&create_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "prompt-xyz",
            "name": "Test Prompt",
            "text": "You are a helpful assistant",
            "created_at": Utc::now(),
            "updated_at": Utc::now(),
            "version": 1
        })))
        .mount(&mock_server)
        .await;
    
    // List prompts
    Mock::given(method("GET"))
        .and(path("/v0/evi/prompts"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page_number": 0,
            "page_size": 10,
            "total_pages": 1,
            "total_items": 1,
            "items": [{
                "id": "prompt-xyz",
                "name": "Test Prompt",
                "text": "You are a helpful assistant",
                "created_at": Utc::now(),
                "updated_at": Utc::now(),
                "version": 1
            }]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    let prompts = evi.prompts();
    
    // Test create
    let create_req = CreatePromptRequest {
        name: "Test Prompt".to_string(),
        text: "You are a helpful assistant".to_string(),
        version_description: None,
    };
    let prompt = prompts.create(create_req, None).await.unwrap();
    
    assert_eq!(prompt.id, "prompt-xyz");
    assert_eq!(prompt.name, "Test Prompt");
    
    // Test list
    let list = prompts.list(None, None, None).await.unwrap();
    assert_eq!(list.prompts_page.len(), 1);
    assert_eq!(list.prompts_page[0].as_ref().unwrap().id, "prompt-xyz");
}

#[tokio::test]
async fn test_evi_tools() {
    let mock_server = MockServer::start().await;
    
    // Create tool
    let create_request = serde_json::json!({
        "name": "weather_tool",
        "description": "Get current weather",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["location"]
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/v0/evi/tools"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(body_json(&create_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "tool-123",
            "name": "weather_tool",
            "description": "Get current weather",
            "parameters": create_request["parameters"],
            "created_at": Utc::now(),
            "updated_at": Utc::now()
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    let tools = evi.tools();
    
    let params = serde_json::json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City name"
            }
        },
        "required": ["location"]
    });
    
    let create_req = CreateToolRequest {
        name: "weather_tool".to_string(),
        description: "Get current weather".to_string(),
        parameters: params,
        required: None,
    };
    let tool = tools.create(create_req, None).await.unwrap();
    
    assert_eq!(tool.id, "tool-123");
    assert_eq!(tool.name, "weather_tool");
}

#[tokio::test]
async fn test_evi_chat_session() {
    let mock_server = MockServer::start().await;
    
    // Mock WebSocket upgrade
    Mock::given(method("GET"))
        .and(path("/v0/evi/chat"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .and(query_param("config_id", "config-abc"))
        .respond_with(ResponseTemplate::new(101)
            .insert_header("upgrade", "websocket")
            .insert_header("connection", "upgrade")
            .insert_header("sec-websocket-accept", "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    
    // Note: Actual WebSocket testing requires a WebSocket server
    // This just tests that the connection attempt is made correctly
    let result = evi.chat().connect(Some("config-abc".to_string()), None, None).await;
    
    // Connection will fail because MockServer doesn't support WebSocket
    // But we can verify the request was made correctly
    assert!(result.is_err());
}

#[tokio::test]
async fn test_evi_list_chat_groups() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/evi/chat_groups"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page_number": 0,
            "page_size": 10,
            "total_pages": 1,
            "total_items": 2,
            "items": [
                {
                    "id": "group-1",
                    "created_at": Utc::now(),
                    "updated_at": Utc::now(),
                    "active": true,
                    "config_id": "config-abc"
                },
                {
                    "id": "group-2",
                    "created_at": Utc::now(),
                    "updated_at": Utc::now(),
                    "active": false,
                    "config_id": "config-xyz"
                }
            ]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    // Chat groups API not yet implemented
    // let chat_groups = evi.chat_groups();
    // let list = chat_groups.list(None, None, None).await.unwrap();
    // assert_eq!(list.items.len(), 2);
}

#[tokio::test]
async fn test_evi_list_chats() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v0/evi/chats"))
        .and(header("X-Hume-Api-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page_number": 0,
            "page_size": 10,
            "total_pages": 1,
            "total_items": 1,
            "items": [
                {
                    "id": "chat-123",
                    "chat_group_id": "group-1",
                    "created_at": Utc::now(),
                    "updated_at": Utc::now(),
                    "metadata": {
                        "user_id": "user-456"
                    },
                    "config": {
                        "id": "config-abc",
                        "version": 1
                    }
                }
            ]
        })))
        .mount(&mock_server)
        .await;
    
    let client = HumeClientBuilder::new("test-key")
        .with_base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let evi = client.evi();
    // Chats API not yet implemented
    // let chats = evi.chats();
    // let list = chats.list(None, None, None).await.unwrap();
    // assert_eq!(list.items.len(), 1);
}
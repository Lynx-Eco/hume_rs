//! Comprehensive EVI Tools Example
//! 
//! This example demonstrates all aspects of tool management in EVI:
//! - Listing existing tools
//! - Creating custom tools
//! - Updating tools
//! - Using tools in conversations
//! - Deleting tools

use hume::{HumeClient, EviClient};
use hume::evi::tools::{CreateToolRequest, UpdateToolRequest};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛠️  EVI Tools Comprehensive Example");
    println!("===================================\n");
    
    dotenvy::dotenv().ok();
    let has_api_key = std::env::var("HUME_API_KEY")
        .map(|k| !k.is_empty() && k != "dummy")
        .unwrap_or(false);
    
    if !has_api_key {
        println!("📋 Running in DEMO MODE (no API key)");
        println!("   This example shows tool management patterns but requires an API key to run.\n");
        demonstrate_tool_patterns();
        return Ok(());
    }
    
    let api_key = std::env::var("HUME_API_KEY")?;
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    let tools_client = evi.tools();
    
    // Example 1: List existing tools
    println!("📌 Example 1: Listing Existing Tools");
    println!("------------------------------------");
    
    match tools_client.list(Some(10), None, None).await {
        Ok(tools) => {
            println!("Found {} tool(s):", tools.tools_page.len());
            for (i, tool_opt) in tools.tools_page.iter().enumerate() {
                if let Some(tool) = tool_opt {
                    println!("\n{}. {} ({})", i + 1, tool.name, tool.id);
                    println!("   Description: {}", tool.description);
                    println!("   Created: {:?}", tool.created_at);
                    
                    if let Ok(params_str) = serde_json::to_string_pretty(&tool.parameters) {
                        println!("   Parameters Schema:");
                        for line in params_str.lines() {
                            println!("     {}", line);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error listing tools: {}", e);
        }
    }
    
    // Example 2: Create a new tool
    println!("\n\n📌 Example 2: Creating Custom Tools");
    println!("-----------------------------------");
    
    // Weather tool
    let weather_tool = CreateToolRequest::builder(
        "get_current_weather",
        "Get the current weather for a location"
    )
    .parameters(json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City and state, e.g. San Francisco, CA"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "default": "fahrenheit"
            }
        },
        "required": ["location"]
    }))
    .build();
    
    println!("Creating weather tool...");
    match tools_client.create(weather_tool, None).await {
        Ok(tool) => {
            println!("✓ Created tool: {} ({})", tool.name, tool.id);
            
            // Example 3: Update the tool
            println!("\n\n📌 Example 3: Updating Tool");
            println!("---------------------------");
            
            let update = UpdateToolRequest {
                description: Some("Get current weather and forecast for any location worldwide".to_string()),
                parameters: Some(json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "City, state/country, e.g. London, UK"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit", "kelvin"],
                            "default": "fahrenheit"
                        },
                        "include_forecast": {
                            "type": "boolean",
                            "description": "Include 5-day forecast",
                            "default": false
                        }
                    },
                    "required": ["location"]
                })),
                ..Default::default()
            };
            
            match tools_client.update(&tool.id, update, None).await {
                Ok(updated) => {
                    println!("✓ Updated tool description and parameters");
                    println!("  New description: {}", updated.description);
                }
                Err(e) => println!("Error updating tool: {}", e),
            }
            
            // Example 4: List tool versions
            println!("\n\n📌 Example 4: Tool Version History");
            println!("----------------------------------");
            
            match tools_client.list_versions(&tool.id, None, None, None).await {
                Ok(versions) => {
                    println!("Tool has {} version(s):", versions.tools_page.len());
                    for version_opt in versions.tools_page.iter() {
                        if let Some(version) = version_opt {
                            println!("  - Version ID: {}", version.version_id.as_deref().unwrap_or("unknown"));
                            println!("    Created: {:?}", version.created_at);
                        }
                    }
                }
                Err(e) => println!("Error listing versions: {}", e),
            }
            
            // Cleanup
            println!("\n🧹 Cleaning up - deleting test tool...");
            match tools_client.delete(&tool.id, None).await {
                Ok(_) => println!("✓ Tool deleted successfully"),
                Err(e) => println!("Error deleting tool: {}", e),
            }
        }
        Err(e) => {
            println!("Error creating tool: {}", e);
            if e.to_string().contains("already exists") {
                println!("  (Tool with this name may already exist)");
            }
        }
    }
    
    // Example 5: More tool examples
    println!("\n\n📌 Example 5: Additional Tool Patterns");
    println!("--------------------------------------");
    demonstrate_advanced_tools();
    
    Ok(())
}

fn demonstrate_tool_patterns() {
    println!("🔧 Tool Management Patterns\n");
    
    println!("1. Basic Tool Creation:");
    println!("```rust");
    println!("let tool = CreateToolRequest::builder(");
    println!("    \"search_web\",");
    println!("    \"Search the web for information\"");
    println!(")");
    println!(".parameters(json!({{");
    println!("    \"type\": \"object\",");
    println!("    \"properties\": {{");
    println!("        \"query\": {{");
    println!("            \"type\": \"string\",");
    println!("            \"description\": \"Search query\"");
    println!("        }}");
    println!("    }},");
    println!("    \"required\": [\"query\"]");
    println!("}}))");
    println!(".build();");
    println!("```\n");
    
    println!("2. Tool with Complex Parameters:");
    println!("```rust");
    println!("let calendar_tool = CreateToolRequest::builder(");
    println!("    \"schedule_meeting\",");
    println!("    \"Schedule a meeting on the calendar\"");
    println!(")");
    println!(".parameters(json!({{");
    println!("    \"type\": \"object\",");
    println!("    \"properties\": {{");
    println!("        \"title\": {{ \"type\": \"string\" }},");
    println!("        \"datetime\": {{");
    println!("            \"type\": \"string\",");
    println!("            \"format\": \"date-time\"");
    println!("        }},");
    println!("        \"duration_minutes\": {{");
    println!("            \"type\": \"integer\",");
    println!("            \"minimum\": 15,");
    println!("            \"maximum\": 480");
    println!("        }},");
    println!("        \"attendees\": {{");
    println!("            \"type\": \"array\",");
    println!("            \"items\": {{ \"type\": \"string\" }}");
    println!("        }}");
    println!("    }},");
    println!("    \"required\": [\"title\", \"datetime\", \"duration_minutes\"]");
    println!("}}))");
    println!(".build();");
    println!("```\n");
    
    println!("3. Using Tools in Conversations:");
    println!("```rust");
    println!("// Configure chat with tools");
    println!("let session = SessionSettings {{");
    println!("    tools: Some(vec![");
    println!("        ToolSpec {{ id: \"tool-id-1\".to_string(), version: None }},");
    println!("        ToolSpec {{ id: \"tool-id-2\".to_string(), version: Some(2) }},");
    println!("    ]),");
    println!("    ..Default::default()");
    println!("}};");
    println!();
    println!("// Handle tool calls in chat");
    println!("match message {{");
    println!("    ServerMessage::ToolCall {{ name, parameters, tool_call_id }} => {{");
    println!("        let result = execute_tool(&name, &parameters).await?;");
    println!("        chat.send_tool_response(tool_call_id, result).await?;");
    println!("    }}");
    println!("    _ => {{}}");
    println!("}}");
    println!("```");
}

fn demonstrate_advanced_tools() {
    println!("\n🎯 Advanced Tool Examples:\n");
    
    println!("1. Database Query Tool:");
    println!("   - Name: query_database");
    println!("   - Parameters: sql_query (with validation)");
    println!("   - Use case: Allow AI to query structured data\n");
    
    println!("2. API Integration Tool:");
    println!("   - Name: call_external_api");
    println!("   - Parameters: endpoint, method, headers, body");
    println!("   - Use case: Integrate with external services\n");
    
    println!("3. File Operation Tool:");
    println!("   - Name: manage_files");
    println!("   - Parameters: action (read/write/list), path, content");
    println!("   - Use case: Allow AI to work with files\n");
    
    println!("4. Calculation Tool:");
    println!("   - Name: calculate");
    println!("   - Parameters: expression, precision");
    println!("   - Use case: Perform complex calculations\n");
    
    println!("5. Memory Tool:");
    println!("   - Name: remember");
    println!("   - Parameters: key, value, action (store/retrieve/delete)");
    println!("   - Use case: Persistent memory across conversations");
}
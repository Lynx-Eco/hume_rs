//! EVI Tools Management Example

use hume::{HumeClient, EviClient};
use hume::evi::tools::CreateToolRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("HUME_API_KEY")
        .expect("Please set HUME_API_KEY environment variable");
    
    let client = HumeClient::new(api_key)?;
    let evi = EviClient::from(client);
    let tools_client = evi.tools();
    
    // Example 1: List existing tools
    println!("Example 1: Listing existing tools");
    let tools = tools_client.list(None, None, None).await?;
    
    println!("Found {} tools:", tools.tools_page.len());
    for tool_opt in tools.tools_page.iter().take(5) {
        if let Some(tool) = tool_opt {
            println!("  - {} ({}): {}", tool.name, tool.id, tool.description);
            if let Some(created) = &tool.created_at {
                println!("    Created: {}", created);
            }
        }
    }
    
    // Example 2: Create a new tool
    println!("\nExample 2: Creating a new tool");
    let weather_tool = CreateToolRequest::builder(
        "get_weather",
        "Get current weather for a location"
    )
    .parameters(serde_json::json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City and state, e.g. San Francisco, CA"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "Temperature unit",
                "default": "fahrenheit"
            }
        },
        "required": ["location"]
    }))
    .build();
    
    match tools_client.create(weather_tool, None).await {
        Ok(tool) => {
            println!("✓ Created tool: {} ({})", tool.name, tool.id);
            
            // Example 3: Get tool details
            println!("\nExample 3: Getting tool details");
            let retrieved = tools_client.get(&tool.id, None).await?;
            println!("Tool details:");
            println!("  Name: {}", retrieved.name);
            println!("  ID: {}", retrieved.id);
            println!("  Description: {}", retrieved.description);
            println!("  Parameters: {}", serde_json::to_string_pretty(&retrieved.parameters)?);
            
            // Example 4: Update the tool
            println!("\nExample 4: Updating tool description");
            use hume::evi::tools::UpdateToolRequest;
            let update = UpdateToolRequest {
                description: Some("Get current weather and forecast for a location".to_string()),
                ..Default::default()
            };
            
            let updated = tools_client.update(&tool.id, update, None).await?;
            println!("✓ Updated description: {}", updated.description);
            
            // Example 5: List tool versions
            println!("\nExample 5: Listing tool versions");
            let versions = tools_client.list_versions(&tool.id, None, None, None).await?;
            println!("Found {} versions:", versions.tools_page.len());
            for version_opt in versions.tools_page.iter() {
                if let Some(version) = version_opt {
                    println!("  - Version: {}", 
                        version.version_id.as_deref().unwrap_or("unknown"));
                }
            }
            
            // Example 6: Delete the tool
            println!("\nExample 6: Deleting the tool");
            tools_client.delete(&tool.id, None).await?;
            println!("✓ Tool deleted successfully");
        }
        Err(e) => {
            println!("Note: Tool creation failed (possibly already exists): {}", e);
        }
    }
    
    Ok(())
}
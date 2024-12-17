use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;

use atomus_ai_agent::{
    api::TwitterApi,
    services::LLMClient
};

#[tokio::test]
async fn test_api_connections() -> Result<()> {
    dotenv().ok();
    
    // Test Twitter API
    let client = Client::new();
    let twitter_api = TwitterApi::new(
        client.clone(),
        std::env::var("TWITTER_BEARER_TOKEN")?
    );
    
    // Test Twitter API connection
    match twitter_api.test_connection().await {
        Ok(true) => {
            println!("Twitter API test successful!");
        },
        Ok(false) => {
            println!("Twitter API test failed: Received unsuccessful status code");
            return Err(anyhow::anyhow!("Twitter API test failed"));
        },
        Err(e) => {
            println!("Twitter API Error: {:?}", e);
            return Err(e);
        }
    }
    
    // Test Claude API
    let llm_client = LLMClient::new(
        std::env::var("ANTHROPIC_API_KEY")?
    )?;
    
    // Test Claude API connection
    match llm_client.test_connection().await {
        Ok(true) => {
            println!("Claude API test successful!");
        },
        Ok(false) => {
            println!("Claude API test failed: Received unsuccessful status code");
            return Err(anyhow::anyhow!("Claude API test failed"));
        },
        Err(e) => {
            println!("Claude API Error: {:?}", e);
            return Err(e);
        }
    }
    
    println!("All API connections tested successfully!");
    Ok(())
}

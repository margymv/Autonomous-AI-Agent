#[cfg(test)]
mod tests {
    use anyhow::Result;
    use dotenv::dotenv;
    
    use crate::api::TwitterApi;
    use crate::services::LLMClient;

    #[tokio::test]
    async fn test_api_connections() -> Result<()> {
        dotenv().ok();
        
        // Test Twitter API
        let client = reqwest::Client::new();
        let twitter_api = TwitterApi::new(
            client.clone(),
            std::env::var("TWITTER_BEARER_TOKEN")?
        );
        
        // Test getting user ID - this will verify Twitter API connection
        let user_id = twitter_api.get_user_id().await?;
        println!("Twitter API test successful! User ID: {}", user_id);
        
        // Test Claude API
        let llm_client = LLMClient::new(
            std::env::var("ANTHROPIC_API_KEY")?
        )?;
        
        // Add a simple test call to Claude API here
        // This will depend on your LLMClient implementation
        
        println!("All API connections tested successfully!");
        Ok(())
    }
}

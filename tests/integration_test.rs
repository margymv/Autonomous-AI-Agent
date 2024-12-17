use anyhow::Result;
use atomus_ai_agent::{
    api::twitter::TwitterApi,
    services::llm::LLMClient,
};
use reqwest::Client;

#[tokio::test]
async fn test_api_construction() -> Result<()> {
    // Test Twitter API construction
    let client = Client::new();
    let _twitter_api = TwitterApi::new(
        client.clone(),
        "test_key".to_string(),
        "test_secret".to_string(),
        "test_token".to_string(),
        "test_token_secret".to_string(),
    );

    // Test LLM client construction
    let _llm_client = LLMClient::new("test_key".to_string())?;

    // Just verify that construction succeeds
    Ok(())
}

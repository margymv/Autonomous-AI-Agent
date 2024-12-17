#[cfg(test)]
mod tests {
    use anyhow::Result;
    use dotenv::dotenv;
    use tokio::time::Duration;

    use atomus_ai_agent::{
        api::TwitterApi,
        services::llm::LLMClient
    };

    const MAX_RETRIES: u32 = 10;
    const TEST_TIMEOUT: u64 = 300; // 5 minutes timeout

    #[tokio::test]
    async fn test_api_connections() -> Result<()> {
        dotenv().ok();
        
        let consumer_key = std::env::var("TWITTER_CONSUMER_KEY")
            .context("TWITTER_CONSUMER_KEY not found in environment")?;
        let consumer_secret = std::env::var("TWITTER_CONSUMER_SECRET")
            .context("TWITTER_CONSUMER_SECRET not found in environment")?;
        let access_token = std::env::var("TWITTER_ACCESS_TOKEN")
            .context("TWITTER_ACCESS_TOKEN not found in environment")?;
        let access_token_secret = std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .context("TWITTER_ACCESS_TOKEN_SECRET not found in environment")?;
        let anthropic_key = std::env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY not found in environment")?;

        let client = reqwest::Client::new();
        let twitter_api = TwitterApi::new(
            client.clone(),
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        );
        let llm_client = LLMClient::new(anthropic_key)?;

        // Test Twitter API connection
        let twitter_ok = twitter_api.test_connection().await?;
        assert!(twitter_ok, "Twitter API connection failed");

        // Add delay between API calls
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Test Claude API connection
        let claude_ok = llm_client.test_connection().await?;
        assert!(claude_ok, "Claude API connection failed");

        println!("All API connections tested successfully!");
        Ok(())
    }

    #[tokio::test]
    async fn test_tweet_operations() -> Result<()> {
        dotenv().ok();
        
        let consumer_key = std::env::var("TWITTER_CONSUMER_KEY")
            .context("TWITTER_CONSUMER_KEY not found in environment")?;
        let consumer_secret = std::env::var("TWITTER_CONSUMER_SECRET")
            .context("TWITTER_CONSUMER_SECRET not found in environment")?;
        let access_token = std::env::var("TWITTER_ACCESS_TOKEN")
            .context("TWITTER_ACCESS_TOKEN not found in environment")?;
        let access_token_secret = std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .context("TWITTER_ACCESS_TOKEN_SECRET not found in environment")?;
        let anthropic_key = std::env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY not found in environment")?;

        let client = reqwest::Client::new();
        let twitter_api = TwitterApi::new(
            client.clone(),
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        );
        let llm_client = LLMClient::new(anthropic_key)?;

        // Get initial limits
        let (reads, writes) = twitter_api.get_remaining_limits();
        println!("Initial limits - Reads: {}, Writes: {}", reads, writes);
        
        let tweet_id = "1865448594754691313";
        
        // Get the tweet content with retry and longer timeout
        println!("Fetching tweet content...");
        let tweet_text = tokio::time::timeout(
            Duration::from_secs(TEST_TIMEOUT),
            twitter_api.get_tweet_with_retry(tweet_id, MAX_RETRIES)
        ).await??;
        println!("Tweet content: {}", tweet_text);
        
        // Check updated limits after read
        let (reads_after, writes_after) = twitter_api.get_remaining_limits();
        println!("Limits after read - Reads: {}, Writes: {}", reads_after, writes_after);
        assert_eq!(reads_after, reads - 1, "Read count should decrease by 1");
        assert_eq!(writes_after, writes, "Write count should remain unchanged");
        
        // Add delay between operations
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Use Claude to generate a reply
        let context = &[
            "You are replying to a tweet about blockchain technology and DeFi",
            "Keep the reply professional and focused on technical aspects",
            "The reply must be under 280 characters",
            "Avoid generic responses",
            "Focus on BTB Finance's approach to optimizing yields across the crypto landscape",
        ];
        
        println!("Generating reply...");
        let reply = llm_client.generate_reply(&tweet_text, context).await?;
        println!("Generated reply: {}", reply);
        
        // Add delay before reply
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Reply to the tweet with retry and longer timeout
        println!("Sending reply...");
        tokio::time::timeout(
            Duration::from_secs(TEST_TIMEOUT),
            twitter_api.reply_to_tweet_with_retry(&reply, tweet_id, MAX_RETRIES)
        ).await??;
        
        // Check final limits
        let (final_reads, final_writes) = twitter_api.get_remaining_limits();
        println!("Final limits - Reads: {}, Writes: {}", final_reads, final_writes);
        assert_eq!(final_writes, writes - 1, "Write count should decrease by 1");
        
        println!("Successfully completed tweet operations!");
        Ok(())
    }
}

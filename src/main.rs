mod api;
mod core;
mod models;
mod services;
mod utils;

use anyhow::Result;
use dotenv::dotenv;
use tracing::{info, warn};
use std::time::Duration;
use tokio::time::sleep;

use api::TwitterApi;
use services::{LLMClient, WalletManager, get_bearer_token};
use core::BotState;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Atomus AI Twitter Agent");

    // Initialize components
    let client = reqwest::Client::new();
    let twitter_api = TwitterApi::new(
        client,
        std::env::var("TWITTER_API_KEY")?,
        std::env::var("TWITTER_API_SECRET")?,
        std::env::var("TWITTER_ACCESS_TOKEN")?,
        std::env::var("TWITTER_ACCESS_TOKEN_SECRET")?,
    );
    let llm_client = LLMClient::new(
        std::env::var("CLAUDE_API_KEY")?
    )?;
    let wallet_manager = WalletManager::new(
        &std::env::var("WALLET_PRIVATE_KEY")?,
        &std::env::var("ETH_RPC_URL")?
    ).await?;

    // Initialize or load state
    let mut state = BotState::new();

    // Main loop
    loop {
        if let Err(e) = process_mentions(
            &twitter_api,
            &llm_client,
            &wallet_manager,
            &mut state
        ).await {
            warn!("Error processing mentions: {}", e);
        }

        sleep(Duration::from_secs(60)).await;
    }
}

async fn process_mentions(
    twitter_api: &TwitterApi,
    llm_client: &LLMClient,
    _wallet_manager: &WalletManager,
    state: &mut BotState,
) -> Result<()> {
    let mentions = twitter_api.get_mentions(state.last_checked_mention_id.as_deref()).await?;

    for mention in mentions {
        info!("Processing mention: {}", mention.id);

        // Generate reply using LLM
        let author_id = mention.author_id.clone().unwrap_or_default();
        let reply = llm_client.generate_reply(
            &mention.text,
            &[
                &format!("User: {}", author_id),
            ]
        ).await?;

        twitter_api.reply_to_tweet(&reply, &mention.id).await?;

        // Update state
        state.add_conversation(
            mention.id.clone(),
            author_id.clone(),
            mention.text.clone(),
        );
        state.update_user_interaction(author_id);

        // Update last checked mention ID
        state.last_checked_mention_id = Some(mention.id);
    }

    Ok(())
}

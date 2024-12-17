mod api;
mod auth;
mod config;
mod models;
mod state;
mod wallet;
mod llm_client;

use anyhow::Result;
use dotenv::dotenv;
use tracing::{info, warn};
use std::time::Duration;
use tokio::time::sleep;

use crate::api::TwitterApi;
use crate::llm_client::LLMClient;
use crate::wallet::WalletManager;
use crate::state::BotState;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Atomus AI Twitter Agent");

    // Initialize components
    let twitter_api = TwitterApi::new().await?;
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
    wallet_manager: &WalletManager,
    state: &mut BotState,
) -> Result<()> {
    let mentions = twitter_api.get_mentions(state.last_checked_mention_id.as_deref()).await?;

    for mention in mentions {
        info!("Processing mention: {}", mention.id);

        // Get conversation context
        let context: Vec<_> = state.conversation_history
            .iter()
            .map(|entry| entry.content.as_str())
            .collect();

        // Generate reply using LLM
        if let Ok(reply) = llm_client.generate_reply(&mention.text, &context).await {
            twitter_api.reply_to_tweet(&reply, &mention.id).await?;
        }

        // Update state
        state.add_conversation(
            mention.id.clone(),
            mention.author_id.unwrap_or_default(),
            mention.text.clone(),
        );
        state.update_user_interaction(mention.author_id.unwrap_or_default());

        // Update last checked mention ID
        state.last_checked_mention_id = Some(mention.id);
    }

    Ok(())
}

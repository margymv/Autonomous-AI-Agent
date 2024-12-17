use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotState {
    pub last_checked_mention_id: Option<String>,
    pub conversation_history: Vec<ConversationEntry>,
    pub user_interactions: HashMap<String, UserInteraction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub tweet_id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInteraction {
    pub last_interaction: DateTime<Utc>,
    pub interaction_count: u32,
    pub tip_history: Vec<TipTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TipTransaction {
    pub timestamp: DateTime<Utc>,
    pub amount: f64,
    pub transaction_hash: String,
}

impl BotState {
    pub fn new() -> Self {
        Self {
            last_checked_mention_id: None,
            conversation_history: Vec::new(),
            user_interactions: HashMap::new(),
        }
    }

    pub fn add_conversation(&mut self, tweet_id: String, user_id: String, content: String) {
        self.conversation_history.push(ConversationEntry {
            tweet_id,
            user_id,
            content,
            timestamp: Utc::now(),
        });

        // Keep only last 100 conversations
        if self.conversation_history.len() > 100 {
            self.conversation_history.remove(0);
        }
    }

    pub fn update_user_interaction(&mut self, user_id: String) {
        let interaction = self.user_interactions
            .entry(user_id)
            .or_insert_with(|| UserInteraction {
                last_interaction: Utc::now(),
                interaction_count: 0,
                tip_history: Vec::new(),
            });

        interaction.last_interaction = Utc::now();
        interaction.interaction_count += 1;
    }

    pub fn add_tip_transaction(&mut self, user_id: String, amount: f64, transaction_hash: String) {
        if let Some(interaction) = self.user_interactions.get_mut(&user_id) {
            interaction.tip_history.push(TipTransaction {
                timestamp: Utc::now(),
                amount,
                transaction_hash,
            });
        }
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeBase {
    responses: HashMap<String, Vec<String>>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        let mut responses = HashMap::new();
        
        // Add some sample responses for different topics
        responses.insert(
            "$BTB".to_string(),
            vec![
                "Interesting analysis on $BTB! The market dynamics look promising.".to_string(),
                "Thanks for sharing your insights on $BTB. Have you considered the recent developments?".to_string(),
                "Great point about $BTB! The community's growth has been remarkable.".to_string(),
            ],
        );

        Self { responses }
    }

    pub fn get_response(&self, topic: &str) -> Option<&String> {
        self.responses.get(topic)
            .and_then(|responses| responses.first())
    }

    pub fn add_response(&mut self, topic: String, response: String) {
        self.responses
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(response);
    }
}

use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub struct LLMClient {
    client: Client,
    api_key: String,
}

impl LLMClient {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, api_key })
    }

    pub async fn generate_reply(&self, tweet_text: &str, context: &[&str]) -> Result<String> {
        let mut messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful AI assistant managing a Twitter account. \
                         Your responses should be concise, friendly, and appropriate for Twitter. \
                         Never reveal sensitive information or private keys. \
                         Focus on providing value while maintaining a professional tone."
                         .to_string(),
            }
        ];

        // Add context messages
        for ctx in context {
            messages.push(Message {
                role: "assistant".to_string(),
                content: ctx.to_string(),
            });
        }

        // Add the current tweet
        messages.push(Message {
            role: "user".to_string(),
            content: tweet_text.to_string(),
        });

        let request = ClaudeRequest {
            model: "gpt-4".to_string(), // or appropriate Claude model
            messages,
            max_tokens: 280, // Twitter limit
            temperature: 0.7,
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude")?;

        let response: ClaudeResponse = response.json().await
            .context("Failed to parse Claude response")?;

        let reply = response.choices.first()
            .context("No response from Claude")?
            .message.content.clone();

        info!("Generated reply: {}", reply);
        Ok(reply)
    }
}

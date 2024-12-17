use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::info;

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    system: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
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
        let system = "You are a helpful AI assistant managing a Twitter account. \
                     Your responses should be concise, friendly, and appropriate for Twitter. \
                     Never reveal sensitive information or private keys. \
                     Focus on providing value while maintaining a professional tone.";

        let mut messages = Vec::new();

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
            model: "claude-3-opus-20240229".to_string(),
            system: system.to_string(),
            messages,
            max_tokens: 280, // Twitter limit
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude")?;

        let response_text = response.text().await?;
        println!("Claude response: {}", response_text);

        let response: ClaudeResponse = serde_json::from_str(&response_text)
            .context("Failed to parse Claude response")?;

        let reply = response.content.first()
            .context("No response from Claude")?
            .text.clone();

        info!("Generated reply: {}", reply);
        Ok(reply)
    }

    pub async fn test_connection(&self) -> Result<bool> {
        let url = "https://api.anthropic.com/v1/messages";
        
        let response = self.client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": "claude-3-opus-20240229",
                "max_tokens": 1,
                "messages": [{
                    "role": "user",
                    "content": "Hi"
                }]
            }))
            .send()
            .await?;

        let status = response.status();
        println!("Claude API Response status: {}", status);
        let text = response.text().await?;
        println!("Claude API Response body: {}", text);
        
        Ok(status.is_success())
    }
}

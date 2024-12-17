use anyhow::{Result, anyhow, Context};
use reqwest::Client;
use tracing::info;
use base64::Engine;

use crate::models::TokenResponse;
use crate::config::TWITTER_AUTH_URL;

pub async fn get_bearer_token(client: &Client, api_key: &str, api_secret: &str) -> Result<String> {
    let auth = format!("{}:{}", api_key, api_secret);
    let auth_header = format!(
        "Basic {}",
        base64::engine::general_purpose::STANDARD.encode(auth)
    );

    info!("Requesting bearer token...");
    attempt_token_request(client, &auth_header).await
}

async fn attempt_token_request(client: &Client, auth_header: &str) -> Result<String> {
    let response = client
        .post(TWITTER_AUTH_URL)
        .header("Authorization", auth_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", &std::env::var("TWITTER_API_KEY").context("TWITTER_API_KEY must be set")?),
            ("client_secret", &std::env::var("TWITTER_API_SECRET").context("TWITTER_API_SECRET must be set")?),
            ("client_type", "service_client"),
            ("scope", "tweet.read tweet.write users.read")
        ])
        .send()
        .await
        .context("Failed to send bearer token request")?;

    let status = response.status();
    let headers = response.headers().clone();
    let response_text = response.text().await
        .context("Failed to get response text")?;

    info!("Token response status: {}, headers: {:?}", status, headers);
    if !status.is_success() {
        return Err(anyhow!("Failed to get bearer token. Status: {}, Response: {}", status, response_text));
    }

    let token: TokenResponse = serde_json::from_str(&response_text)
        .context("Failed to parse token response")?;

    Ok(token.access_token)
}

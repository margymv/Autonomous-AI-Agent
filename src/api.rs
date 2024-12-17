use anyhow::{Result, Context};
use reqwest::Client;
use tracing::{info, warn};
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;

use crate::models::{Tweet, TwitterResponse, TweetCreate};
use crate::config::{TWITTER_API_BASE, MAX_RETRIES, RETRY_DELAY};

pub struct TwitterApi {
    client: Client,
    bearer_token: String,
}

impl TwitterApi {
    pub fn new(client: Client, bearer_token: String) -> Self {
        Self {
            client,
            bearer_token,
        }
    }

    pub async fn search_and_interact(&self, topic: &str) -> Result<()> {
        let query = format!("{} -is:retweet -is:reply lang:en", topic);
        let url = format!("{}/tweets/search/recent", TWITTER_API_BASE);

        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .query(&[
                ("query", query),
                ("tweet.fields", "author_id".into()),
                ("max_results", "10".into()),
            ])
            .send()
            .await?;

        let tweets: TwitterResponse<Tweet> = response.json().await?;

        if let Some(tweet_list) = tweets.data {
            for tweet in tweet_list {
                info!("Processing tweet: {}", tweet.id);
                if let Some(reply) = self.generate_reply(&tweet.text)? {
                    self.reply_to_tweet(&reply, &tweet.id).await?;
                }
                self.like_tweet(&tweet.id).await?;
            }
        }

        Ok(())
    }

    pub async fn like_tweet(&self, tweet_id: &str) -> Result<()> {
        let url = format!("{}/users/{}/likes", TWITTER_API_BASE, self.get_user_id().await?);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.bearer_token)
            .json(&serde_json::json!({ "tweet_id": tweet_id }))
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Failed to like tweet: {}", response.text().await?);
        }

        Ok(())
    }

    pub async fn reply_to_tweet(&self, text: &str, reply_to_id: &str) -> Result<()> {
        let url = format!("{}/tweets", TWITTER_API_BASE);
        
        let tweet = TweetCreate {
            text: text.to_string(),
            reply: Some(crate::models::ReplySettings {
                in_reply_to_tweet_id: reply_to_id.to_string(),
            }),
        };

        let response = self.client
            .post(&url)
            .bearer_auth(&self.bearer_token)
            .json(&tweet)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Failed to reply to tweet: {}", response.text().await?);
        }

        Ok(())
    }

    pub async fn get_user_id(&self) -> Result<String> {
        let url = format!("{}/users/me", TWITTER_API_BASE);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        data["data"]["id"]
            .as_str()
            .map(String::from)
            .context("Failed to get user ID")
    }

    pub fn generate_reply(&self, tweet_text: &str) -> Result<Option<String>> {
        // TODO: Implement AI-based reply generation
        Ok(Some(format!("Thanks for sharing your thoughts about: {}", tweet_text)))
    }

    pub async fn retry_with_backoff<F, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Result<T>,
    {
        let mut retries = 0;
        loop {
            match f() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if retries >= MAX_RETRIES {
                        return Err(e);
                    }
                    retries += 1;
                    let delay = RETRY_DELAY * (1 << retries);
                    let jitter = rand::thread_rng().gen_range(0..1000) as u64;
                    warn!("Operation failed, attempt {}/{}. Retrying in {} seconds...", 
                          retries, MAX_RETRIES, delay);
                    sleep(Duration::from_secs(delay) + Duration::from_millis(jitter)).await;
                }
            }
        }
    }
}

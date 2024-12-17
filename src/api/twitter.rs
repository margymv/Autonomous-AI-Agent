use crate::services::rate_limiter::RateLimiter;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::{Result, anyhow, Context};
use rand::Rng;
use oauth1_request::{OAuthRequest, HmacSha1};

use crate::models::{Tweet, TwitterResponse};
use crate::utils::config::{TWITTER_API_BASE, MAX_RETRIES, RETRY_DELAY};

const BASE_DELAY: u64 = 15; // Base delay in seconds

pub struct TwitterApi {
    client: Client,
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    rate_limiter: Arc<RateLimiter>,
}

impl TwitterApi {
    pub fn new(
        client: Client,
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Self {
            client,
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
            rate_limiter: Arc::new(RateLimiter::new(100, 500)),
        }
    }

    async fn calculate_backoff(retries: u32) -> Duration {
        let base = BASE_DELAY as f64;
        let max_jitter = (base * 0.2) as u64; // 20% jitter
        let jitter = rand::thread_rng().gen_range(0..=max_jitter);
        let delay = (base * (2_f64.powf(retries as f64))) as u64;
        Duration::from_secs(delay) + Duration::from_secs(jitter)
    }

    async fn sign_request(&self, method: &str, url: &str) -> Result<String> {
        let oauth = OAuthRequest::new(
            method,
            url,
            &self.consumer_key,
            &self.consumer_secret,
            Some((&self.access_token, &self.access_token_secret)),
        );
        
        Ok(oauth.sign_header::<HmacSha1>())
    }

    pub async fn get_tweet_with_retry(&self, tweet_id: &str, max_retries: u32) -> Result<String> {
        // Check rate limits before proceeding
        self.rate_limiter.check_and_update_limits().await
            .map_err(|e| anyhow!(e))?;

        let mut retries = 0;

        while retries < max_retries {
            match self.get_tweet(tweet_id).await {
                Ok(tweet) => {
                    self.rate_limiter.increment_read();
                    return Ok(tweet);
                }
                Err(e) => {
                    if retries == max_retries - 1 {
                        return Err(e);
                    }

                    let error_str = e.to_string();
                    let backoff = Self::calculate_backoff(retries).await;
                    
                    if error_str.contains("429") {
                        println!("Rate limited, waiting {:?} before retry {}/{}", backoff, retries + 1, max_retries);
                    } else {
                        println!("Error getting tweet, retrying in {:?}: {:?}", backoff, e);
                    }
                    
                    sleep(backoff).await;
                    retries += 1;
                }
            }
        }
        
        Err(anyhow!("Max retries exceeded"))
    }

    pub async fn reply_to_tweet_with_retry(&self, reply: &str, tweet_id: &str, max_retries: u32) -> Result<()> {
        // Check rate limits before proceeding
        self.rate_limiter.check_and_update_limits().await
            .map_err(|e| anyhow!(e))?;

        let mut retries = 0;

        while retries < max_retries {
            match self.reply_to_tweet(reply, tweet_id).await {
                Ok(_) => {
                    self.rate_limiter.increment_write();
                    return Ok(());
                }
                Err(e) => {
                    if retries == max_retries - 1 {
                        return Err(e);
                    }

                    let error_str = e.to_string();
                    let backoff = Self::calculate_backoff(retries).await;
                    
                    if error_str.contains("429") {
                        println!("Rate limited, waiting {:?} before retry {}/{}", backoff, retries + 1, max_retries);
                    } else {
                        println!("Error replying to tweet, retrying in {:?}: {:?}", backoff, e);
                    }
                    
                    sleep(backoff).await;
                    retries += 1;
                }
            }
        }
        
        Err(anyhow!("Max retries exceeded"))
    }

    pub async fn reply_to_tweet(&self, text: &str, reply_to_id: &str) -> Result<()> {
        let url = format!("{}/tweets", TWITTER_API_BASE);
        let auth_header = self.sign_request("POST", &url).await?;

        let body = serde_json::json!({
            "text": text,
            "reply": {
                "in_reply_to_tweet_id": reply_to_id
            }
        });

        let response = self.client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to reply to tweet (status {}): {}", 
                response.status(), error_text));
        }

        Ok(())
    }

    pub fn get_remaining_limits(&self) -> (u32, u32) {
        (
            self.rate_limiter.get_remaining_reads(),
            self.rate_limiter.get_remaining_writes()
        )
    }

    pub async fn search_and_interact(&self, topic: &str) -> Result<()> {
        let query = format!("{} -is:retweet -is:reply lang:en", topic);
        let url = format!("{}/tweets/search/recent", TWITTER_API_BASE);

        let auth_header = self.sign_request("GET", &url).await?;

        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
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
                println!("Processing tweet: {}", tweet.id);
                if let Some(reply) = self.generate_reply(&tweet.text)? {
                    self.reply_to_tweet_with_retry(&reply, &tweet.id, MAX_RETRIES).await?;
                }
                self.like_tweet(&tweet.id).await?;
            }
        }

        Ok(())
    }

    pub async fn like_tweet(&self, tweet_id: &str) -> Result<()> {
        let url = format!("{}/users/{}/likes", TWITTER_API_BASE, self.get_user_id().await?);
        let auth_header = self.sign_request("POST", &url).await?;
        
        let response = self.client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&serde_json::json!({ "tweet_id": tweet_id }))
            .send()
            .await?;

        if !response.status().is_success() {
            println!("Failed to like tweet: {}", response.text().await?);
        }

        Ok(())
    }

    pub async fn get_user_id(&self) -> Result<String> {
        let url = format!("{}/users/me", TWITTER_API_BASE);
        let auth_header = self.sign_request("GET", &url).await?;
        
        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        println!("Response status: {}", response.status());
        let text = response.text().await?;
        println!("Response body: {}", text);
        
        let data: serde_json::Value = serde_json::from_str(&text)?;
        data["data"]["id"]
            .as_str()
            .map(String::from)
            .context("Failed to get user ID")
    }

    pub async fn get_mentions(&self, since_id: Option<&str>) -> Result<Vec<Tweet>> {
        let url = format!("{}/tweets/search/recent", TWITTER_API_BASE);
        let user_id = self.get_user_id().await?;
        let query = format!("@{}", user_id);

        let auth_header = self.sign_request("GET", &url).await?;

        let mut params = vec![
            ("query", query.as_str()),
            ("tweet.fields", "author_id,referenced_tweets"),
            ("max_results", "100"),
        ];

        if let Some(id) = since_id {
            params.push(("since_id", id));
        }

        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
            .query(&params)
            .send()
            .await
            .context("Failed to fetch mentions")?;

        let tweets: TwitterResponse<Tweet> = response.json().await
            .context("Failed to parse mentions response")?;

        Ok(tweets.data.unwrap_or_default())
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
                    println!("Operation failed, attempt {}/{}. Retrying in {} seconds...", 
                          retries, MAX_RETRIES, delay);
                    sleep(Duration::from_secs(delay) + Duration::from_millis(jitter)).await;
                }
            }
        }
    }

    pub async fn test_connection(&self) -> Result<bool> {
        let url = format!("{}/tweets/search/recent?query=rust", TWITTER_API_BASE);
        let auth_header = self.sign_request("GET", &url).await?;
        
        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;
            
        let status = response.status();
        println!("Response status: {}", status);
        
        // Get response body for debugging
        let body = response.text().await?;
        println!("Response body: {}", body);
        
        // Consider both 200 OK and 429 Rate Limit as successful connections
        // since 429 confirms we can reach the API but are just rate limited
        Ok(status.is_success() || status.as_u16() == 429)
    }

    pub async fn get_tweet(&self, tweet_id: &str) -> Result<String> {
        let url = format!("https://api.twitter.com/2/tweets/{}", tweet_id);
        let auth_header = self.sign_request("GET", &url).await?;
        
        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;
            
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to get tweet (status {}): {}", status, error_text));
        }
        
        let body: serde_json::Value = response.json().await?;
        let tweet_text = body["data"]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Tweet text not found in response: {:?}", body))?
            .to_string();
            
        Ok(tweet_text)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TwitterResponse<T> {
    pub data: Option<Vec<T>>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub newest_id: Option<String>,
    pub oldest_id: Option<String>,
    pub result_count: Option<i32>,
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub author_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TweetCreate {
    pub text: String,
    pub reply: Option<ReplySettings>,
}

#[derive(Debug, Serialize)]
pub struct ReplySettings {
    pub in_reply_to_tweet_id: String,
}

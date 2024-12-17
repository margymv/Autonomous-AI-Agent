// Constants for API configuration
pub const MAX_RETRIES: u32 = 5;
pub const RETRY_DELAY: u64 = 10;
pub const REQUEST_TIMEOUT: u64 = 30;
pub const CONNECT_TIMEOUT: u64 = 20;
pub const POOL_TIMEOUT: u64 = 90;

// API endpoints
pub const TWITTER_API_BASE: &str = "https://api.twitter.com/2";
pub const TWITTER_AUTH_URL: &str = "https://api.twitter.com/2/oauth2/token";

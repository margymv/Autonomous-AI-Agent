use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use chrono::{DateTime, Utc, Datelike};
use tokio::sync::Mutex;

pub struct RateLimiter {
    read_count: AtomicU32,
    write_count: AtomicU32,
    last_reset: Arc<Mutex<DateTime<Utc>>>,
    max_reads: u32,
    max_writes: u32,
}

impl RateLimiter {
    pub fn new(max_reads: u32, max_writes: u32) -> Self {
        Self {
            read_count: AtomicU32::new(0),
            write_count: AtomicU32::new(0),
            last_reset: Arc::new(Mutex::new(Utc::now())),
            max_reads,
            max_writes,
        }
    }

    pub async fn check_and_update_limits(&self) -> Result<(), String> {
        let mut last_reset = self.last_reset.lock().await;
        let now = Utc::now();
        
        // Reset counters if a month has passed
        if now.month() != last_reset.month() || now.year() != last_reset.year() {
            self.read_count.store(0, Ordering::SeqCst);
            self.write_count.store(0, Ordering::SeqCst);
            *last_reset = now;
        }

        let reads = self.read_count.load(Ordering::SeqCst);
        let writes = self.write_count.load(Ordering::SeqCst);

        if reads >= self.max_reads {
            return Err("Monthly read limit exceeded".to_string());
        }
        if writes >= self.max_writes {
            return Err("Monthly write limit exceeded".to_string());
        }

        Ok(())
    }

    pub fn increment_read(&self) {
        self.read_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_write(&self) {
        self.write_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_remaining_reads(&self) -> u32 {
        self.max_reads - self.read_count.load(Ordering::SeqCst)
    }

    pub fn get_remaining_writes(&self) -> u32 {
        self.max_writes - self.write_count.load(Ordering::SeqCst)
    }
}

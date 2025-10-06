//! Rate limiting for downloads.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Simple token bucket rate limiter.
#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
    bytes_per_second: u64,
}

struct RateLimiterState {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter with the given bytes per second limit.
    pub fn new(bytes_per_second: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: bytes_per_second as f64,
                last_refill: Instant::now(),
            })),
            bytes_per_second,
        }
    }

    /// Wait until sufficient capacity is available for the given number of bytes.
    pub async fn wait_for_capacity(&self, bytes: usize) {
        let bytes = bytes as f64;
        
        loop {
            let mut state = self.state.lock().await;
            
            // Refill tokens based on elapsed time
            let now = Instant::now();
            let elapsed = now.duration_since(state.last_refill).as_secs_f64();
            state.tokens = (state.tokens + elapsed * self.bytes_per_second as f64)
                .min(self.bytes_per_second as f64);
            state.last_refill = now;

            // Check if we have enough tokens
            if state.tokens >= bytes {
                state.tokens -= bytes;
                return;
            }

            // Calculate how long to wait for more tokens
            let deficit = bytes - state.tokens;
            let wait_duration = Duration::from_secs_f64(deficit / self.bytes_per_second as f64);
            
            drop(state); // Release lock before sleeping
            tokio::time::sleep(wait_duration).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(1000); // 1000 bytes per second
        
        let start = Instant::now();
        limiter.wait_for_capacity(500).await;
        let elapsed = start.elapsed();
        
        // Should be nearly instant for first request
        assert!(elapsed.as_millis() < 100);
    }

    #[tokio::test]
    async fn test_rate_limiter_multiple_requests() {
        let limiter = RateLimiter::new(1000); // 1000 bytes per second
        
        // First request should be instant
        limiter.wait_for_capacity(500).await;
        
        // Second request should wait
        let start = Instant::now();
        limiter.wait_for_capacity(600).await;
        let elapsed = start.elapsed();
        
        // Should wait for tokens to refill
        assert!(elapsed.as_millis() >= 100);
    }
}

use diem_rate_limiter::rate_limit::TokenBucketRateLimiter;
use std::time::Instant;

pub type RpcTokenBucketLimiter<'a> = TokenBucketRateLimiter<&'a str>;

struct RpcRateLimiterConfig {
    initial_fill_rate_pct: u8,
    bucket_size: usize,        // how many tokens in one bucket
    global_bucket_size: usize, // how many tokens in global bucket
}

struct RpcRateLimiter<'a> {
    config: RpcRateLimiterConfig,
    buckets: RpcTokenBucketLimiter<'a>,
    global_buckets: RpcTokenBucketLimiter<'a>,
}
const GLOBAL_KEY: &str = "global";
impl<'a> RpcRateLimiter<'a> {
    fn new(config: RpcRateLimiterConfig) -> Self {
        let buckets = TokenBucketRateLimiter::new(
            "rpc_ratelimiter",
            String::from("rpc_ratelimiter"),
            config.initial_fill_rate_pct,
            config.bucket_size,
            1, // replenish bucket with 1 token per second
            None,
        );
        let global_buckets = TokenBucketRateLimiter::new(
            "rpc_ratelimiter",
            String::from("rpc_ratelimiter"),
            config.initial_fill_rate_pct,
            config.global_bucket_size,
            1, // replenish bucket with 1 token per second
            None,
        );
        RpcRateLimiter {
            config,
            buckets,
            global_buckets,
        }
    }

    pub fn acquire_all_tokens(
        &mut self,
        key: &'a str,
        requested: usize,
    ) -> Result<(), Option<Instant>> {
        if requested == 0 {
            return Ok(());
        } else if requested > self.config.bucket_size {
            return Err(None);
        }

        // acquire bucket tokens
        if let Err(_) = self
            .buckets
            .bucket(key)
            .lock()
            .acquire_all_tokens(requested)
        {
            // borrow to ensure that at last 50% of global limit left
            let sz = self.config.global_bucket_size;
            let to_save = sz / 2;
            if requested > sz - to_save {
                // request is more that can possibly allow
                return Err(None as Option<Instant>);
            }
            // request that many tickets and return to_save back
            let new_request = requested + to_save;
            let borrowed_tokens = self
                .global_buckets
                .bucket(GLOBAL_KEY)
                .lock()
                .acquire_all_tokens(new_request)?;
            // return extra tokens borrowed
            self.global_buckets
                .bucket(GLOBAL_KEY)
                .lock()
                .return_tokens(to_save);
            return Ok(borrowed_tokens);
        }

        // acquire global tokens
        let global_tokens = self
            .global_buckets
            .bucket(GLOBAL_KEY)
            .lock()
            .acquire_all_tokens(requested);
        if let Err(_) = global_tokens {
            // cannot acquire global tokens, return bucket tokens
            self.buckets.bucket(key).lock().return_tokens(requested);
        }
        return global_tokens;
    }
}

#[cfg(test)]
mod tests {
    use crate::{RpcRateLimiter, RpcRateLimiterConfig};
    use std::{thread, time};

    // rate limit with single token per sec
    #[test]
    fn single_token() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 1,
            global_bucket_size: 1,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens("aa", 1)
            .expect("Should be successful");
        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens("aa", 1)
            .expect("Should be successful");
        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");
    }
    // test with multiple tokens per bucket
    #[test]
    fn multiple_tokens() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 10,
            global_bucket_size: 10,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens("aa", 10)
            .expect("Should be successful");
        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens("aa", 1)
            .expect("Should be successful");
        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");
    }
    // 2 buckets with 5 tokens each and global limit 10
    #[test]
    fn multiple_buckets() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 5,
            global_bucket_size: 10,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens("aa", 5)
            .expect("Should be successful");
        rl.acquire_all_tokens("bb", 5)
            .expect("Should be successful");

        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");
        rl.acquire_all_tokens("bb", 1).expect_err("Expected error");

        // wait for 2 seconds
        thread::sleep(time::Duration::from_millis(2000));
        rl.acquire_all_tokens("aa", 1)
            .expect("Should be successful");
        rl.acquire_all_tokens("bb", 1)
            .expect("Should be successful");

        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");
        rl.acquire_all_tokens("bb", 1).expect_err("Expected error");
    }

    //3 buckets with 5 tokens each and global limit 12
    // one cline tborrows tokens from the other
    #[test]
    fn borrow_tokens() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 5,
            global_bucket_size: 12,
        };
        let mut rl = RpcRateLimiter::new(config);
        // cannot borrow 6, since 6/12 <= 0.5
        rl.acquire_all_tokens("aa", 5)
            .expect("Should be successful");
        rl.acquire_all_tokens("aa", 1)
            .expect("Should be successful");
        // but no more
        rl.acquire_all_tokens("aa", 1).expect_err("Expected error");
        // b can  borrow 5 tokens
        rl.acquire_all_tokens("bb", 5)
            .expect("Should be successful");
        // but can not borrow  any extra
        rl.acquire_all_tokens("bb", 1).expect_err("Expected error");
        // c cannot  borrow 5 tokens
        rl.acquire_all_tokens("cc", 5).expect_err("Expected error");
        // but can borrow 1 remaining
        rl.acquire_all_tokens("c", 1).expect("Should be successful");
    }
}

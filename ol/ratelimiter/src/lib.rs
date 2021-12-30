mod rate_limit;
use rate_limit::TokenBucketRateLimiter;
use std::time::Instant;

pub type RpcTokenBucketLimiter = TokenBucketRateLimiter<String>;

pub struct RpcRateLimiterConfig {
    pub initial_fill_rate_pct: u8,
    pub bucket_size: usize,        // how many tokens in one bucket
    pub global_bucket_size: usize, // how many tokens in global bucket
    pub fill_rate_tps: f64, // tokens per seconds
}

pub struct RpcRateLimiter {
    config: RpcRateLimiterConfig,
    buckets: RpcTokenBucketLimiter,
    global_buckets: RpcTokenBucketLimiter,
}
const GLOBAL_KEY: &str = "global";
impl RpcRateLimiter {
    pub fn new(config: RpcRateLimiterConfig) -> Self {
        let buckets = TokenBucketRateLimiter::new(
            "rpc_ratelimiter",
            String::from("rpc_ratelimiter"),
            config.initial_fill_rate_pct,
            config.bucket_size,
            config.fill_rate_tps, // replenish bucket with fill_rate_tps per second
            None,
        );
        let global_buckets = TokenBucketRateLimiter::new(
            "rpc_ratelimiter",
            String::from("rpc_ratelimiter"),
            config.initial_fill_rate_pct,
            config.global_bucket_size,
            config.fill_rate_tps, // replenish bucket with fill_rate_tps token per second
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
        key: String,
        requested: usize,
    ) -> Result<(), Option<Instant>> {
        if requested == 0 {
            return Ok(());
        } else if requested > self.config.bucket_size {
            return Err(None);
        }

        let global_key = String::from(GLOBAL_KEY);
        // acquire bucket tokens
        if let Err(_) = self
            .buckets
            .bucket(key.clone())
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
                .bucket(global_key.clone())
                .lock()
                .acquire_all_tokens(new_request)?;
            // return extra tokens borrowed
            self.global_buckets
                .bucket(global_key.clone())
                .lock()
                .return_tokens(to_save);
            return Ok(borrowed_tokens);
        }

        // acquire global tokens
        let global_tokens = self
            .global_buckets
            .bucket(global_key.clone())
            .lock()
            .acquire_all_tokens(requested);
        if let Err(_) = global_tokens {
            // cannot acquire global tokens, return bucket tokens
            self.buckets
                .bucket(key.clone())
                .lock()
                .return_tokens(requested);
        }
        return global_tokens;
    }
}

#[cfg(test)]
mod tests {
    use crate::{RpcRateLimiter, RpcRateLimiterConfig};
    use std::{thread, time};
/*
    // rate limit with single token per sec
    #[test]
    fn single_token() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 1,
            global_bucket_size: 1,
            fill_rate_tps: 1.0,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
    }
    // test with multiple tokens per bucket
    #[test]
    fn multiple_tokens() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 10,
            global_bucket_size: 10,
            fill_rate_tps: 1.0,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens(String::from("aa"), 10)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
    }
    // 2 buckets with 5 tokens each and global limit 10
    #[test]
    fn multiple_buckets() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 5,
            global_bucket_size: 10,
            fill_rate_tps: 1.0,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens(String::from("aa"), 5)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("bb"), 5)
            .expect("Should be successful");

        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
        rl.acquire_all_tokens(String::from("bb"), 1)
            .expect_err("Expected error");

        // wait for 2 seconds
        thread::sleep(time::Duration::from_millis(2000));
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("bb"), 1)
            .expect("Should be successful");

        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
        rl.acquire_all_tokens(String::from("bb"), 1)
            .expect_err("Expected error");
    }

    //3 buckets with 5 tokens each and global limit 12
    // one cline tborrows tokens from the other
    #[test]
    fn can_borrow_tokens() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 5,
            global_bucket_size: 12,
            fill_rate_tps: 1.0,
        };
        let mut rl = RpcRateLimiter::new(config);
        // cannot borrow 6, since 6/12 <= 0.5
        rl.acquire_all_tokens(String::from("aa"), 5)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        // but no more
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
        // b can  borrow 5 tokens
        rl.acquire_all_tokens(String::from("bb"), 5)
            .expect("Should be successful");
        // but can not borrow  any extra
        rl.acquire_all_tokens(String::from("bb"), 1)
            .expect_err("Expected error");
        // c cannot  borrow 5 tokens
        rl.acquire_all_tokens(String::from("cc"), 5)
            .expect_err("Expected error");
        // but can borrow 1 remaining
        rl.acquire_all_tokens(String::from("cc"), 1)
            .expect("Should be successful");
    }

    //3 buckets with 5 tokens each and global limit 10
    // ensure that global limit is honored
    #[test]
    fn global_limit_preserved() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 5,
            global_bucket_size: 10,
            fill_rate_tps: 1.0,
        };
        let mut rl = RpcRateLimiter::new(config);
        // cannot borrow 6, since 6/12 <= 0.5
        rl.acquire_all_tokens(String::from("aa"), 5)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("bb"), 5)
            .expect("Should be successful");
        // c has no tickets
        rl.acquire_all_tokens(String::from("cc"), 1)
            .expect_err("Expected error");

        // wait for 1 seconds
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens(String::from("cc"), 1)
            .expect("Should be successful");
    }

 */
    // fill rate less than 1 tps works
    #[test]
    fn fill_rate_less_1tps() {
        let config = RpcRateLimiterConfig {
            initial_fill_rate_pct: 100,
            bucket_size: 1,
            global_bucket_size: 1,
            fill_rate_tps: 0.5,
        };
        let mut rl = RpcRateLimiter::new(config);
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");

        // wait for 1 second
        thread::sleep(time::Duration::from_millis(1000));
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect("Should be successful");
        rl.acquire_all_tokens(String::from("aa"), 1)
            .expect_err("Expected error");
    }

}

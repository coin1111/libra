# Rate Limiter for Write Transactions For Fullnode

## Goal
Rate limiter restricts number of write transactions send via rpc port 8080. It's current purpose is to 
control number of proofs submitted by miners.

## Configuration
fullnode.node.yaml
```
json_rpc:
  address: 127.0.0.1:8080
  rpc_ratelimit_enabled: false
  bucket_size: 1 # number of tokens allowed per caller
  global_bucket_size: 5 # number of tokens allowed for all callers
  fill_rate_tps: 0.3 # token refill rate tokens/sec (tps)
```
## Functionality
Every write transaction (Submit) is subject to a rate limit. Sender account id is used as a key.
Each request is requires one token. If a token is available, access is allowed, otherwise error is returned.
Each request also requires a token from a global bucket. For example if caller bucket capacity is one token and global capacity is 5,
then only the first 5 callers are allowed, the rest is bloked despide them having available tokens in their buckets.

Rate limiter allows borrowing. E.g. if caller used up its tockens and global buket still has available tokens,
the caller can take more tokens as long as global bucket has at least 50% remaining capacity. For example if 
global bucket is 5, a caller can take 2 tokens (2/5 < 50%).
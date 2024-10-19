# Marketeer is a libarary to query exchanges and indexers in pure Rust
Currently supported:

- nonkyc

Planning to support:

- coingecko

# Authentication
In order to be able to call routes that require an API key,
place your keys in environment variables.

```rust
    let public_key = env::var("NON_KYC_PUBLIC_KEY").unwrap_or("0x00".to_string());
    let secret_key = env::var("NON_KYC_SECRET_KEY").unwrap_or("0x00".to_string());
```
# tokio-retry2

Forked from https://github.com/srijs/rust-tokio-retry to keep it up-to-date

Extensible, asynchronous retry behaviours for the ecosystem of [tokio](https://tokio.rs/) libraries.

[![Crates.io](https://img.shields.io/crates/v/tokio-retry2.svg)](https://crates.io/crates/tokio-retry2)
[![dependency status](https://deps.rs/repo/github/naomijub/tokio-retry/status.svg)](https://deps.rs/repo/github/namijub/tokio-retry)
[![codecov](https://codecov.io/gh/naomijub/tokio-retry/branch/main/graph/badge.svg?token=4VMVTZTN8A)](https://codecov.io/gh/naomijub/tokio-retry)

[Documentation](https://docs.rs/tokio-retry2)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-retry2 = { version = "0.5", features = ["jitter"] }
```

## Examples

```rust
use tokio_retry2::{Retry, RetryError};
use tokio_retry2::strategy::{ExponentialBackoff, jitter, MaxInterval};

async fn action() -> Result<u64, ()> {
    // do some real-world stuff here...
    RetryError::to_transient(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .factor(1) // multiplication factor applied to deplay
        .max_delay_millis(100) // set max delay between retries to 500ms
        .max_interval(10000) // set max interval to 10 seconds
        .map(jitter) // add jitter to delays
        .take(3);    // limit to 3 retries

    let result = Retry::spawn(retry_strategy, action).await?;

    Ok(())
}
```

Or, to retry with a notification function:

```rust
use tokio_retry2::{Retry, RetryError};
use tokio_retry2::strategy::{ExponentialBackoff, jitter, MaxInterval};

async fn action() -> Result<u64, std::io::Error> {
    // do some real-world stuff here...
    RetryError::to_permanent(()) // Early exits on this error
}

fn notify(err: &std::io::Error, duration: std::time::Duration) {
    tracing::info!("Error {err:?} occurred at {duration:?}");
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .factor(1) // multiplication factor applied to deplay
        .max_delay_millis(100) // set max delay between retries to 500ms
        .max_interval(10000) // set max interval to 10 seconds
        .map(jitter) // add jitter to delays
        .take(3);    // limit to 3 retries

    let result = Retry::spawn_notify(retry_strategy, action, notify).await?;

    Ok(())
}
```

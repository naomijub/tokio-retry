# Changelog

## Version 0.8.0 [#22](https://github.com/naomijub/tokio-retry/pull/22)

- Implemented Notify for Box<dyn Notify> to support trait objects
- Modified spawn_notify to accept generic Notify implementations
- Added EmptyNotify for default no-op behavior
- Exported Notify trait from library root
- Tighten clippy behaviour [#23](https://github.com/naomijub/tokio-retry/pull/23)

## Version 0.7.0

- Breaking: Increased minimum required version to 1.88.
- Internal API change.

## Version 0.6.0 [#13](https://github.com/naomijub/tokio-retry/pull/13)

- Breaking change: Updated to `edition 2024`.

## Version 0.5.6

- Added `ExponentialFactorBackoff`, where the exponential value is the factor and not the duration.

## Version 0.5.5 [#9](https://github.com/naomijub/tokio-retry/pull/9)

- Include feature `tracing` to inform when a `max_duration` or `max_delay` has been reached.
- Fixed bug that `retry_after` duration was not overwriting the strategy duration.
- `jitter` ranges between 50% and 150% of initial duration.
- Adds `jitter_range` for custom `map` jitter ranges.

## Version 0.5.4

- Exports `MaxIntervalIterator` as standard `strategy` struct.

## Version 0.5.3

- Documentation fix.
- Fixes Transient duration behavior, overwriting original duration.


## Version 0.5.2 @ [#8](https://github.com/naomijub/tokio-retry/pull/8)

- Add implicit conversion from `Result<T, E>` to `RetryResult<T, E>` a type enabled by feature to handle implicit type conversions from `E` to `RetryError<E>::Transient`. Same for `Option<T>`.
- Adds `PartialEq` between  `Result<T, RetryError<E>>` and `RetryResult<T, E>` for `T` and `E` bounded by `PartialEq`.
- Cargo update.

## Version 0.5.1 @ [#7](https://github.com/naomijub/tokio-retry/pull/7)
> [#6](https://github.com/naomijub/tokio-retry/pull/6)

- Adds `MapErr` trait to `RetryError`, that facilitates handling `map_err` in `Result` types, transforming them into `Result<_, RetryError<_>>`

## Version 0.5.0 @ [#5](https://github.com/naomijub/tokio-retry/pull/5)

- Adds `RetryError` type inspired by [`backoff Error`](https://docs.rs/backoff/latest/backoff/enum.Error.html) where user can define Errors as `Transient` and `Permanent` so it is possible to early exit a retry. 

## Version 0.4.0 @ [#4](https://github.com/naomijub/tokio-retry/pull/4)

- Adds `Retry::spawn_notify` to notify a retry with the associated `Error` and current `Duration`, PR [#4](https://github.com/naomijub/tokio-retry/pull/4).
- Adds github actions removing obsolete travis.ci.
- Bi-monthly runs CI checking for outdated dependencies.
- Applies const to functions that can be const.
- Adds linting defaults.
- Optional jitter from @fuzzbuck PR [#26](https://github.com/srijs/rust-tokio-retry/pull/26)
- Implements max_interval from @pzmarzly PR [#27](https://github.com/srijs/rust-tokio-retry/pull/27)
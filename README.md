# Listenbrainz_rs

Api bindings for the listenbrainz api

## Feature flags

Async: 
- `sync`: Enable the sync api
- `async`: Enable the async api (Sync and Async aren't mutually exclusive)

Fetching:
- `native_tls`: Use the system's native TLS. By default, Rustls is used to not have to depend on the system's tls
- `rate_limit`: Add a rate limiter to the requests, using the `governor` crate. Please note that it only affect `async` variants of functions, as `governor` is made to work in async functions only. If you know a ratelimit crate that does both sync and async, feel free to submit an issue 

Debuging:
- `backtrace`: Enable error backtraces
- `tracing`: Enable tracing
- `hotpath`, `hotpath-alloc`, `hotpath-off`: Enable [hotpath](https://github.com/pawurb/hotpath-rs) debuging / perf analysis.


## Why another crate? What's so special about it?

This crate is focus on aspects lacking from [listenbrainz](https://crates.io/crates/listenbrainz) and [listenbrainz-rust](https://crates.io/crates/listenbrainz-rust). Here's what's special:
- Full async support
- Ratelimiting
- Auto retries of failed requests (If the error is temporary)
- Tracing
- Useful errors
- Similar names to the [Official documentation](https://listenbrainz.readthedocs.io/en/latest/users/api/core.html)
- Source code that isn't unreadable 1000 lines long files
- Api similar to [musicbrainz_rs](https://crates.io/crates/musicbrainz_rs)
- Builder pattern for query parameters using [bon](https://crates.io/crates/bon)
- Paranoid CI suite
- Runtime agnostic (This crate use [`blocking`](https://crates.io/crates/blocking) to turn [`ureq`](https://crates.io/crates/ureq)'s blocking requests into async ones by spawning classic `std` threads. However those threads do nothing while waiting for the response, so it's fine to spawn more than the number of cores in the cpu)


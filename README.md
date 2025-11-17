# Listenbrainz-rs-nova

Api bindings for the listenbrainz api

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


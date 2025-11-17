#[cfg(feature = "rate_limit")]
use core::num::NonZeroU32;
#[cfg(feature = "rate_limit")]
use std::sync::Arc;

#[cfg(feature = "rate_limit")]
use governor::Quota;
#[cfg(feature = "rate_limit")]
use governor::RateLimiter;
#[cfg(feature = "rate_limit")]
use governor::clock;
#[cfg(feature = "rate_limit")]
use governor::middleware::NoOpMiddleware;
#[cfg(feature = "rate_limit")]
use governor::state::InMemoryState;
#[cfg(feature = "rate_limit")]
use governor::state::NotKeyed;
use reqwest::Client;

pub mod api_request;
pub mod http_verb;

/// The client handling fetching
pub struct ListenBrainzClient {
    reqwest_client: Client,

    pub api_domain: String,

    max_retries: u32,

    #[cfg(feature = "rate_limit")]
    pub rate_limit:
        Option<Arc<RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>>>,
}

impl ListenBrainzClient {
    pub fn new() -> Self {
        #[cfg(feature = "rate_limit")]
        let quota =
            Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());

        Self {
            reqwest_client: Client::new(),

            api_domain: "api.listenbrainz.org".to_string(),

            max_retries: 10,

            #[cfg(feature = "rate_limit")]
            rate_limit: Some(Arc::new(RateLimiter::direct(quota))),
        }
    }

    /// Wait for a ratelimit spot
    #[cfg(feature = "rate_limit")]
    pub async fn await_rate_limit(&self) {
        if let Some(rate) = &self.rate_limit {
            rate.until_ready().await
        }
    }
}

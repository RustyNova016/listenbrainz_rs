use core::time::Duration;

use async_io::Timer;
use snafu::ResultExt;
use ureq::Body;
use ureq::http::Response;

use crate::client::ListenBrainzClient;
use crate::client::api_request::ApiRequest;
use crate::client::api_request::error::ApiRequestError;
use crate::client::api_request::error::MaxRetriesExceededSnafu;
use crate::client::api_request::error::UreqSnafu;
use crate::client::http_verb::HTTPVerb;

impl<T> ApiRequest<T> {
    /// Send the request without any fluff.
    pub async fn send_request_raw(
        &self,
        client: &ListenBrainzClient,
    ) -> Result<Response<Body>, ApiRequestError> {
        debug_assert_eq!(self.verb, HTTPVerb::Get);

        let url = format!("https://{}{}", client.api_domain, &self.url);

        #[cfg(feature = "tracing")]
        tracing::debug!("Sending GET request at {url}");

        blocking::unblock(|| {
            ureq::get(url)
                .config()
                .http_status_as_error(false)
                .build()
                .call()
        })
        .await
        .context(UreqSnafu)
    }

    /// Send the request, deal with errors and ratelimiting
    #[mutants::skip]
    pub async fn try_send_request(
        &mut self,
        client: &ListenBrainzClient,
    ) -> Result<Option<Response<Body>>, ApiRequestError> {
        client.await_rate_limit().await;
        self.tries += 1;

        let response = match self.send_request_raw(client).await {
            Ok(val) => val,
            Err(err) => {
                if err.is_retryable() {
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        };

        // Let's check if we hit the rate limit
        if response.status().as_u16() == 503 {
            // Oh no. Let's wait the timeout
            let headers = response.headers();
            let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
            let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
            Timer::after(duration).await;

            return Ok(None);
        };

        Ok(Some(response))
    }

    /// Send the request, and retry on failure
    #[mutants::skip]
    pub async fn send_with_retries(
        &mut self,
        client: &ListenBrainzClient,
    ) -> Result<Response<Body>, ApiRequestError> {
        while self.tries < client.max_retries {
            if let Some(res) = self.try_send_request(client).await? {
                return Ok(res);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }
}

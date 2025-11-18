use core::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::client::ListenBrainzClient;
use crate::client::api_request::error::ApiRequestError;
use crate::client::http_verb::HTTPVerb;

pub mod error;
pub mod fetching;
pub mod parsing;

/// A raw API request, used to send custom requests to the API
pub struct ApiRequest<T> {
    /// The url to fetch
    pub url: String,

    /// The http verb of the api request
    pub verb: HTTPVerb,

    /// The current number of times the request has been tried
    pub tries: u32,

    /// The schema of the returned data
    pub returned_schema: PhantomData<T>,
}

impl<T> ApiRequest<T> {
    pub fn new(url: String, verb: HTTPVerb) -> Self {
        Self {
            url,
            verb,
            tries: 0,
            returned_schema: Default::default(),
        }
    }

    pub async fn send(&mut self, client: &ListenBrainzClient) -> Result<T, ApiRequestError>
    where
        T: DeserializeOwned,
    {
        let response = self.send_with_retries(client).await?;
        self.parse_response(response).await
    }
}

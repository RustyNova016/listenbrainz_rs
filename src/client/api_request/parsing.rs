use reqwest::Response;
use serde::de::DeserializeOwned;
use snafu::ResultExt;

use crate::client::api_request::ApiRequest;
use crate::client::api_request::error::ApiRequestError;
use crate::client::api_request::error::InvalidResponseSnafu;
use crate::client::api_request::error::ReqwestSnafu;

impl<T> ApiRequest<T> {
    /// Parse the request json
    pub async fn parse_response(&self, response: Response) -> Result<T, ApiRequestError>
    where
        T: DeserializeOwned,
    {
        let text = response.text().await.context(ReqwestSnafu)?;

        // Try to deserialize as our result
        let err = match serde_json::from_str::<T>(&text) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // // We have an error. Let's try deserializing MB's error
        // if let Ok(result) = serde_json::from_value::<MusicbrainzError>(json.clone()) {
        //     return Err(result).context(ApiSnafu);
        // };

        // Not a server error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| InvalidResponseSnafu { data: text })
    }
}

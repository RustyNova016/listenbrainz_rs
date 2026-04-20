use api_bindium::ApiRequest;
use api_bindium::ApiRequestError;
use api_bindium::endpoints::UriBuilderError;
use serde::Deserialize;
use serde::Serialize;
use snafu::Snafu;

use crate::api::validate_token::ValidateTokenResponse;

/// API token for logged in requests
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserToken(String);

impl UserToken {
    /// Verify the validity of the token.
    #[cfg(feature = "sync")]
    pub fn verify(
        &self,
        client: &crate::ListenBrainzClient,
    ) -> Result<ValidateTokenResponse, UserTokenError> {
        use snafu::ResultExt as _;

        let res = client
            .endpoints()
            .post_validate_token(self.clone())
            .context(UriBuilderSnafu)?
            .send(client.api_client())
            .context(ApiRequestSnafu)?
            .parse()
            .context(ApiRequestSnafu)?;

        if !res.valid {
            return InvalidTokenSnafu { response: res }.fail();
        }

        Ok(res)
    }

    /// Verify the validity of the token.
    #[cfg(feature = "async")]
    pub async fn verify_async(
        &self,
        client: &crate::ListenBrainzClient,
    ) -> Result<ValidateTokenResponse, UserTokenError> {
        use snafu::ResultExt as _;

        let res = client
            .endpoints()
            .post_validate_token(self.clone())
            .context(UriBuilderSnafu)?
            .send_async(client.api_client())
            .await
            .context(ApiRequestSnafu)?
            .parse()
            .context(ApiRequestSnafu)?;

        if !res.valid {
            return InvalidTokenSnafu { response: res }.fail();
        }

        Ok(res)
    }

    /// Add the token to the api request
    pub fn add_authorization<P>(&self, request: &mut ApiRequest<P>) {
        // TODO: Upstream authorization headers into `api_bindium`
        request
            .headers_mut()
            .insert("AUTHORIZATION".to_string(), format!("Token {}", self.0));
    }
}

impl From<UserToken> for String {
    fn from(value: UserToken) -> Self {
        value.0
    }
}

impl From<String> for UserToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Snafu)]
pub enum UserTokenError {
    UriBuilderError {
        source: UriBuilderError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    ApiRequestError {
        source: ApiRequestError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    // Invalid token
    InvalidToken {
        response: ValidateTokenResponse,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::JsonParser;
use api_bindium::endpoints::UriBuilderError;
use serde::Deserialize;
use serde::Serialize;

use crate::api::ListenBrainzAPIEnpoints;
use crate::models::token::UserToken;

impl ListenBrainzAPIEnpoints {
    pub fn post_validate_token(
        &self,
        token: UserToken,
    ) -> Result<ApiRequest<JsonParser<ValidateTokenResponse>>, UriBuilderError> {
        let mut request = self
            .endpoint_builder()
            .set_path("/1/validate-token")
            .into_api_request(HTTPVerb::Get, JsonParser::default())?;

        token.add_authorization(&mut request);

        Ok(request)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateTokenResponse {
    pub code: u32,
    pub message: String,
    pub valid: bool,
    pub user_name: String,
}

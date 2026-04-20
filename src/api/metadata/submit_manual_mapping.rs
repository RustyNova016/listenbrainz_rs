use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::JsonParser;
use api_bindium::endpoints::UriBuilderError;

use crate::api::ListenBrainzAPIEnpoints;
use crate::models::token::UserToken;

impl ListenBrainzAPIEnpoints {
    pub fn post_submit_manual_mapping(
        &self,
        mapping: SubmitManualMappingBody,
        token: UserToken,
    ) -> Result<ApiRequest<JsonParser<Vec<SubmitManualMappingBody>>>, UriBuilderError> {
        let mut request = self
            .endpoint_builder()
            .set_path("/1/metadata/submit_manual_mapping/")
            .into_api_request_with_body(
                HTTPVerb::Post,
                serde_json::to_value(mapping).unwrap(),
                JsonParser::default(),
            )?;

        token.add_authorization(&mut request);

        Ok(request)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubmitManualMappingBody {
    /// The MSID to map
    recording_msid: String,

    /// The MBID to map
    recording_mbid: String,
}

use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::JsonParser;
use api_bindium::endpoints::UriBuilderError;

use crate::api::ListenBrainzAPIEnpoints;

#[bon::bon]
impl ListenBrainzAPIEnpoints {
    #[builder]
    pub fn delete_listen(
        &self,
        recording_msid: &str,
        listened_at: u64,
    ) -> Result<ApiRequest<JsonParser<()>>, UriBuilderError> {
        self.endpoint_builder()
            .set_path(&format!("/1/delete-listen"))
            .into_api_request_with_body(
                HTTPVerb::Post,
                serde_json::to_value(DeleteListenBody {
                    listened_at,
                    recording_msid: recording_msid.to_string(),
                })
                .unwrap(),
                JsonParser::default(),
            )
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteListenBody {
    pub listened_at: u64,
    pub recording_msid: String,
}

use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::UriBuilderError;

use crate::api::ListenBrainzAPIEnpoints;

impl ListenBrainzAPIEnpoints {
    pub fn post_popularity_recording(
        &self,
        recording_mbids: Vec<String>,
    ) -> Result<ApiRequest<JsonParser<Vec<PopularityRecordingResponse>>>, UriBuilderError> {
        self.endpoint_builder()
            .set_path("/1/popularity/recording")
            .into_api_request_with_body(
                HTTPVerb::Post,
                serde_json::to_value(PopularityRecordingQuery { recording_mbids }).unwrap(),
            )
    }
}

#[derive(serde::Serialize)]
struct PopularityRecordingQuery {
    recording_mbids: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct PopularityRecordingResponse {
    pub recording_mbid: String,
    pub total_listen_count: Option<u64>,
    pub total_user_count: Option<u64>,
}

#[cfg(test)]
#[cfg(feature = "async")]
mod test {

    use macro_rules_attribute::apply;

    use crate::client::ListenBrainzClient;

    #[apply(smol_macros::test!)]

    async fn post_popularity_recording_test() {
        let client = ListenBrainzClient::default();

        let mut req = client
            .endpoints()
            .post_popularity_recording(vec!["61c54b0e-3a82-49af-9cc7-73ff34365697".to_string()])
            .unwrap();
        let mut res = req.send_async(client.api_client()).await.unwrap();

        let res = res.pop().unwrap();
        assert_eq!(
            res.recording_mbid,
            "61c54b0e-3a82-49af-9cc7-73ff34365697".to_string()
        );
        assert!(res.total_listen_count.is_some());
    }
}

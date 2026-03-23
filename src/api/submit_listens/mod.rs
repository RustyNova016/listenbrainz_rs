use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::TextParser;
use api_bindium::endpoints::UriBuilderError;
use serde::Deserialize;
use serde::Serialize;

use crate::ListenBrainzAPIEnpoints;
use crate::api::user::username::listens::UserListensListen;
use crate::api::user::username::listens::UserListensTrackMetadata;

impl ListenBrainzAPIEnpoints {
    pub fn post_submit_listens(
        &self,
        listens: SubmitListens,
        api_token: String,
    ) -> Result<ApiRequest<TextParser>, UriBuilderError> {
        let mut req = self
            .endpoint_builder()
            .set_path("/1/submit-listens")
            .into_api_request_with_body(
                HTTPVerb::Post,
                serde_json::to_value(listens).unwrap(),
                TextParser,
            )?;

        req.headers_mut()
            .insert("Authorization".to_string(), api_token);

        Ok(req)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "listen_type")]
pub enum SubmitListens {
    Single { payload: Vec<SubmitListensListen> },
}

/// Type of the [`UserListensPayload::listens`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SubmitListensListen {
    pub user_name: Option<String>,
    pub inserted_at: Option<i64>,
    pub listened_at: i64,
    pub recording_msid: Option<String>,
    pub track_metadata: UserListensTrackMetadata,
}

impl From<UserListensListen> for SubmitListensListen {
    fn from(value: UserListensListen) -> Self {
        Self {
            inserted_at: Some(value.inserted_at),
            listened_at: value.listened_at,
            recording_msid: Some(value.recording_msid),
            track_metadata: value.track_metadata,
            user_name: Some(value.user_name),
        }
    }
}

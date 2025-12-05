use std::collections::HashMap;

use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use serde::Deserialize;
use serde::Serialize;
use ureq::http::uri::InvalidUri;

use crate::api::ListenBrainzAPIEnpoints;

#[bon::bon]
impl ListenBrainzAPIEnpoints {
    #[builder]
    pub fn get_user_username_listens(
        &self,
        username: &str,
        max_ts: Option<u64>,
        min_ts: Option<u64>,
        count: Option<u64>,
    ) -> Result<ApiRequest<UserListensResponse>, InvalidUri> {
        self.endpoint_builder()
            .set_path(&format!("/1/user/{username}/listens"))
            .maybe_add_parameter("max_ts", max_ts)
            .maybe_add_parameter("min_ts", min_ts)
            .maybe_add_parameter("count", count)
            .into_api_request(HTTPVerb::Get)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensResponse {
    pub payload: UserListensPayload,
}

/// Type of the [`UserListensResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensPayload {
    pub count: u64,
    pub latest_listen_ts: i64,
    pub oldest_listen_ts: i64,
    pub user_id: String,
    pub listens: Vec<UserListensListen>,
}

/// Type of the [`UserListensPayload::listens`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensListen {
    pub user_name: String,
    pub inserted_at: i64,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UserListensTrackMetadata,
}

/// Type of the [`UserListensListen::track_metadata`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
    pub mbid_mapping: Option<UserListensMBIDMapping>,
}

/// Type of the [`UserListensTrackMetadata::mbid_mapping`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensMBIDMapping {
    pub artist_mbids: Option<Vec<String>>,
    pub artists: Option<Vec<UserListensMappingArtist>>,
    pub recording_mbid: String,
    pub recording_name: Option<String>,
    pub caa_id: Option<u64>,
    pub caa_release_mbid: Option<String>,
    pub release_mbid: Option<String>,
}

/// Type of the [`UserListensMBIDMapping::artists`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensMappingArtist {
    pub artist_mbid: String,
    pub artist_credit_name: String,
    pub join_phrase: String,
}

#[cfg(test)]
#[cfg(feature = "async")]
mod test {

    use macro_rules_attribute::apply;

    use crate::client::ListenBrainzClient;

    #[apply(smol_macros::test!)]

    async fn get_user_username_listens_test() {
        let client = ListenBrainzClient::default();

        let mut req = client
            .endpoints()
            .get_user_username_listens()
            .username("RustyNova")
            .min_ts(1763396995)
            .max_ts(1763396997)
            .count(1)
            .call()
            .unwrap();
        let mut res = req.send_async(client.api_client()).await.unwrap();

        assert_eq!(res.payload.count, 1);

        let first = res.payload.listens.pop().unwrap();

        assert_eq!(first.recording_msid, "cfb002e7-f093-4678-8bf7-fb139a4f718c")
    }
}

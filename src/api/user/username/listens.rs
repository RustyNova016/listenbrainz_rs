use std::collections::HashMap;
use std::fmt::Write;

use serde::Deserialize;
use serde::Serialize;

use crate::api::ListenBrainzAPI;
use crate::client::api_request::ApiRequest;
use crate::client::http_verb::HTTPVerb;

#[bon::bon]
impl ListenBrainzAPI {
    #[builder]
    pub fn get_user_username_listens(
        username: &str,
        max_ts: Option<u64>,
        min_ts: Option<u64>,
        count: Option<u64>,
    ) -> ApiRequest<UserListensResponse> {
        let mut url = format!("/1/user/{username}/listens?");

        if let Some(max_ts) = max_ts {
            write!(url, "max_ts={}&", max_ts).unwrap();
        }

        if let Some(min_ts) = min_ts {
            write!(url, "min_ts={}&", min_ts).unwrap();
        }

        if let Some(count) = count {
            write!(url, "count={}&", count).unwrap();
        }

        ApiRequest::new(url, HTTPVerb::Get)
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
mod test {

    use crate::api::ListenBrainzAPI;
    use crate::client::ListenBrainzClient;

    #[tokio::test]
    #[serial_test::serial]
    async fn get_user_username_listens_test() {
        let client = ListenBrainzClient::default();

        let mut req = ListenBrainzAPI::get_user_username_listens().username("RustyNova").min_ts(1763396995).max_ts(1763396997).count(1).call();
        let mut res = req.send(&client).await.unwrap();

        assert_eq!(res.payload.count, 1);

        let first = res.payload.listens.pop().unwrap();

        assert_eq!(first.recording_msid, "cfb002e7-f093-4678-8bf7-fb139a4f718c")
    }
}
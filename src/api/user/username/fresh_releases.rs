use core::fmt::Display;

use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use serde::Deserialize;
use serde::Serialize;
use ureq::http::uri::InvalidUri;

use crate::api::ListenBrainzAPIEnpoints;

#[bon::bon]
impl ListenBrainzAPIEnpoints {
    #[builder]
    pub fn get_user_username_fresh_releases(
        &self,
        username: &str,
        sort: Option<FreshReleaseSort>,
        past: Option<bool>,
        future: Option<bool>,
    ) -> Result<ApiRequest<FreshReleaseResponse>, InvalidUri> {
        self.endpoint_builder()
            .set_path(&format!("/1/user/{username}/fresh_releases"))
            .maybe_add_parameter("sort", sort)
            .maybe_add_parameter("past", past)
            .maybe_add_parameter("future", future)
            .into_api_request(HTTPVerb::Get)
    }
}

// === Argument types ===

#[derive(Debug)]
pub enum FreshReleaseSort {
    ReleaseDate,
    ArtistCreditName,
    ReleaseName,
    Confidence,
}

impl Display for FreshReleaseSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArtistCreditName => write!(f, "artist_credit_name"),
            Self::Confidence => write!(f, "confidence"),
            Self::ReleaseDate => write!(f, "release_date"),
            Self::ReleaseName => write!(f, "release_name"),
        }
    }
}

// === Response types ===

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FreshReleaseResponse {
    pub payload: FreshReleasePayload,
}

/// Type of the [`FreshReleaseResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FreshReleasePayload {
    pub user_id: String,
    pub releases: Vec<FreshReleaseRelease>,
}

/// Type of the [`FreshReleasePayload::releases`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FreshReleaseRelease {
    pub artist_credit_name: String,
    pub artist_mbids: Vec<String>,
    pub caa_id: Option<u64>,
    pub caa_release_mbid: Option<String>,
    pub confidence: u64,
    pub listen_count: u64,
    pub release_date: String,
    pub release_group_mbid: String,
    pub release_group_primary_type: Option<String>,
    pub release_group_secondary_type: Option<String>,
    pub release_mbid: String,
    pub release_name: String,
    pub release_tags: Vec<String>,
}

#[cfg(test)]
#[cfg(feature = "async")]
mod test {

    use macro_rules_attribute::apply;

    use crate::client::ListenBrainzClient;

    #[apply(smol_macros::test!)]

    async fn get_user_username_fresh_releases_test() {
        let client = ListenBrainzClient::default();

        let mut req = client
            .endpoints()
            .get_user_username_fresh_releases()
            .username("RustyNova")
            .call()
            .unwrap();
        let _res = req.send_async(client.api_client()).await.unwrap();

        // Can't do much here as fresh releases are dynamic...
    }
}

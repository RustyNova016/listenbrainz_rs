use api_bindium::ApiClient;

use crate::api::ListenBrainzAPIEnpoints;

#[derive(Debug, bon::Builder, Clone)]
pub struct ListenBrainzClient {
    #[builder(default)]
    api_client: ApiClient,

    #[builder(default)]
    endpoints: ListenBrainzAPIEnpoints,
}

impl ListenBrainzClient {
    pub fn endpoints(&self) -> &ListenBrainzAPIEnpoints {
        &self.endpoints
    }

    pub fn api_client(&self) -> &ApiClient {
        &self.api_client
    }
}

impl Default for ListenBrainzClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

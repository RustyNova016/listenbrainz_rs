use std::sync::Arc;

use api_bindium::ApiClient;
use async_executor::Executor;

use crate::api::ListenBrainzAPIEnpoints;

#[derive(Debug, bon::Builder, Clone)]
pub struct ListenBrainzClient {
    #[builder(default)]
    api_client: ApiClient,

    /// An async concurent [Executor] of the api. You can reuse your own to prevent duplicated runtimes
    #[builder(default)]
    async_executor: Arc<Executor<'static>>,

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

    pub fn async_executor(&self) -> &Arc<Executor<'static>> {
        &self.async_executor
    }
}

impl Default for ListenBrainzClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

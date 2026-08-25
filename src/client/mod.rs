#[cfg(feature = "async")]
use std::sync::Arc;

use api_bindium::ApiClient;
use api_bindium::ureq::Agent;
use api_bindium::ureq::config::Config;
use api_bindium::ureq::config::ConfigBuilder;
#[cfg(feature = "native_tls")]
use api_bindium::ureq::tls::TlsConfig;
#[cfg(feature = "native_tls")]
use api_bindium::ureq::tls::TlsProvider;
use api_bindium::ureq::typestate::AgentScope;
#[cfg(feature = "async")]
use async_executor::Executor;

use crate::api::ListenBrainzAPIEnpoints;

pub(crate) const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, bon::Builder, Clone)]
pub struct ListenBrainzClient {
    #[builder(default = ListenBrainzClient::default_api_client())]
    pub api_client: ApiClient,

    /// An async concurent [Executor] of the api. You can reuse your own to prevent duplicated runtimes
    #[cfg(feature = "async")]
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

    #[cfg(feature = "async")]
    pub fn async_executor(&self) -> &Arc<Executor<'static>> {
        &self.async_executor
    }

    /// The default config for the internal ureq http agent. Allows configuring middleware, tls, and more
    ///
    /// If the `native_tls` feature is enabled, it automatically set the tls to use `native-tls`
    ///
    /// If the `hotpath` feature is enabled, it automatically add the hotpath middleware
    pub fn default_agent_config_builder() -> ConfigBuilder<AgentScope> {
        let conf = Config::builder().user_agent(DEFAULT_USER_AGENT);

        #[cfg(feature = "hotpath-http")]
        let conf = hotpath::http!(conf);

        #[cfg(feature = "native_tls")]
        let conf = conf.tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::NativeTls)
                .build(),
        );

        conf
    }

    /// Create the default [`api_bindium::ApiClient`] for the api requesting.
    pub fn default_api_client() -> ApiClient {
        let agent_conf = Self::default_agent_config_builder().build();
        let agent = Agent::new_with_config(agent_conf);

        ApiClient::builder().agent(agent).build()
    }
}

impl Default for ListenBrainzClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

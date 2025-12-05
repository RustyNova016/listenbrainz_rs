use api_bindium::endpoints::EndpointUriBuilder;
use api_bindium::endpoints::path::EndpointUriBuilderPath;

pub mod popularity;
pub mod user;

#[derive(Debug, bon::Builder, Clone)]
pub struct ListenBrainzAPIEnpoints {
    /// The domain of the listenbrainz server.
    ///
    /// Please note that all the api endpoints must be accessed by HTTPS.
    #[builder(default = "api.listenbrainz.org".to_string())]
    lb_domain: String,
}

impl ListenBrainzAPIEnpoints {
    /// The api root
    pub fn api_root(&self) -> String {
        format!("https://{}", self.lb_domain)
    }

    /// Return the root Uri for the
    pub fn endpoint_builder(&self) -> EndpointUriBuilder<EndpointUriBuilderPath> {
        EndpointUriBuilder::new()
            .https()
            .set_authority(&self.lb_domain)
    }
}

impl Default for ListenBrainzAPIEnpoints {
    fn default() -> Self {
        Self::builder().build()
    }
}

#![allow(
    clippy::result_large_err,
    reason = "Until `snafu` got something to easily box error sources, we ignore it"
)]

pub mod api;
pub mod client;
#[cfg(feature = "async")]
mod inner_macros;
pub mod models;

pub use crate::api::ListenBrainzAPIEnpoints;
pub use crate::client::ListenBrainzClient;
pub use api_bindium;

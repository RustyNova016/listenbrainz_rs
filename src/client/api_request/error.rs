use core::error::Error as _;

use snafu::Snafu;


#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum ApiRequestError {
    #[snafu(display("Couldn't successfully send the http request"))]
    ReqwestError {
        source: reqwest::Error,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("The max retry count for the request as been exeeded. You may want to check if the correct url is set, the server is online, or you aren't hitting the ratelimit."))]
    MaxRetriesExceeded {
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("The api's response couldn't be deserialized:\n{data}"))]
    InvalidResponse {
        source: serde_json::Error,
        data: String,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    }
}

impl ApiRequestError {
    /// Return true if the error is temporary and should be retried
    pub fn is_retryable(&self) -> bool {
        self.is_timeout() || self.is_connection_reset()
    }

    /// Return true if the error is a timeout
    pub fn is_timeout(&self) -> bool {
        // Reqwest error
        let Some(source) = self.source() else {
            return false;
        };
        let Some(reqwest_error) = source.downcast_ref::<reqwest::Error>() else {
            return false;
        };

        reqwest_error.is_timeout()
    }

    /// Return true if the error is a connection reset
    pub fn is_connection_reset(&self) -> bool {
        // Reqwest error
        let Some(source) = self.source() else {
            return false;
        };
        let Some(reqwest_error) = source.downcast_ref::<reqwest::Error>() else {
            return false;
        };

        // Hyper_util error
        let Some(source) = reqwest_error.source() else {
            return false;
        };
        let Some(hyper_util_error) = source.downcast_ref::<hyper_util::client::legacy::Error>()
        else {
            return false;
        };

        // Hyper error
        let Some(source) = hyper_util_error.source() else {
            return false;
        };
        let Some(hyper_error) = source.downcast_ref::<hyper::Error>() else {
            return false;
        };

        // IO error
        let Some(source) = hyper_error.source() else {
            return false;
        };
        let Some(std_error) = source.downcast_ref::<std::io::Error>() else {
            return false;
        };

        std_error.kind() == std::io::ErrorKind::ConnectionReset
    }
}

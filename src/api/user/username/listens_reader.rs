use api_bindium::ApiRequestError;
use chrono::Utc;
use snafu::ResultExt;
use snafu::Snafu;
use ureq::http::uri::InvalidUri;

use crate::api::ListenBrainzAPIEnpoints;
use crate::api::user::username::listens::UserListensListen;
use crate::api::user::username::listens::UserListensResponse;
use crate::client::ListenBrainzClient;

#[bon::bon]
impl ListenBrainzAPIEnpoints {
    /// Get all the listens in a time period, removing the paging.
    ///
    /// Due to implementation details and quirks in the API, the listens may not be sorted,
    /// or require more queries than neccesary

    #[builder]
    pub async fn get_user_username_listens_full<'s>(
        client: &'s ListenBrainzClient,
        username: &'s str,
        start: Option<u64>,
        end: Option<u64>,
    ) -> Result<Vec<UserListensListen>, ListenFullFetchError> {
        let mut works = vec![(
            start.unwrap_or_default(),
            end.unwrap_or_else(|| Utc::now().timestamp() as u64),
        )];
        let mut listens = Vec::new();

        let mut min_start = None;
        while let Some((start, end)) = works.pop() {
            // Prevent fetching a period that is before any listen
            if min_start.is_some_and(|min_start| end < min_start) {
                continue;
            }

            // If the period is too big, cut it
            if end - start > 3600 * 24 * 15 {
                let middle = ((end - start) / 2) + start;
                works.push((start, middle + 1));
                works.push((middle, end));
                continue;
            }

            let res = send_request(client, username, start, end).await?;

            min_start = Some(res.payload.oldest_listen_ts as u64);

            // Check if the period overflowed
            if res.payload.listens.len() == 1000 {
                let middle = ((end - start) / 2) + start;
                works.push((start, middle + 1));
                works.push((middle, end));
            } else {
                listens.extend(res.payload.listens);
            }
        }

        Ok(listens)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
async fn send_request(
    client: &ListenBrainzClient,
    username: &str,
    start: u64,
    end: u64,
) -> Result<UserListensResponse, ListenFullFetchError> {
    let mut req = client
        .endpoints()
        .get_user_username_listens()
        .username(username)
        .min_ts(start)
        .max_ts(end)
        .count(1000)
        .call()
        .context(InvalidUriSnafu)?;

    req.send_async(client.api_client())
        .await
        .context(ApiRequestSnafu)
}

#[derive(Debug, Snafu)]
pub enum ListenFullFetchError {
    ApiRequestError {
        source: ApiRequestError,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    InvalidUriError {
        source: InvalidUri,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

#[cfg(feature = "async")]
#[cfg(test)]
mod test {

    use macro_rules_attribute::apply;

    use crate::api::ListenBrainzAPIEnpoints;
    use crate::client::ListenBrainzClient;

    #[apply(smol_macros::test!)]
    async fn get_user_username_listens_test() {
        #[cfg(feature = "hotpath")]
        let _hotpath = hotpath::GuardBuilder::new("test_async_function")
            .percentiles(&[50, 90, 95])
            .format(hotpath::Format::Table)
            .build();

        let client = ListenBrainzClient::default();

        let req = ListenBrainzAPIEnpoints::get_user_username_listens_full()
            .client(&client)
            .username("RustyNova")
            .start(1705000000)
            .end(1710000000)
            .call()
            .await
            .unwrap();

        assert_eq!(req.len(), 4840);
    }
}

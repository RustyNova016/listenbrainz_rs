use std::collections::VecDeque;

use api_bindium::ApiRequestError;
use api_bindium::endpoints::UriBuilderError;
use chrono::Utc;
use snafu::ResultExt;
use snafu::Snafu;

use crate::api::ListenBrainzAPIEnpoints;
use crate::api::user::username::listens::UserListensListen;
use crate::api::user::username::listens::UserListensResponse;
use crate::client::ListenBrainzClient;
use crate::inner_macros::pg_counted;
use crate::inner_macros::pg_inc;

#[bon::bon]
impl ListenBrainzAPIEnpoints {
    /// Get all the listens in a time period, removing the paging.
    ///
    /// Due to implementation details and quirks in the API, the listens may not be sorted,
    /// or require more queries than neccesary

    #[builder]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty)))]
    pub async fn get_user_username_listens_full<'s>(
        client: &'s ListenBrainzClient,
        username: &'s str,
        start: Option<u64>,
        end: Option<u64>,
    ) -> Result<Vec<UserListensListen>, ListenFullFetchError> {
        pg_counted!(1, "Fetching listens");

        #[allow(unused_variables)]
        let mut fetch_count = 1;

        let mut works = VecDeque::from([(
            start.unwrap_or_default(),
            end.unwrap_or_else(|| Utc::now().timestamp() as u64),
        )]);

        let mut listens = Vec::new();

        let mut min_start = None;
        while let Some((start, end)) = works.pop_front() {
            // Prevent fetching a period that is before any listen
            if min_start.is_some_and(|min_start| end < min_start) {
                fetch_count -= 1;
                pg_counted!(fetch_count, "Fetching listens");
                continue;
            }

            // If the period is too big, cut it
            if end - start > 3600 * 24 * 15 {
                let middle = ((end - start) / 2) + start;
                works.push_back((start, middle + 1));
                works.push_back((middle, end));
                fetch_count += 1;
                pg_counted!(fetch_count, "Fetching listens");
                continue;
            }

            let res = send_request(client, username, start, end).await?;

            min_start = Some(res.payload.oldest_listen_ts as u64);

            // Check if the period overflowed
            if res.payload.listens.len() == 1000 {
                let middle = ((end - start) / 2) + start;
                works.push_back((start, middle + 1));
                works.push_back((middle, end));
                fetch_count += 1;
                pg_counted!(fetch_count, "Fetching listens");
            } else {
                listens.extend(res.payload.listens);
                pg_inc!();
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
        .context(UriBuilderSnafu)?;

    req.send_async(client.api_client())
        .await
        .context(ApiRequestSnafu)
}

#[derive(Debug, Snafu)]
pub enum ListenFullFetchError {
    ApiRequestError {
        source: ApiRequestError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    UriBuilderError {
        source: UriBuilderError,

        #[snafu(implicit)]
        location: snafu::Location,

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
        let _hotpath = hotpath::FunctionsGuardBuilder::new("test_async_function")
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

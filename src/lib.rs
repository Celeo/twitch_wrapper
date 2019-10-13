//! Rust wrappers for the Twitch API.

// #![warn(missing_docs)]

use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

mod models;

// Main API wrapper.
pub struct Twitch {
    /// The reqwest HTTP client instance
    client: Client,
    /// The developer's client id from their Twitch developer apps
    client_id: String,
}

impl Twitch {
    /// Construct a new instance of the Twitch struct in order to access the API.
    ///
    /// # Arguments
    ///
    /// * `client_id` - your client id from the [developer console]
    ///
    /// [developer console]: https://dev.twitch.tv/console/apps
    pub fn new(client_id: &str) -> Self {
        Twitch {
            client: Client::new(),
            client_id: client_id.to_owned(),
        }
    }

    /// Get the base REST API URL.
    fn base_url(&self) -> String {
        #[cfg(not(test))]
        return "https://api.twitch.tv/helix".to_owned();
        #[cfg(test)]
        return mockito::server_url();
    }

    /// Populate a map of the required headers.
    fn get_headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(
            HeaderName::from_lowercase(b"client-id").unwrap(),
            HeaderValue::from_bytes(self.client_id.as_bytes()).unwrap(),
        );
        map
    }

    /// Get the top streams.
    pub fn get_streams(&self, count: u64) -> Result<models::streams::StreamList> {
        let mut resp = self
            .client
            .get(&format!("{}/streams", self.base_url()))
            .headers(self.get_headers())
            .query(&[("first", &format!("{}", count))]) // TODO
            .send()?;
        if !resp.status().is_success() {
            anyhow::bail!("Received error status code from API: {}", resp.status());
        }
        let resp: models::streams::StreamList = resp.json()?;
        Ok(resp)
    }
}

//! Rust wrappers for the Twitch API.

// #![warn(missing_docs)]

use reqwest::Client;

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
    ///
    /// Supports local unit testing through mockito.
    fn base_url(&self) -> String {
        #[cfg(not(test))]
        return "https://api.twitch.tv/helix".to_owned();
        #[cfg(test)]
        return mockito::server_url();
    }
}

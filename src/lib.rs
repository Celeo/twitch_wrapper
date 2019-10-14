//! Rust wrappers for the Twitch API.
//!
//! Start by getting a Client-ID from the twitch [developer console]. This
//! header value is required to make any API requests. Then, create an instance
//! of the [Twitch] struct:
//!
//! ```rust,no_run
//! use twitch_wrapper::Twitch;
//!
//! let twitch = Twitch::new("your_client_id_here");
//! ```
//!
//! From there, look at the struct documentation to see which functions are available.
//!
//! [developer console]: https://dev.twitch.tv/console/apps
//! [Twitch]: ./struct.Twitch.html

#![warn(missing_docs)]

use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::str::FromStr;

pub mod models;

/// Main API wrapper.
///
/// Construct with `Twitch::new`, passing in your Client-ID from the developer console.
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
            HeaderName::from_str("client-id").unwrap(),
            HeaderValue::from_bytes(self.client_id.as_bytes()).unwrap(),
        );
        map
    }

    /// Query an endpoint.
    ///
    /// This is mostly used as an internal method but can be used
    /// by applications for endpoints that don't have wrapper functions.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method string
    /// * `endpoint` - API endpoint (don't include a leading slash)
    /// * `query` - optional query params to include
    ///
    /// # Types
    ///
    /// * `T` - a struct to deserialize the data into
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twitch_wrapper::{Twitch, models::streams::StreamList};
    /// # let twitch = Twitch::new("abc");
    /// let resp: StreamList = twitch.query("GET", "streams", None).unwrap();
    /// ```
    pub fn query<T: DeserializeOwned>(
        &self,
        method: &str,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T> {
        let req = self
            .client
            .request(
                Method::from_str(method)?,
                &format!("{}/{}", self.base_url(), endpoint),
            )
            .headers(self.get_headers());
        let req = match query {
            Some(q) => req.query(&q),
            None => req,
        };
        let mut resp = req.send()?;
        if !resp.status().is_success() {
            anyhow::bail!("Received status code from API: {}", resp.status());
        }
        let resp: T = resp.json()?;
        Ok(resp)
    }

    /// Query a paginated endpoint.
    ///
    /// This is mostly used as an internal method but can be used
    /// by applications for endpoints that don't have wrapper functions.
    ///
    /// This function automatically handles the Twitch API's pagination
    /// feature to return the number of desired items by calling the API
    /// however many times are required, adhering to the endpoint's maximum
    /// number of items per request. Because of this, don't pass in query
    /// params such as "first" or "after"; those are automatically determined
    /// in this function and passed in the HTTP request.
    ///
    /// If you don't want the automatic pagination handling that this function
    /// provides, use the simpler `query` function provided by this library
    /// instead, optionally passing in "first"/"after"/etc. query params.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method string
    /// * `endpoint` - API endpoint (don't include a leading slash)
    /// * `query` - optional query params to include
    /// * `endpoint_maximum` - how many items the endpoint returns per request
    /// * `count` - how many items to get
    ///
    /// # Types
    ///
    /// * `T` - a struct to deserialize the individual data items
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twitch_wrapper::{Twitch, models::streams::StreamListItem};
    /// # let twitch = Twitch::new("abc");
    /// let resp: Vec<StreamListItem> = twitch.query_paginated("GET", "streams", None, 100, 250).unwrap();
    /// ```
    pub fn query_paginated<T: DeserializeOwned>(
        &self,
        method: &str,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        endpoint_maximum: u64,
        count: u64,
    ) -> Result<Vec<T>> {
        let pages_to_request = (count as f64 / endpoint_maximum as f64).ceil() as u64;
        let mut items = vec![];
        let mut after = String::new();
        for i in 0..pages_to_request {
            let req_count = if i + 1 == pages_to_request {
                count - items.len() as u64
            } else {
                endpoint_maximum
            };
            let req_count = format!("{}", req_count);
            let mut all_query: Vec<(&str, &str)> = vec![];
            if let Some(q) = query {
                for pair in q {
                    all_query.push(*pair);
                }
            }
            all_query.push(("first", &req_count));
            all_query.push(("after", &after));
            let raw_data: Value = self.query(method, endpoint, Some(&all_query))?;
            after = raw_data["pagination"]["cursor"]
                .as_str()
                .unwrap()
                .to_owned();
            let raw_data_items = serde_json::to_string(raw_data["data"].as_array().unwrap())?;
            let mut data_items: Vec<T> = serde_json::from_str(&raw_data_items)?;
            items.append(&mut data_items);
        }
        Ok(items)
    }

    /// Get the top streams.
    ///
    /// # Arguments
    ///
    /// * `count` - how many to retrieve
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twitch_wrapper::Twitch;
    /// # let twitch = Twitch::new("abc");
    /// let streams = twitch.get_streams(25).unwrap();
    /// ```
    pub fn get_streams(&self, count: u64) -> Result<Vec<models::streams::StreamListItem>> {
        let data = self.query_paginated("GET", "streams", None, 100, count)?;
        Ok(data)
    }
}

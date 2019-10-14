//! Models for deserializing the Twitch API repsonse JSON into structs.

use serde::{Deserialize, Serialize};

pub mod streams;

/// Struct to hold the pagination information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    /// the pagination cursor string
    pub cursor: String,
}

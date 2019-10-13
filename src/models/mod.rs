use serde::{Deserialize, Serialize};

pub mod streams;

/// Struct to hold the pagination information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    cursor: String,
}

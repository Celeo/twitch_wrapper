use serde::{Deserialize, Serialize};

/// Struct to hold the pagination information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    cursor: String,
}

/// An item in the list of streams.
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamListItem {
    id: String,
    user_id: String,
    user_name: String,
    game_id: String,
    #[serde(rename = "type")]
    type_: String,
    title: String,
    viewer_count: u64,
    started_at: String,
    language: String,
    thumbnail_url: String,
    tag_ids: Vec<String>,
}

/// The list of streams.
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamList {
    data: Vec<StreamListItem>,
    pagination: Pagination,
}

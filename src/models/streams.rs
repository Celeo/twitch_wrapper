//! Models relating to endpoints dealing with streams.

use super::Pagination;
use serde::{Deserialize, Serialize};

/// An item in the list of streams.
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamListItem {
    /// 'id' field
    pub id: String,
    /// 'user_id' field
    pub user_id: String,
    /// 'user_name' field
    pub user_name: String,
    /// 'game_id' field
    pub game_id: String,
    #[serde(rename = "type")]
    /// 'type_' field (gets automatically renamed from 'type')
    pub type_: String,
    /// 'title' field
    pub title: String,
    /// 'viewer_count' field
    pub viewer_count: u64,
    /// 'started_at' field
    pub started_at: String,
    /// 'language' field
    pub language: String,
    /// 'thumbnail_url' field
    pub thumbnail_url: String,
    /// 'tag_ids' field
    pub tag_ids: Vec<String>,
}

/// The list of streams.
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamList {
    /// 'data' field
    pub data: Vec<StreamListItem>,
    /// 'pagination' field
    pub pagination: Pagination,
}

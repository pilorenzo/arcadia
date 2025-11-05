use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Bookmark {
    pub id: i64,
    pub bookmarked_by_id: i32,
    pub bookmarked_torrent_id: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedBookmark {
    pub bookmarked_torrent_id: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedBookmark {
    pub id: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RemoveBookmarkQuery {
    pub id: i64,
}

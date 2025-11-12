use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Bookmark {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub bookmarked_by_id: i32,
    pub bookmarked_title_group_id: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedBookmark {
    pub title_group_id: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedBookmark {
    pub id: i64,
    pub description: Option<String>,
}

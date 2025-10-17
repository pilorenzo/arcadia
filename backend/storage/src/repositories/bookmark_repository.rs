use crate::{
    connection_pool::ConnectionPool,
    models::bookmark::{Bookmark, UserCreatedBookmark},
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_bookmark(
        &self,
        bookmark: &mut UserCreatedBookmark,
        current_user_id: i32,
    ) -> Result<Bookmark> {
        let created_bookmark = sqlx::query_as!(
            Bookmark,
            r#"
                INSERT INTO bookmarks (bookmarked_by_id, bookmarked_torrent_id, description)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            current_user_id,
            bookmark.bookmarked_torrent_id,
            bookmark.description,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateBookmark)?;

        Ok(created_bookmark)
    }
}

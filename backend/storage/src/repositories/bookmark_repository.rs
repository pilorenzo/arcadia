use crate::{
    connection_pool::ConnectionPool,
    models::bookmark::{Bookmark, UserCreatedBookmark},
};
use arcadia_common::error::{Error, Result};
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

    pub async fn find_bookmark(&self, bookmark_id: i64) -> Result<Bookmark> {
        let bookmark = sqlx::query_as!(
            Bookmark,
            r#"
            SELECT
                id, bookmarked_by_id, bookmarked_torrent_id, description
            FROM bookmarks
            WHERE id = $1
            "#,
            bookmark_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindBookmark)?;

        Ok(bookmark)
    }
}

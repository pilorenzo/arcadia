use crate::{
    connection_pool::ConnectionPool,
    models::bookmark::{Bookmark, EditedBookmark, UserCreatedBookmark},
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

    pub async fn update_bookmark(
        &self,
        edited_bookmark: &EditedBookmark,
        bookmark_id: i64,
    ) -> Result<Bookmark> {
        let updated_bookmark = sqlx::query_as!(
            Bookmark,
            r#"
            UPDATE bookmarks
            SET
                description = $2
            WHERE id = $1
            RETURNING
                id, bookmarked_by_id, bookmarked_torrent_id, description
            "#,
            bookmark_id,
            edited_bookmark.description
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingBookmark(e.to_string()))?;

        Ok(updated_bookmark)
    }

    pub async fn delete_bookmark(&self, bookmark_id: i64, current_user_id: i32) -> Result<()> {
        let _ = sqlx::query(
            r#"
                DELETE FROM bookmarks
                WHERE id = $1 AND current_user_id = $2;
            "#,
        )
        .bind(bookmark_id)
        .bind(current_user_id)
        .execute(self.borrow())
        .await?;

        Ok(())
    }
}

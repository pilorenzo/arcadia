use crate::connection_pool::ConnectionPool;
use arcadia_common::error::{Error, Result};
use sqlx::{Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn notify_users_title_group_torrents(
        tx: &mut Transaction<'_, Postgres>,
        title_group_id: i32,
        torrent_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                WITH user_ids AS (
                    SELECT user_id
                    FROM subscriptions_title_group_torrents
                    WHERE title_group_id = $1
                )
                INSERT INTO notifications_title_group_torrents (user_id, torrent_id)
                SELECT
                    user_id,
                    $2
                FROM user_ids
            "#,
            title_group_id,
            torrent_id
        )
        .execute(&mut **tx)
        .await
        .map_err(Error::CouldNotCreateNotification)?;

        Ok(())
    }

    pub async fn find_unread_notifications_amount_title_group_torrents(
        &self,
        user_id: i32,
    ) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM notifications_title_group_torrents
            WHERE user_id = $1 AND read_status = FALSE
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?
        .unwrap_or(0);

        Ok(count)
    }
}

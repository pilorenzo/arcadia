use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct RemoveBookmarkQuery {
    pub id: i64,
}

#[utoipa::path(
    delete,
    operation_id = "Remove bookmark",
    tag = "Bookmark",
    path = "/api/bookmarks",
    params (RemoveBookmarkQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully removed bookmark"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveBookmarkQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool.delete_bookmark(query.id, user.sub).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}

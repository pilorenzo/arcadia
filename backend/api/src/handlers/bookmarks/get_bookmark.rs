use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::bookmark::Bookmark, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetBookmarkQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get bookmarks",
    tag = "Bookmark",
    params(GetBookmarkQuery),
    path = "/api/bookmarks",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Found the bookmark", body=Bookmark),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetBookmarkQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let bookmark = arc.pool.find_bookmark(query.id).await?;

    Ok(HttpResponse::Ok().json(bookmark))
}

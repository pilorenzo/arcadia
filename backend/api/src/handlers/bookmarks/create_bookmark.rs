use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::bookmark::{Bookmark, UserCreatedBookmark},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create bookmark",
    tag = "Bookmark",
    path = "/api/bookmarks",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully created the bookmark", body=Bookmark),)
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    bookmark: Json<UserCreatedBookmark>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let bookmark = arc.pool.create_bookmark(&bookmark, user.sub).await?;

    Ok(HttpResponse::Created().json(bookmark))
}

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::bookmark::{Bookmark, EditedBookmark},
    redis::RedisPoolInterface,
};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;

#[utoipa::path(
    put,
    operation_id = "Edit bookmark",
    tag = "Bookmark",
    path = "/api/bookmarks",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the bookmark", body=Bookmark),
    )
)]

pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedBookmark>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let bookmark = arc.pool.find_bookmark(form.id, user.sub).await?;

    let updated_bookmark = arc.pool.update_bookmark(&form, bookmark.id).await?;

    Ok(HttpResponse::Ok().json(updated_bookmark))
}

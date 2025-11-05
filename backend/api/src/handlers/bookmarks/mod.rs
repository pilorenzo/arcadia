pub mod create_bookmark;
pub mod edit_bookmark;
pub mod get_bookmark;
pub mod remove_bookmark;

use actix_web::web::{delete, get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_bookmark::exec::<R>))
            .route(get().to(self::get_bookmark::exec::<R>))
            .route(put().to(self::edit_bookmark::exec::<R>))
            .route(delete().to(self::remove_bookmark::exec::<R>)),
    );
}

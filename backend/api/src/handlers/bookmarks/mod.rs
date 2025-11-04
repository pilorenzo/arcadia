pub mod create_bookmark;
pub mod get_bookmark;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_bookmark::exec::<R>))
            .route(get().to(self::get_bookmark::exec::<R>)),
    );
}

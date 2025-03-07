use std::sync::Arc;
use crate::{cache::cache_client::CacheClient, routes, BaseCache};
use actix_web::{put, web::{Data, Path, Bytes}, HttpResponse, Responder};

routes! {
    route set_item
}

#[put("/{key:.*}")]
pub async fn set_item(cache: Data<Arc<CacheClient>>, key: Path<String>, body: Bytes) -> impl Responder {
    cache
        .as_ref()
        .clone()
        .set_item(
            Arc::new(key.into_inner()),
            Arc::new(body.into())
        )
        .await;

    HttpResponse::NoContent()
}

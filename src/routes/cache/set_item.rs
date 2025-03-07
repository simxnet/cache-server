use std::sync::Arc;
use crate::{cache::cache_client::CacheClient, routes, BaseCache};
use actix_web::{put, web::{Data, Json, Path}, HttpResponse, Responder};

routes! {
    route set_item
}

#[put("/{key:.*}")]
pub async fn set_item<'c>(cache: Data<Arc<CacheClient>>, key: Path<String>, body: Json<String>) -> impl Responder {
    cache
        .as_ref()
        .clone()
        .set_item(
            Arc::new(key.into_inner()),
            Arc::new(body.into_inner())
        )
        .await;

    HttpResponse::Ok()
        .body("Value stored")
}

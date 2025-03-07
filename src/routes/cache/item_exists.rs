use std::sync::Arc;
use actix_web::{get, web::{Data, Path}, HttpResponse, Responder};
use crate::{routes, BaseCache, CacheClient};

routes! {
    route item_exists
}

#[get("/{key:.*}/exists")]
pub async fn item_exists(cache: Data<Arc<CacheClient>>, key: Path<String>) -> impl Responder {
    match cache.as_ref().clone().has_item(Arc::new(key.into_inner())).await {
        true => HttpResponse::NoContent(),
        false => HttpResponse::NotFound()
    }
}

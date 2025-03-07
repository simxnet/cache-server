use std::sync::Arc;
use actix_web::{get, web::{Data, Path}, HttpResponse, Responder};
use crate::{routes, BaseCache, CacheClient};

routes! {
    route item_exists
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::has_item` as an HTTP wrapper.
///
/// This route checks whether an item exists in the cache instance
/// and returns `204 No Content` if the item exists and `404 Not Found`
/// if it doesn't.
#[get("/{key:.*}/exists")]
pub async fn item_exists(cache: Data<Arc<CacheClient>>, key: Path<String>) -> impl Responder {
    match cache.as_ref().clone().has_item(Arc::new(key.into_inner())).await {
        true => HttpResponse::NoContent(),
        false => HttpResponse::NotFound()
    }
}

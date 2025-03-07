use std::sync::Arc;
use actix_web::{get, web::{Data, Path}, HttpRequest, HttpResponse, Responder};
use log::info;
use crate::{ip_address, routes, BaseCache, CacheClient};

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
pub async fn item_exists(req: HttpRequest, cache: Data<Arc<CacheClient>>, key: Path<String>) -> impl Responder {
    let key = Arc::new(key.into_inner());
    let found = cache.as_ref().clone().has_item(key.clone()).await;

    info!(
        "A request whether the key <{}> exists was made by <{}> and it returned: {}",
        key.clone(),
        ip_address!(req),
        found
            .to_string()
            .to_uppercase()
    );

    match found {
        true => HttpResponse::NoContent(),
        false => HttpResponse::NotFound()
    }
}

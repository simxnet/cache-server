use std::sync::Arc;
use crate::{cache::cache_client::CacheClient, ip_address, routes, BaseCache};
use actix_web::{put, web::{Bytes, Data, Path}, HttpRequest, HttpResponse, Responder};
use log::info;

routes! {
    route set_item
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::set_item` as an HTTP wrapper.
///
/// This route inserts or overwrites an item from the cache collection,
/// the route always returns `204 No Content` because this is not
/// a fallible operation.
#[put("/{key:.*}")]
pub async fn set_item(req: HttpRequest, cache: Data<Arc<CacheClient>>, key: Path<String>, body: Bytes) -> impl Responder {
    let key = Arc::new(key.into_inner());
    let body_length = body.len();

    cache
        .as_ref()
        .clone()
        .set_item(
            key.clone(),
            Arc::new(body.into())
        )
        .await;

    info!("{} bytes where assigned to the key <{}> by <{}>.", body_length, key.clone(), ip_address!(req));

    HttpResponse::NoContent()
}

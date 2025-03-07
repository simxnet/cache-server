use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpResponse};
use crate::{cache::cache_client::CacheClientError, routes, BaseCache, CacheClient};
use actix_error_proc::{proof_route, HttpResult};

routes! {
    route remove_item
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::remove_item` as an HTTP wrapper.
///
/// This route removes an item from the cache collection and
/// returns it with the `200 Ok` along the item in the response
/// body or `404 Not Found` if the item couldn't be found.
#[proof_route(delete("/{key:.*}"))]
pub async fn remove_item(cache: Data<Arc<CacheClient>>, key: Path<String>) -> HttpResult<CacheClientError> {
    let item = cache
        .as_ref()
        .clone()
        .remove_item(Arc::new(key.into_inner()))
        .await?;

    Ok(
        HttpResponse::Ok()
            .body(item.to_vec())
    )
}

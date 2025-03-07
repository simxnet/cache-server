use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpRequest, HttpResponse};
use log::{info, warn};
use crate::{cache::cache_client::CacheClientError, ip_address, routes, BaseCache, CacheClient};
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
pub async fn remove_item(req: HttpRequest, cache: Data<Arc<CacheClient>>, key: Path<String>) -> HttpResult<CacheClientError> {
    let key = Arc::new(key.into_inner());
    let item = cache
        .as_ref()
        .clone()
        .remove_item(key.clone())
        .await
        .inspect_err(|_| warn!(
            "A request to remove the key <{}> from cache by <{}> returned an error, \
likely because there was no associated item to that key.",
            key.clone(),
            ip_address!(req)
        ))?;


    info!("An item with the key <{}> was removed from cache by <{}>.", key.clone(), ip_address!(req));

    Ok(
        HttpResponse::Ok()
            .body(item.to_vec())
    )
}

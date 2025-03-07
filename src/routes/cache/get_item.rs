use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpRequest, HttpResponse};
use log::{info, warn};
use crate::{cache::cache_client::CacheClientError, ip_address, routes, BaseCache, CacheClient};
use actix_error_proc::{proof_route, HttpResult};

routes! {
    route get_item
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::get_item` as an HTTP wrapper.
///
/// This route returns `200 Ok` if the item could be found,
/// otherwise `404 Not Found` as per the `ActixError` error
/// derive on `CacheClientError`.
#[proof_route(get("/{key:.*}"))]
pub async fn get_item(req: HttpRequest, cache: Data<Arc<CacheClient>>, key: Path<String>) -> HttpResult<CacheClientError> {
    let key = Arc::new(key.into_inner());

    let item = cache
        .as_ref()
        .clone()
        .get_item(key.clone())
        .await
        .inspect_err(|_| warn!(
            "A request to obtain the key <{}> from cache by <{}> returned an error, \
likely because there was no associated item to that key.",
            key.clone(),
            ip_address!(req)
        ))?
        .to_vec();

    info!("An item with the key <{}> was obtained from cache by <{}>.", key.clone(), ip_address!(req));

    Ok(
        HttpResponse::Ok()
            .body(item)
    )
}

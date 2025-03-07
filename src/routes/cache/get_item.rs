use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpResponse};
use crate::{cache::cache_client::CacheClientError, routes, BaseCache, CacheClient};
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
pub async fn get_item(cache: Data<Arc<CacheClient>>, key: Path<String>) -> HttpResult<CacheClientError> {
    Ok(
        HttpResponse::Ok()
            .body(
                cache
                    .as_ref()
                    .clone()
                    .get_item(Arc::new(key.into_inner()))
                    .await?
                    .to_vec()
            )
    )
}

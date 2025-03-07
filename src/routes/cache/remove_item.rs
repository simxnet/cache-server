use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpResponse};
use crate::{cache::cache_client::CacheClientError, routes, BaseCache, CacheClient};
use actix_error_proc::{proof_route, HttpResult};

routes! {
    route remove_item
}

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

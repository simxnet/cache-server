use std::sync::Arc;
use actix_web::{web::{Data, Path}, HttpResponse};
use crate::{cache::cache_client::CacheClientError, routes, BaseCache, CacheClient};
use actix_error_proc::{proof_route, HttpResult};

routes! {
    route get_item
}

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
                    .to_string()
            )
    )
}

use std::sync::Arc;
use actix_web::{get, web::Data, HttpResponse, Responder};
use crate::{BaseCache, CacheClient, routes};

routes! {
    route get_keys
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::get_keys` as an HTTP wrapper.
///
/// This route always returns `200 Ok` because the lookup has
/// no checks related to it.
#[get("/")]
pub async fn get_keys(cache: Data<Arc<CacheClient>>) -> impl Responder {
    HttpResponse::Ok()
        .json(
            cache
                .as_ref()
                .clone()
                .keys()
                .await
        )
}

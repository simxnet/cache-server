use std::sync::Arc;
use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use log::info;
use crate::{ip_address, routes, BaseCache, CacheClient};

routes! {
    route get_keys
}

/// This route retrieves a CacheClient instance as a 'service'
/// and acts on behalf of `CacheClient::get_keys` as an HTTP wrapper.
///
/// This route always returns `200 Ok` because the lookup has
/// no checks related to it.
#[get("/")]
pub async fn get_keys(req: HttpRequest, cache: Data<Arc<CacheClient>>) -> impl Responder {
    info!("The cache keys were queryed by <{}>", ip_address!(req));

    HttpResponse::Ok()
        .json(
            cache
                .as_ref()
                .clone()
                .keys()
                .await
        )
}

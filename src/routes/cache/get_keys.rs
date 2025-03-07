use std::sync::Arc;
use actix_web::{get, web::Data, HttpResponse, Responder};
use crate::{BaseCache, CacheClient, routes};

routes! {
    route get_keys
}

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

use actix_web::{HttpResponse, Responder, get, web};

use crate::{BaseCache, Cache, routes};

routes! {
    route get_keys
}

#[get("/")]
pub async fn get_keys(state: web::Data<Cache>) -> impl Responder {
    let keys = state.keys();

    HttpResponse::Ok().json(keys)
}

use actix_web::{HttpResponse, Responder, get, web};

use crate::{BaseCache, Cache, routes};

routes! {
    route get_item
}

#[get("/{key:.*}")]
pub async fn get_item(state: web::Data<Cache>, path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();

    match state.get_item(key) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body("Key not found"),
    }
}

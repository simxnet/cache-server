use crate::{BaseCache, Cache, routes};
use actix_web::{HttpResponse, Responder, put, web};

routes! {
    route set_item
}

#[put("/{key:.*}")]
pub async fn set_item(
    state: web::Data<Cache>,
    path: web::Path<String>,
    body: web::Json<String>,
) -> impl Responder {
    let key = path.into_inner();

    state.set_item(key, body.into_inner());

    HttpResponse::Ok().body("Value stored")
}

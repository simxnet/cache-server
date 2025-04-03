use actix_web::{HttpResponse, Responder, Result, get, web::Data};

use crate::{
    SharedCache,
    cache::BaseDriver,
    util::{error::AppError, macros},
};

macros::routes! {
    route get_keys
}

#[get("/")]
pub async fn get_keys(cache: Data<SharedCache>) -> Result<impl Responder, AppError> {
    let keys = cache.keys();

    Ok(HttpResponse::Ok().json(keys))
}

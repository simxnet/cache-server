use actix_web::{
    HttpResponse, Responder, Result, put,
    web::{Bytes, Data, Path},
};

use crate::{
    SharedCache,
    cache::BaseDriver,
    util::{error::AppError, macros},
};

macros::routes! {
    route set_item
}

#[put("/{key:.*}")]
pub async fn set_item(
    cache: Data<SharedCache>,
    path: Path<String>,
    body: Bytes,
) -> Result<impl Responder, AppError> {
    let key = path.into_inner();

    // living the life
    cache.set_item(key, String::from_utf8(body.to_vec()).unwrap());

    Ok(HttpResponse::Ok().body("ok"))
}

use actix_web::{
    HttpResponse, Responder, Result, delete,
    web::{Data, Path},
};

use crate::{
    SharedCache,
    cache::BaseDriver,
    util::{error::AppError, macros},
};

macros::routes! {
    route remove_item
}

#[delete("/{key:.*}")]
pub async fn remove_item(
    cache: Data<SharedCache>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let key = path.into_inner();

    let item = cache.remove_item(key);

    match item {
        Some(item) => Ok(HttpResponse::Ok().body(item)),
        None => Err(AppError::NotFound("not found".to_string())),
    }
}

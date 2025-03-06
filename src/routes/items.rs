use crate::cache::BaseCache;
use actix_web::{HttpResponse, Responder, get, put, web};
use serde_json::Value;

use crate::{AppState, routes};

// todo: i know this file looks horrible

routes! {
    route get_keys,
    route get_item,
    route set_item
}

#[get("/")]
async fn get_keys(state: web::Data<AppState>) -> impl Responder {
    let keys = state.cache.keys();
    HttpResponse::Ok().json(keys)
}

/// GET /{key} -> Returns the cached value if it exists
#[get("/{key:.*}")]
async fn get_item(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();

    match state.cache.get_item(key) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body("Key not found"),
    }
}

/// PUT /{key} (with JSON body) -> Stores the value in the cache
#[put("/{key:.*}")]
async fn set_item(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<Value>,
) -> impl Responder {
    let key = path.into_inner();
    state.cache.set_item(key, body.into_inner());
    println!("{:#?}", state.cache.keys());
    HttpResponse::Ok().body("Value stored")
}

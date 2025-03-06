use std::sync::Arc;

use actix_web::{App, Error, HttpServer, web};
use cache::{BaseCache, dashmap::CacheClient};

mod cache;
mod routes;
mod util;

pub type Cache = Arc<CacheClient>;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let cache = Arc::new(CacheClient::new());

    HttpServer::new(move || {
        let cache_clone = cache.clone();

        App::new()
            .app_data(web::Data::new(cache_clone))
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}

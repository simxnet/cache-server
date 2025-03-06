use std::sync::Arc;

use actix_web::{App, Error, HttpServer, web};
use cache::{BaseCache, dashmap::CacheClient};

mod cache;
mod routes;
mod util;

pub struct AppState {
    cache: Arc<CacheClient>,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    cache: Arc::new(CacheClient::new()),
                })
                .clone(),
            )
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}

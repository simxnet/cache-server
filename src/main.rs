use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use cache::{BaseDriver, dashmap::DashMapDriver};
use util::logger::init_tracing;

#[macro_use]
extern crate tracing;

mod cache;
mod routes;
mod util;

/// Type for routes
pub type SharedCache = Arc<DashMapDriver<String>>;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init_tracing();

    let addr = ("0.0.0.0", 8000);
    let cache = Arc::new(DashMapDriver::<String>::new());

    info!("Starting server at http://{}:{}", addr.0, addr.1);

    HttpServer::new(move || {
        let cache_clone = cache.clone();

        App::new()
            .app_data(Data::new(cache_clone))
            .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

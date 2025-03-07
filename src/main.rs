use std::{sync::Arc, io::Error as IoError};
use actix_web::{main, web::Data, App, HttpServer};
use cache::{base_cache::BaseCache, cache_client::CacheClient};
use flexi_logger::{Logger, FlexiLoggerError};
use routes::routes as app_routes;
use thiserror::Error;
use util::logging::format_log;

mod cache;
mod routes;
mod util;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error while starting the actix server: {0:#}")]
    Io(#[from] IoError),

    #[error("Error while starting the logger: {0:#}")]
    Logger(#[from] FlexiLoggerError)
}

#[main]
async fn main() -> Result<(), AppError> {
    Logger::try_with_env_or_str("INFO")?
        .format(format_log)
        .start()?;

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Arc::new(CacheClient::new())))
            .configure(app_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}

mod services;

use crate::api::services::alive;
use crate::configurations::load_config;
use actix_web::dev::{Server, Service};
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::io::ErrorKind;
use tracing::{error, info};

pub async fn start_api() -> std::io::Result<()> {
    info!("Starting API...");
    let server: Server;

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to load config",
            ));
        }
    };

    let pool = match PgPoolOptions::new()
        .max_connections(config.db.max_connections)
        .connect(&config.db.url)
        .await
    {
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(
                ErrorKind::NotFound,
                "Failed to connect to database",
            ));
        }
    };

    let config_data = web::Data::new(config);
    let api_bind = config_data.api.bind.clone();
    let api_workers = config_data.api.workers;

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .app_data(pool.clone())
            .service(
                web::scope("/api").service(
                    web::scope("/alive")
                        .service(alive)
                        .wrap_fn(|s, r| r.call(s)),
                ),
            )
    })
    .workers(api_workers);

    server = http_server.bind(api_bind)?.run();
    info!("API started successfully!");
    server.await
}

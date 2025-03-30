mod services;

use crate::api::services::alive;
use crate::configurations::load_config;
use actix_web::dev::{Server, Service};
use actix_web::{App, HttpServer, web};
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

    let config_data = web::Data::new(config);
    let api_bind = config_data.api.bind.clone();
    let api_workers = config_data.api.workers;

    let http_server = HttpServer::new(move || {
        App::new().app_data(config_data.clone()).service(
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

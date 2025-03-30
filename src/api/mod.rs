mod services;

use crate::api::services::alive;
use actix_web::dev::{Server, Service};
use actix_web::{App, HttpServer, web};
use tracing::info;

pub async fn start_api() -> std::io::Result<()> {
    info!("Starting API...");
    let server: Server;

    let api_bind = "127.0.0.1:8080";

    let http_server = HttpServer::new(move || {
        App::new().service(
            web::scope("/api").service(
                web::scope("/alive")
                    .service(alive)
                    .wrap_fn(|s, r| r.call(s)),
            ),
        )
    })
    .workers(32);

    server = http_server.bind(api_bind)?.run();
    info!("API started successfully!");
    server.await
}

use crate::api::start_api;

mod api;
mod configurations;
mod modules;

#[actix_rt::main]
async fn main() {
    if let Err(e) = start_api().await {
        eprintln!("Failed to start API: {}", e);
    };
}

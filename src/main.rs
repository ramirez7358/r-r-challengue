use crate::api::start_api;

mod api;

#[actix_rt::main]
async fn main() {
    if let Err(e) = start_api().await {
        eprintln!("Failed to start API: {}", e);
    };
}

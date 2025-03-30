use actix_web::{HttpResponse, Responder, get};

#[get("")]
pub(super) async fn alive() -> impl Responder {
    let alive = format!(
        "{} Version: {}\nSince: {}",
        "R&R Challengue",
        env!("CARGO_PKG_VERSION"),
        chrono::Local::now().naive_utc().format("%Y-%m-%d %H:%M:%S"),
    );
    HttpResponse::Ok().body(alive)
}

use crate::api::build_json_response;
use crate::modules::transactions::repository::get_all_transactions;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sqlx::PgPool;

#[get("")]
async fn get_transactions(req: HttpRequest) -> impl Responder {
    let pool = match req.app_data::<web::Data<PgPool>>() {
        Some(pool) => pool,
        None => {
            return HttpResponse::InternalServerError().json("Database pool not found");
        }
    };

    match get_all_transactions(pool).await {
        Ok(transactions) => build_json_response(transactions, StatusCode::OK),
        Err(err) => {
            eprintln!("Failed to fetch transactions: {}", err);

            if err.downcast_ref::<sqlx::Error>().is_some() {
                HttpResponse::InternalServerError().json("Database error occurred")
            } else {
                HttpResponse::InternalServerError().json("An unexpected error occurred")
            }
        }
    }
}

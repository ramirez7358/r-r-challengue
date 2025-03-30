use crate::api::build_json_response;
use crate::modules::transactions::repository::get_transactions_by_address;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use rust_decimal::Decimal;
use sqlx::PgPool;

#[get("/balance/{address}")]
pub async fn get_balance(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let pool = match req.app_data::<web::Data<PgPool>>() {
        Some(pool) => pool,
        None => {
            return HttpResponse::InternalServerError().json("Database pool not found");
        }
    };

    let address = path.into_inner();

    let transactions = match get_transactions_by_address(pool, &address).await {
        Ok(transactions) => transactions,
        Err(err) => {
            eprintln!("Failed to fetch transactions: {}", err);

            return if err.downcast_ref::<sqlx::Error>().is_some() {
                HttpResponse::InternalServerError().json("Database error occurred")
            } else {
                HttpResponse::InternalServerError().json("An unexpected error occurred")
            };
        }
    };

    let mut balance = Decimal::new(0, 0);

    for tx in transactions {
        match tx.transaction_type.as_str() {
            "deposit" => balance += tx.amount,
            "withdrawal" => balance -= tx.amount,
            _ => return HttpResponse::InternalServerError().json("Invalid transaction type"),
        }
    }

    build_json_response(balance, StatusCode::OK)
}

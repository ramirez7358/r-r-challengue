use crate::api::build_json_response;
use crate::modules::transactions::repository::{get_all_transactions, get_transactions_by_address};
use crate::modules::transactions::request::CreateTransactionRequest;
use crate::modules::transactions::response::Transaction;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
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

#[get("{address}")]
async fn get_transactions_address(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let pool = match req.app_data::<web::Data<PgPool>>() {
        Some(pool) => pool,
        None => {
            return HttpResponse::InternalServerError().json("Database pool not found");
        }
    };

    let address = path.into_inner();

    match get_transactions_by_address(pool, &address).await {
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

#[post("")]
async fn create_transaction(
    req: HttpRequest,
    body: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    let pool = match req.app_data::<web::Data<PgPool>>() {
        Some(pool) => pool,
        None => {
            return HttpResponse::InternalServerError().json("Database pool not found");
        }
    };

    let transaction: Transaction = body.into_inner().into();

    match transaction.insert(pool).await {
        Ok(id) => build_json_response(id, StatusCode::CREATED),
        Err(e) => {
            eprintln!("Failed to insert transaction: {}", e);
            HttpResponse::InternalServerError().json("Failed to create transaction")
        }
    }
}

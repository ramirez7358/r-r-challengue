use crate::api::build_json_response;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Responder, get, web};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions);
}

#[get("")]
async fn get_transactions(_req: HttpRequest) -> impl Responder {
    build_json_response(vec![""], StatusCode::OK)
}

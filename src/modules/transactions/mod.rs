mod repository;
mod response;
mod services;

use crate::modules::transactions::services::get_transactions;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions);
}

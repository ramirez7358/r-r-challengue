use crate::modules::transactions::services::{
    create_transaction, get_transactions, get_transactions_address,
};
use actix_web::web;

mod repository;
mod request;
mod response;
mod services;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions)
        .service(get_transactions_address)
        .service(create_transaction);
}

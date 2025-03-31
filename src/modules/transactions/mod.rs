use crate::modules::transactions::services::{
    create_transaction, get_transactions, get_transactions_address,
};
use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod repository;
mod request;
pub mod response;
mod services;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions)
        .service(get_transactions_address)
        .service(create_transaction);
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

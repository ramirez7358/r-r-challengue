mod repository;
mod request;
mod response;
mod services;

use crate::modules::transactions::services::{create_transaction, get_transactions};
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions).service(create_transaction);
}

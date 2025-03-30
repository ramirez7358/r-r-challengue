mod services;

use crate::modules::wallet::services::get_balance;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_balance);
}

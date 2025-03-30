use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub address_from: String,
    pub address_to: String,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub transaction_type: String,
    pub created_at: PrimitiveDateTime,
}

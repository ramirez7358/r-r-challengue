use crate::modules::transactions::TransactionType;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateTransactionRequest {
    pub address_from: String,
    pub address_to: String,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
}

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateTransactionRequest {
    pub address_from: String,
    pub address_to: String,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "lowercase")]
pub(crate) enum TransactionType {
    Deposit,
    Withdrawal,
}

use crate::modules::transactions::TransactionType;
use crate::modules::transactions::request::CreateTransactionRequest;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{FromRow, PgPool};
use std::error::Error;
use time::{PrimitiveDateTime, format_description};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub address_from: String,
    pub address_to: String,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_primitive_date"
    )]
    pub created_at: Option<PrimitiveDateTime>,
}

impl Transaction {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, Box<dyn Error>> {
        let id: i32 = sqlx::query_scalar(
            "INSERT INTO transactions (address_from, address_to, amount, type)
             VALUES ($1, $2, $3, $4)
             RETURNING id",
        )
        .bind(&self.address_from)
        .bind(&self.address_to)
        .bind(self.amount)
        .bind(&self.transaction_type)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }
}

impl From<CreateTransactionRequest> for Transaction {
    fn from(request: CreateTransactionRequest) -> Self {
        Transaction {
            id: None,
            address_from: request.address_from,
            address_to: request.address_to,
            amount: request.amount,
            transaction_type: request.transaction_type,
            created_at: None,
        }
    }
}

fn serialize_primitive_date<S>(
    date: &Option<PrimitiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = date {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]")
                .unwrap();
        let date_string = date.format(&format).map_err(serde::ser::Error::custom)?;
        serializer.serialize_some(&date_string)
    } else {
        serializer.serialize_none()
    }
}

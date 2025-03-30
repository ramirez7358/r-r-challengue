use crate::modules::transactions::response::Transaction;
use sqlx::PgPool;
use std::error::Error;

pub(crate) async fn get_all_transactions(
    pool: &PgPool,
) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let transactions = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions")
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

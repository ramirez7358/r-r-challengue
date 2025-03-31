use crate::api::build_json_response;
use crate::modules::transactions::TransactionType;
use crate::modules::transactions::repository::get_transactions_by_address;
use crate::modules::transactions::response::Transaction;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use rust_decimal::Decimal;
use sqlx::PgPool;

#[get("/balance/{address}")]
pub async fn get_balance(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let pool = match req.app_data::<web::Data<PgPool>>() {
        Some(pool) => pool,
        None => {
            return HttpResponse::InternalServerError().json("Database pool not found");
        }
    };

    let address = path.into_inner();

    let transactions = match get_transactions_by_address(pool, &address).await {
        Ok(transactions) => transactions,
        Err(err) => {
            eprintln!("Failed to fetch transactions: {}", err);

            return if err.downcast_ref::<sqlx::Error>().is_some() {
                HttpResponse::InternalServerError().json("Database error occurred")
            } else {
                HttpResponse::InternalServerError().json("An unexpected error occurred")
            };
        }
    };

    let balance = calculate_balance(transactions);

    build_json_response(balance, StatusCode::OK)
}

fn calculate_balance(transactions: Vec<Transaction>) -> Decimal {
    let mut balance = Decimal::new(0, 0);

    for tx in transactions {
        match tx.transaction_type {
            TransactionType::Deposit => balance += tx.amount,
            TransactionType::Withdrawal => balance -= tx.amount,
        }
    }
    balance
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use time::{PrimitiveDateTime, format_description};

    fn create_tx(
        amount: i64,
        transaction_type: TransactionType,
        created_at: Option<&str>,
    ) -> Transaction {
        let created_at_parsed = created_at.map(|date_str| {
            let format =
                format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
            PrimitiveDateTime::parse(date_str, &format).unwrap()
        });

        Transaction {
            id: None,
            address_from: "0x123".to_string(),
            address_to: "0x456".to_string(),
            amount: Decimal::new(amount, 0),
            transaction_type,
            created_at: created_at_parsed,
        }
    }

    #[test]
    fn test_empty_transactions() {
        assert_eq!(calculate_balance(vec![]), Decimal::new(0, 0));
    }

    #[test]
    fn test_only_deposits() {
        let transactions = vec![
            create_tx(100, TransactionType::Deposit, None),
            create_tx(200, TransactionType::Deposit, None),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(300, 0));
    }

    #[test]
    fn test_only_withdrawals() {
        let transactions = vec![
            create_tx(100, TransactionType::Withdrawal, None),
            create_tx(50, TransactionType::Withdrawal, None),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(-150, 0));
    }

    #[test]
    fn test_mixed_transactions() {
        let transactions = vec![
            create_tx(100, TransactionType::Deposit, Some("2025-03-30 12:00:00")),
            create_tx(50, TransactionType::Withdrawal, Some("2025-03-30 13:00:00")),
            create_tx(200, TransactionType::Deposit, Some("2025-03-30 14:00:00")),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(250, 0));
    }

    #[test]
    fn test_balance_zero() {
        let transactions = vec![
            create_tx(150, TransactionType::Deposit, None),
            create_tx(150, TransactionType::Withdrawal, None),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(0, 0));
    }

    #[test]
    fn test_with_created_at_none() {
        let transactions = vec![
            create_tx(100, TransactionType::Deposit, None),
            create_tx(50, TransactionType::Withdrawal, None),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(50, 0));
    }

    #[test]
    fn test_zero_amount_transaction() {
        let transactions = vec![
            create_tx(0, TransactionType::Deposit, None),
            create_tx(0, TransactionType::Withdrawal, None),
        ];
        assert_eq!(calculate_balance(transactions), Decimal::new(0, 0));
    }
}

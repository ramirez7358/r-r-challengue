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

    let balance = calculate_balance(&address, &transactions);

    build_json_response(balance, StatusCode::OK)
}

pub fn calculate_balance(address: &str, transactions: &[Transaction]) -> Decimal {
    let balance = transactions
        .iter()
        .fold(Decimal::new(0, 0), |mut balance, tx| {
            match tx.transaction_type {
                TransactionType::Deposit => {
                    if tx.address_to == address {
                        balance += tx.amount;
                    } else {
                        balance -= tx.amount;
                    }
                }
                TransactionType::Withdrawal => {
                    if tx.address_from == address {
                        balance -= tx.amount;
                    } else {
                        balance += tx.amount;
                    }
                }
            }
            balance
        });

    if balance < Decimal::ZERO {
        Decimal::ZERO
    } else {
        balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use time::{PrimitiveDateTime, format_description};

    const MY_ADDRESS: &str = "0xABC";
    const OTHER_ADDRESS: &str = "0xDEF";

    fn create_tx(
        from: &str,
        to: &str,
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
            address_from: from.to_string(),
            address_to: to.to_string(),
            amount: Decimal::new(amount, 0),
            transaction_type,
            created_at: created_at_parsed,
        }
    }

    #[test]
    fn test_empty_transactions() {
        assert_eq!(calculate_balance(MY_ADDRESS, &vec![]), Decimal::new(0, 0));
    }

    #[test]
    fn test_only_deposits_to_me() {
        let transactions = vec![
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                100,
                TransactionType::Deposit,
                None,
            ),
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                200,
                TransactionType::Deposit,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(300, 0)
        );
    }

    #[test]
    fn test_only_withdrawals_from_me() {
        let transactions = vec![
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                100,
                TransactionType::Withdrawal,
                None,
            ),
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                50,
                TransactionType::Withdrawal,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(-150, 0)
        );
    }

    #[test]
    fn test_mixed_transactions_to_and_from_me() {
        let transactions = vec![
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                100,
                TransactionType::Deposit,
                None,
            ),
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                50,
                TransactionType::Withdrawal,
                None,
            ),
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                200,
                TransactionType::Deposit,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(250, 0)
        );
    }

    #[test]
    fn test_balance_zero_equal_in_and_out() {
        let transactions = vec![
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                150,
                TransactionType::Deposit,
                None,
            ),
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                150,
                TransactionType::Withdrawal,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(0, 0)
        );
    }

    #[test]
    fn test_transactions_not_involving_me() {
        let transactions = vec![
            create_tx("0x111", "0x222", 100, TransactionType::Deposit, None),
            create_tx("0x333", "0x444", 50, TransactionType::Withdrawal, None),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(0, 0)
        );
    }

    #[test]
    fn test_zero_amount_transaction() {
        let transactions = vec![
            create_tx(OTHER_ADDRESS, MY_ADDRESS, 0, TransactionType::Deposit, None),
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                0,
                TransactionType::Withdrawal,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(0, 0)
        );
    }

    #[test]
    fn test_mixed_with_irrelevant_transactions() {
        let transactions = vec![
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                100,
                TransactionType::Deposit,
                None,
            ),
            create_tx(
                MY_ADDRESS,
                OTHER_ADDRESS,
                40,
                TransactionType::Withdrawal,
                None,
            ),
            create_tx("0x999", "0x888", 999, TransactionType::Deposit, None), // no afecta
            create_tx(
                OTHER_ADDRESS,
                MY_ADDRESS,
                60,
                TransactionType::Deposit,
                None,
            ),
        ];
        assert_eq!(
            calculate_balance(MY_ADDRESS, &transactions),
            Decimal::new(120, 0)
        );
    }
}

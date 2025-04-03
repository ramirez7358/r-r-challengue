use crate::modules::transactions::TransactionType;
use crate::modules::transactions::response::Transaction;
use rust_decimal::Decimal;

fn deposit(from: &str, to: &str, amount: i64) -> Transaction {
    Transaction {
        id: None,
        address_from: from.to_string(),
        address_to: to.to_string(),
        amount: Decimal::new(amount, 0),
        transaction_type: TransactionType::Deposit,
        created_at: None,
    }
}

fn withdrawal(from: &str, to: &str, amount: i64) -> Transaction {
    Transaction {
        id: None,
        address_from: from.to_string(),
        address_to: to.to_string(),
        amount: Decimal::new(amount, 0),
        transaction_type: TransactionType::Withdrawal,
        created_at: None,
    }
}

#[test]
fn test_valid_deposit_transaction() {
    let tx = deposit(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        100,
    );
    let errors = tx.validate(&[]);
    assert!(errors.is_empty());
}

#[test]
fn test_valid_withdrawal_with_sufficient_balance() {
    let history = vec![deposit(
        "0xOTHER",
        "0xAAA1111111111111111111111111111111111111",
        200,
    )];
    let tx = withdrawal(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        150,
    );
    let errors = tx.validate(&history);
    assert!(errors.is_empty());
}

#[test]
fn test_withdrawal_insufficient_balance() {
    let history = vec![deposit(
        "0xOTHER",
        "0xAAA1111111111111111111111111111111111111",
        50,
    )];
    let tx = withdrawal(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        100,
    );
    let errors = tx.validate(&history);
    assert_eq!(errors.len(), 1);
    assert!(errors.contains(&"Insufficient balance".to_string()));
}

#[test]
fn test_same_source_and_destination_address() {
    let tx = deposit(
        "0xAAA1111111111111111111111111111111111111",
        "0xAAA1111111111111111111111111111111111111",
        100,
    );
    let errors = tx.validate(&[]);
    assert_eq!(errors.len(), 1);
    assert!(errors.contains(&"Source and destination addresses cannot be the same.".to_string()));
}

#[test]
fn test_invalid_source_address() {
    let tx = withdrawal(
        "invalid_address",
        "0xBBB2222222222222222222222222222222222222",
        100,
    );
    let errors = tx.validate(&[]);
    assert_eq!(errors.len(), 1);
    assert!(errors.contains(&"Invalid source address format.".to_string()));
}

#[test]
fn test_invalid_destination_address() {
    let tx = deposit(
        "0xAAA1111111111111111111111111111111111111",
        "invalid_address",
        100,
    );
    let errors = tx.validate(&[]);
    assert_eq!(errors.len(), 1);
    assert!(errors.contains(&"Invalid destination address format.".to_string()));
}

#[test]
fn test_invalid_both_addresses() {
    let tx = withdrawal("invalid", "invalid", 100);
    let errors = tx.validate(&[]);
    assert!(errors.contains(&"Invalid source address format.".to_string()));
    assert!(errors.contains(&"Invalid destination address format.".to_string()));
    assert!(errors.contains(&"Source and destination addresses cannot be the same.".to_string()));
}

#[test]
fn test_zero_amount_deposit() {
    let tx = deposit(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        0,
    );
    let errors = tx.validate(&[]);
    assert!(errors.contains(&"Transaction amount must be greater than zero.".to_string()));
}

#[test]
fn test_negative_amount_withdrawal() {
    let tx = withdrawal(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        -100,
    );
    let errors = tx.validate(&[]);
    assert!(errors.contains(&"Transaction amount must be greater than zero.".to_string()));
}

#[test]
fn test_multiple_errors_combined() {
    let tx = withdrawal("invalid", "invalid", 0);
    let errors = tx.validate(&[]);
    assert_eq!(errors.len(), 4);
    assert!(errors.contains(&"Invalid source address format.".to_string()));
    assert!(errors.contains(&"Invalid destination address format.".to_string()));
    assert!(errors.contains(&"Transaction amount must be greater than zero.".to_string()));
    assert!(errors.contains(&"Source and destination addresses cannot be the same.".to_string()));
}

#[test]
fn test_deposit_does_not_check_balance() {
    let tx = deposit(
        "0xAAA1111111111111111111111111111111111111",
        "0xBBB2222222222222222222222222222222222222",
        1_000_000,
    );
    let errors = tx.validate(&[]);
    assert!(errors.is_empty());
}

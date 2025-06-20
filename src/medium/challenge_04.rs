#![allow(unused_imports)]
use std::collections::HashMap;

fn validate_and_process(
    processor: &mut TransactionProcessor,
    tx: Transaction
) -> Result<String, ProcessingError> {
    processor.validate_transaction(&tx)
        .map_err(ProcessingError::from)
        .and_then(|_| processor.process_transaction(tx))
}

#[derive(Debug, PartialEq)]
enum ValidationError {
    InsufficientBalance { required: u64, available: u64 },
    AccountNotFound(String),
    InvalidSignature,
    AccountInactive(String),
}

#[derive(Debug, PartialEq)]
enum ProcessingError {
    Validation(ValidationError),
    Network(String),
    Storage(String),
    Timeout,
}

impl From<ValidationError> for ProcessingError {
    fn from(err: ValidationError) -> Self {
        ProcessingError::Validation(err)
    }
}


#[derive(Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    signature: String
}

#[derive(Debug, PartialEq)]
struct Account {
    id: String,
    balance: u64,
    is_active: bool
}

impl Account {
    fn new(id: String, balance: u64) -> Self{
        Account{id, balance, is_active: true}
    }
    
}

struct TransactionProcessor {
    accounts: HashMap<String, Account>,
    min_balance: u64,
}

impl TransactionProcessor {

    fn new(min_balance: u64) -> Self{
        TransactionProcessor{
            accounts: HashMap::new(),
            min_balance
        }
    }

    fn add_account(&mut self, id: &str, balance: u64) {
        let key = id.to_string();
        let account = Account::new(id.to_string(), balance);
        self.accounts.insert(key, account);
    }

    fn process_transaction(&mut self, tx: Transaction) -> Result<String, ProcessingError> {
        self.validate_transaction(&tx)
            .map_err(ProcessingError::from)?;
        self.safe_transfer(&tx.from, &tx.to, tx.amount)?;
        let tx_id = format!("tx_{}_{}_{}", tx.from, tx.to, tx.amount);
        Ok(tx_id)
    }

    fn batch_process(&mut self, transactions: Vec<Transaction>) -> Vec<Result<String, ProcessingError>> {
        transactions.iter()
            .map(|tx| self.process_transaction(tx.clone()))
                .collect()
    }

    fn safe_transfer(&mut self, from_id: &str, to_id: &str, amount: u64) -> Result<(), ProcessingError> {
        // Get mutable source account
        let from_account = self.accounts.get_mut(from_id)
            .ok_or_else(|| ProcessingError::Validation(ValidationError::AccountNotFound(from_id.to_string())))?;

        // Sufficient balance validation (including min_balance) has already been done in `validate_transaction`.
        // If `validate_transaction` passed, `from_account.balance >= amount + self.min_balance`,
        // which implies `from_account.balance >= amount`.
        // Therefore, `checked_sub` here should not fail due to balance < amount.
        let new_from_balance = from_account.balance.checked_sub(amount)
            .expect("Balance already validated; subtraction should not fail due to insufficiency."); // In a real scenario, it could be an internal error if it fails.
        // For this challenge, `expect` is acceptable here given the pre-validation,
        // but an `ok_or` for an internal logic error would be more robust.
        from_account.balance = new_from_balance;

        // Get mutable destination account
        let to_account = self.accounts.get_mut(to_id)
            .ok_or_else(|| ProcessingError::Validation(ValidationError::AccountNotFound(to_id.to_string())))?;

        let new_to_balance = to_account.balance.checked_add(amount)
            .ok_or(ProcessingError::Storage("Overflow when adding balance to the destination account.".to_string()))?;
        to_account.balance = new_to_balance;

        Ok(())
    }




    fn validate_transaction(&self, tx: &Transaction) -> Result<(), ValidationError> {
        self.ensure_valid_signature(&tx.signature)?;
        let from_account = self.ensure_active_account(&tx.from)?;
        self.ensure_account_exists(&tx.to)?;
        self.ensure_sufficient_balance(from_account, tx.amount)?;
        Ok(())
    }
    
    fn ensure_account_exists(&self, id: &str) -> Result<&Account, ValidationError> {
        self.accounts.get(id)
            .ok_or_else(|| ValidationError::AccountNotFound(id.to_string()))
    }

    fn ensure_active_account(&self, id: &str) -> Result<&Account, ValidationError> {
        let account = self.ensure_account_exists(id)?;
        if !account.is_active {
            return Err(ValidationError::AccountInactive(id.to_string()));
        }
        Ok(account)
    }

    fn ensure_sufficient_balance(&self, account: &Account, amount: u64) -> Result<(), ValidationError> {
        let required_total = amount.checked_add(self.min_balance)
            .ok_or(ValidationError::InsufficientBalance { // Error if the required total calculation overflows
                required: u64::MAX, // Or a value indicative of overflow
                available: account.balance,
            })?;

        if account.balance < required_total {
            return Err(ValidationError::InsufficientBalance {
                required: required_total,
                available: account.balance,
            });
        }
        Ok(())
    }


    fn ensure_valid_signature(&self, signature: &str) -> Result<(), ValidationError> {
        if signature != "valid_sig" && signature != "is_valid" {
            return Err(ValidationError::InvalidSignature);
        }
        Ok(())
    }


}
mod tests {
    use crate::medium::challenge_04::{validate_and_process, Account, ProcessingError, Transaction, TransactionProcessor, ValidationError};

    #[test]
    fn add_account_test() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 1000);
        processor.add_account("bob", 500);
        assert!(processor.accounts.get("alice").is_some());
        assert!(processor.accounts.get("bob").is_some());
    }

    #[test]
    fn validate_transaction_test() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 1000);
        processor.add_account("bob", 500);
        
        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 200,
            signature: "is_valid".to_string(),
        };
        let tx_result = processor.validate_transaction(&tx);
        assert!(tx_result.is_ok())
    }

    #[test]
    fn validate_transaction_return_insufficient_balance() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 10);
        processor.add_account("bob", 10);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "is_valid".to_string(),
        };
        let tx_result = processor.validate_transaction(&tx);
        assert!(tx_result.is_err());
        assert_eq!(tx_result.err().unwrap(), ValidationError::InsufficientBalance { 
            required: 20,
            available: 10 
        })
        
    }

    #[test]
    fn validate_transaction_return_account_not_found() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 10);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "is_valid".to_string(),
        };
        let tx_result = processor.validate_transaction(&tx);
        assert!(tx_result.is_err());
        assert_eq!(tx_result.err().unwrap(), ValidationError::AccountNotFound("bob".to_string()));
        }


    #[test]
    fn validate_transaction_return_invalid_signature() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 10);
        processor.add_account("bob", 10);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "no_valid".to_string(),
        };
        let tx_result = processor.validate_transaction(&tx);
        assert!(tx_result.is_err());
        assert_eq!(tx_result.err().unwrap(), ValidationError::InvalidSignature);
    }

    #[test]
    fn process_transaction_test() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 100);
        processor.add_account("bob", 100);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "is_valid".to_string(),
        };
        
        let process_result = validate_and_process(&mut processor, tx);
        assert!(process_result.is_ok());
        assert_eq!(processor.accounts.get("alice").unwrap().balance, 80);
        assert_eq!(processor.accounts.get("bob").unwrap().balance, 120);
    }

    #[test]
    fn process_transaction_return_validation_error() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 10);
        processor.add_account("bob", 100);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "is_valid".to_string(),
        };

        let process_result = validate_and_process(&mut processor, tx);
        assert!(process_result.is_err());
        assert_eq!(process_result.err().unwrap(), ProcessingError::Validation(
            ValidationError::InsufficientBalance {
                required: 20, 
                available: 10
            }));
        
    }

    #[test]
    fn batch_process_with_mixed_results() {
        let mut processor = TransactionProcessor::new(0);
        processor.add_account("alice", 10);
        processor.add_account("bob", 100);

        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 5,
            signature: "is_valid".to_string(),
        };

        let tx1 = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 20,
            signature: "is_valid".to_string(),
        };

        let tx3 = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 5,
            signature: "no_valid".to_string(),
        };


        let transactions = vec![tx, tx1, tx3];
        let batch_result = processor.batch_process(transactions);
        assert_eq!(batch_result.len(), 3);
        assert!(batch_result[0].is_ok());   // Primeira passou
        assert!(batch_result[1].is_err());  // Segunda falhou
        assert!(batch_result[2].is_err());
    }
}
    
    


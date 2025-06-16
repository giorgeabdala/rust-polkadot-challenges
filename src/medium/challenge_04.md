# Challenge 4: Advanced Error Handling

**Estimated Time:** 40 minutes  
**Difficulty:** Medium  
**Topics:** Custom Error Types, Error Propagation, Result Combinators, Error Conversion

## Learning Objectives

By completing this challenge, you will understand:
- Creating custom error types with context
- Error propagation patterns with `?` operator
- Result combinators (`map`, `and_then`, `or_else`)
- Error conversion traits (`From`, `Into`)
- Error handling best practices in systems programming

## Background

Robust error handling is critical in blockchain systems where failures can have significant consequences. Rust's `Result<T, E>` type provides:
- **Explicit error handling**: Errors are part of the type system
- **Composable error handling**: Chain operations safely
- **Zero-cost abstractions**: No runtime overhead
- **Exhaustive error checking**: Compiler ensures all errors are handled

Substrate uses sophisticated error handling throughout its runtime and pallets.

## Challenge

Create a transaction processing system with comprehensive error handling.

### Requirements

1. **Create custom error types:**
   ```rust
   #[derive(Debug, PartialEq)]
   enum ValidationError {
       InvalidAmount(u64),
       InsufficientBalance { required: u64, available: u64 },
       AccountNotFound(String),
       InvalidSignature,
   }

   #[derive(Debug, PartialEq)]
   enum ProcessingError {
       Validation(ValidationError),
       Network(String),
       Storage(String),
       Timeout,
   }
   ```

2. **Create a `Transaction` struct:**
   ```rust
   struct Transaction {
       from: String,
       to: String,
       amount: u64,
       signature: String,
   }
   ```

3. **Create an `Account` struct:**
   ```rust
   struct Account {
       id: String,
       balance: u64,
       is_active: bool,
   }
   ```

4. **Create a `TransactionProcessor` struct:**
   ```rust
   struct TransactionProcessor {
       accounts: HashMap<String, Account>,
       min_balance: u64,
   }
   ```

5. **Implement error handling methods:**
   - `validate_transaction(&self, tx: &Transaction) -> Result<(), ValidationError>`
   - `process_transaction(&mut self, tx: Transaction) -> Result<String, ProcessingError>`
   - `batch_process(&mut self, transactions: Vec<Transaction>) -> Result<Vec<String>, Vec<ProcessingError>>`
   - `safe_transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), ProcessingError>`

### Expected Behavior

```rust
let mut processor = TransactionProcessor::new(100); // min_balance = 100

// Add accounts
processor.add_account("alice", 1000);
processor.add_account("bob", 500);

let tx = Transaction {
    from: "alice".to_string(),
    to: "bob".to_string(),
    amount: 200,
    signature: "valid_sig".to_string(),
};

// Successful processing
match processor.process_transaction(tx) {
    Ok(tx_id) => println!("Transaction processed: {}", tx_id),
    Err(ProcessingError::Validation(ValidationError::InsufficientBalance { required, available })) => {
        println!("Insufficient balance: need {}, have {}", required, available);
    },
    Err(e) => println!("Processing failed: {:?}", e),
}

// Batch processing with partial failures
let transactions = vec![/* multiple transactions */];
let results = processor.batch_process(transactions);
// Handle mixed success/failure results
```

## Advanced Requirements

1. **Implement `From` traits for error conversion:**
   ```rust
   impl From<ValidationError> for ProcessingError {
       fn from(err: ValidationError) -> Self {
           ProcessingError::Validation(err)
       }
   }
   ```

2. **Create helper functions with error combinators:**
   ```rust
   fn validate_and_process(
       processor: &mut TransactionProcessor,
       tx: Transaction
   ) -> Result<String, ProcessingError> {
       processor.validate_transaction(&tx)
           .map_err(ProcessingError::from)
           .and_then(|_| processor.process_transaction(tx))
   }
   ```

3. **Implement retry logic with error handling:**
   ```rust
   fn process_with_retry(
       processor: &mut TransactionProcessor,
       tx: Transaction,
       max_retries: u32
   ) -> Result<String, ProcessingError> {
       // Implement retry logic for network errors
   }
   ```

4. **Create error context helpers:**
   ```rust
   trait ErrorContext<T> {
       fn with_context(self, context: &str) -> Result<T, ProcessingError>;
   }
   ```

## Testing

Write tests that demonstrate:
- Different error types and their propagation
- Error conversion between types
- Result combinators in action
- Batch processing with mixed results
- Error context and debugging information

## Error Handling Patterns

1. **Early Return with `?`:**
   ```rust
   fn validate_transaction(&self, tx: &Transaction) -> Result<(), ValidationError> {
       self.validate_amount(tx.amount)?;
       self.validate_accounts(&tx.from, &tx.to)?;
       self.validate_signature(&tx.signature)?;
       Ok(())
   }
   ```

2. **Error Mapping:**
   ```rust
   fn get_account(&self, id: &str) -> Result<&Account, ProcessingError> {
       self.accounts.get(id)
           .ok_or_else(|| ValidationError::AccountNotFound(id.to_string()))
           .map_err(ProcessingError::from)
   }
   ```

3. **Combining Results:**
   ```rust
   fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), ProcessingError> {
       let from_account = self.get_account(from)?;
       let to_account = self.get_account(to)?;
       // Process transfer
   }
   ```

## Tips

- Use `?` operator for clean error propagation
- Implement `Display` for user-friendly error messages
- Use `thiserror` crate patterns (but implement manually)
- Group related errors in enums
- Provide context with error variants

## Key Learning Points

- **Error Types**: Designing informative error hierarchies
- **Error Propagation**: Using `?` operator effectively
- **Error Conversion**: Automatic conversion between error types
- **Result Combinators**: Functional error handling patterns
- **Error Context**: Providing useful debugging information

## Substrate Connection

Substrate's error handling patterns:
- `DispatchError` for runtime errors
- Pallet-specific error enums
- `ensure!` macro for validation
- `Result<DispatchResult, DispatchError>` return types
- Error conversion in cross-pallet calls

## Bonus Challenges

1. Create a `TransactionError` that includes transaction context
2. Implement error recovery strategies for different error types
3. Add logging integration with error levels
4. Create a generic `Retryable<T, E>` wrapper for retry logic
5. Implement error aggregation for batch operations
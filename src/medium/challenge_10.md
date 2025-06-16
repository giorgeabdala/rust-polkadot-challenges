# Challenge 10: Testing and Documentation

**Estimated Time:** 35 minutes  
**Difficulty:** Medium  
**Topics:** Unit Testing, Integration Testing, Documentation, Benchmarking, Test-Driven Development

## Learning Objectives

By completing this challenge, you will understand:
- Writing comprehensive unit and integration tests
- Using Rust's built-in testing framework
- Creating effective documentation with examples
- Benchmarking code performance
- Test-driven development practices

## Background

Testing and documentation are crucial for maintainable software:
- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test component interactions
- **Documentation**: Explain code purpose and usage
- **Benchmarks**: Measure and optimize performance
- **Examples**: Demonstrate real-world usage

Substrate has extensive testing and documentation practices that ensure reliability.

## Challenge

Create a well-tested and documented blockchain account management system.

### Requirements

1. **Create the core data structures:**
   ```rust
   /// Represents a unique account identifier
   #[derive(Debug, Clone, PartialEq, Eq, Hash)]
   pub struct AccountId(pub String);

   /// Account information with balance and metadata
   #[derive(Debug, Clone, PartialEq)]
   pub struct Account {
       pub id: AccountId,
       pub balance: u64,
       pub nonce: u32,
       pub is_frozen: bool,
   }

   /// Errors that can occur during account operations
   #[derive(Debug, PartialEq)]
   pub enum AccountError {
       AccountNotFound(AccountId),
       InsufficientBalance { required: u64, available: u64 },
       AccountFrozen(AccountId),
       InvalidAmount(u64),
   }
   ```

2. **Create the `AccountManager` with full documentation:**
   ```rust
   /// Manages blockchain accounts with balance tracking and validation
   /// 
   /// The `AccountManager` provides thread-safe operations for:
   /// - Creating and managing accounts
   /// - Transferring funds between accounts
   /// - Freezing/unfreezing accounts
   /// - Querying account information
   /// 
   /// # Examples
   /// 
   /// ```
   /// use account_manager::{AccountManager, AccountId};
   /// 
   /// let mut manager = AccountManager::new();
   /// let alice = AccountId("alice".to_string());
   /// 
   /// // Create account with initial balance
   /// manager.create_account(alice.clone(), 1000).unwrap();
   /// 
   /// // Check balance
   /// assert_eq!(manager.get_balance(&alice).unwrap(), 1000);
   /// ```
   pub struct AccountManager {
       accounts: HashMap<AccountId, Account>,
       total_supply: u64,
   }
   ```

3. **Implement methods with comprehensive documentation:**
   - `new() -> Self`
   - `create_account(&mut self, id: AccountId, initial_balance: u64) -> Result<(), AccountError>`
   - `get_account(&self, id: &AccountId) -> Option<&Account>`
   - `get_balance(&self, id: &AccountId) -> Result<u64, AccountError>`
   - `transfer(&mut self, from: &AccountId, to: &AccountId, amount: u64) -> Result<(), AccountError>`
   - `freeze_account(&mut self, id: &AccountId) -> Result<(), AccountError>`
   - `unfreeze_account(&mut self, id: &AccountId) -> Result<(), AccountError>`
   - `get_total_supply(&self) -> u64`

4. **Write comprehensive tests:**
   - Unit tests for each method
   - Integration tests for complex workflows
   - Property-based tests for invariants
   - Error condition tests
   - Performance benchmarks

### Expected Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account_success() {
        let mut manager = AccountManager::new();
        let alice = AccountId("alice".to_string());
        
        let result = manager.create_account(alice.clone(), 1000);
        assert!(result.is_ok());
        assert_eq!(manager.get_balance(&alice).unwrap(), 1000);
        assert_eq!(manager.get_total_supply(), 1000);
    }

    #[test]
    fn test_transfer_success() {
        let mut manager = AccountManager::new();
        let alice = AccountId("alice".to_string());
        let bob = AccountId("bob".to_string());
        
        manager.create_account(alice.clone(), 1000).unwrap();
        manager.create_account(bob.clone(), 500).unwrap();
        
        let result = manager.transfer(&alice, &bob, 200);
        assert!(result.is_ok());
        assert_eq!(manager.get_balance(&alice).unwrap(), 800);
        assert_eq!(manager.get_balance(&bob).unwrap(), 700);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut manager = AccountManager::new();
        let alice = AccountId("alice".to_string());
        let bob = AccountId("bob".to_string());
        
        manager.create_account(alice.clone(), 100).unwrap();
        manager.create_account(bob.clone(), 0).unwrap();
        
        let result = manager.transfer(&alice, &bob, 200);
        assert_eq!(result, Err(AccountError::InsufficientBalance { 
            required: 200, 
            available: 100 
        }));
    }

    // Property-based test
    #[test]
    fn test_total_supply_invariant() {
        let mut manager = AccountManager::new();
        let accounts: Vec<_> = (0..10)
            .map(|i| AccountId(format!("account_{}", i)))
            .collect();
        
        // Create accounts with random balances
        let mut expected_total = 0;
        for (i, account) in accounts.iter().enumerate() {
            let balance = (i + 1) * 100;
            manager.create_account(account.clone(), balance as u64).unwrap();
            expected_total += balance as u64;
        }
        
        assert_eq!(manager.get_total_supply(), expected_total);
        
        // Perform transfers and verify total supply remains constant
        for i in 0..5 {
            manager.transfer(&accounts[i], &accounts[i + 5], 50).unwrap();
        }
        
        assert_eq!(manager.get_total_supply(), expected_total);
    }
}
```

## Advanced Requirements

1. **Create integration tests in `tests/` directory:**
   ```rust
   // tests/integration_test.rs
   use account_manager::*;
   
   #[test]
   fn test_complex_workflow() {
       // Test realistic usage scenarios
   }
   
   #[test]
   fn test_concurrent_access() {
       // Test thread safety if applicable
   }
   ```

2. **Add benchmarks:**
   ```rust
   // benches/account_benchmark.rs
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   use account_manager::*;
   
   fn benchmark_account_creation(c: &mut Criterion) {
       c.bench_function("create_account", |b| {
           b.iter(|| {
               let mut manager = AccountManager::new();
               for i in 0..1000 {
                   let id = AccountId(format!("account_{}", i));
                   manager.create_account(black_box(id), black_box(1000)).unwrap();
               }
           })
       });
   }
   
   fn benchmark_transfers(c: &mut Criterion) {
       let mut manager = AccountManager::new();
       // Setup accounts...
       
       c.bench_function("transfer", |b| {
           b.iter(|| {
               manager.transfer(
                   black_box(&AccountId("alice".to_string())),
                   black_box(&AccountId("bob".to_string())),
                   black_box(10)
               ).unwrap();
           })
       });
   }
   
   criterion_group!(benches, benchmark_account_creation, benchmark_transfers);
   criterion_main!(benches);
   ```

3. **Add comprehensive documentation:**
   ```rust
   /// Transfers funds between two accounts
   /// 
   /// This method performs a secure transfer of funds from one account to another,
   /// with comprehensive validation and error handling.
   /// 
   /// # Arguments
   /// 
   /// * `from` - The account to transfer funds from
   /// * `to` - The account to transfer funds to  
   /// * `amount` - The amount to transfer (must be > 0)
   /// 
   /// # Returns
   /// 
   /// * `Ok(())` - Transfer completed successfully
   /// * `Err(AccountError)` - Transfer failed due to validation error
   /// 
   /// # Errors
   /// 
   /// This method will return an error if:
   /// * Either account doesn't exist (`AccountNotFound`)
   /// * Source account has insufficient balance (`InsufficientBalance`)
   /// * Either account is frozen (`AccountFrozen`)
   /// * Amount is zero (`InvalidAmount`)
   /// 
   /// # Examples
   /// 
   /// ```
   /// # use account_manager::{AccountManager, AccountId};
   /// let mut manager = AccountManager::new();
   /// let alice = AccountId("alice".to_string());
   /// let bob = AccountId("bob".to_string());
   /// 
   /// manager.create_account(alice.clone(), 1000).unwrap();
   /// manager.create_account(bob.clone(), 0).unwrap();
   /// 
   /// // Transfer 100 units from Alice to Bob
   /// manager.transfer(&alice, &bob, 100).unwrap();
   /// 
   /// assert_eq!(manager.get_balance(&alice).unwrap(), 900);
   /// assert_eq!(manager.get_balance(&bob).unwrap(), 100);
   /// ```
   /// 
   /// # Panics
   /// 
   /// This method does not panic under normal circumstances.
   pub fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: u64) -> Result<(), AccountError> {
       // Implementation...
   }
   ```

## Testing Patterns

1. **Arrange-Act-Assert Pattern:**
   ```rust
   #[test]
   fn test_freeze_account() {
       // Arrange
       let mut manager = AccountManager::new();
       let alice = AccountId("alice".to_string());
       manager.create_account(alice.clone(), 1000).unwrap();
       
       // Act
       let result = manager.freeze_account(&alice);
       
       // Assert
       assert!(result.is_ok());
       let account = manager.get_account(&alice).unwrap();
       assert!(account.is_frozen);
   }
   ```

2. **Test Fixtures:**
   ```rust
   fn setup_test_accounts() -> (AccountManager, AccountId, AccountId) {
       let mut manager = AccountManager::new();
       let alice = AccountId("alice".to_string());
       let bob = AccountId("bob".to_string());
       
       manager.create_account(alice.clone(), 1000).unwrap();
       manager.create_account(bob.clone(), 500).unwrap();
       
       (manager, alice, bob)
   }
   
   #[test]
   fn test_with_fixture() {
       let (mut manager, alice, bob) = setup_test_accounts();
       // Test logic...
   }
   ```

3. **Parameterized Tests:**
   ```rust
   #[test]
   fn test_invalid_amounts() {
       let test_cases = vec![0, u64::MAX];
       
       for invalid_amount in test_cases {
           let mut manager = AccountManager::new();
           let alice = AccountId("alice".to_string());
           
           let result = manager.create_account(alice, invalid_amount);
           assert!(matches!(result, Err(AccountError::InvalidAmount(_))));
       }
   }
   ```

## Tips

- Write tests before implementing functionality (TDD)
- Use descriptive test names that explain the scenario
- Test both success and failure cases
- Include examples in documentation
- Use `cargo test` and `cargo doc` regularly
- Consider edge cases and boundary conditions

## Key Learning Points

- **Test Organization**: Structuring tests for maintainability
- **Documentation**: Writing clear, helpful documentation
- **Error Testing**: Verifying error conditions work correctly
- **Performance Testing**: Using benchmarks to measure performance
- **Test Coverage**: Ensuring comprehensive test coverage

## Substrate Connection

Substrate's testing practices:
- Extensive unit tests for all pallets
- Integration tests for runtime behavior
- Mock runtime for testing pallets in isolation
- Benchmarking for weight calculation
- Documentation with examples for all public APIs

## Bonus Challenges

1. Add property-based testing with `proptest` crate
2. Create mock objects for testing complex interactions
3. Implement test coverage reporting
4. Add stress tests for concurrent operations
5. Create automated documentation generation with custom examples 
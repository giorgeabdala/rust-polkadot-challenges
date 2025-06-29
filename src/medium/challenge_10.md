# Challenge 10: Basic Testing and Documentation

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Unit Testing, Documentation, Test-Driven Development

## Learning Objectives

By completing this challenge, you will understand:
- Writing basic unit tests with Rust's testing framework
- Creating clear documentation with examples
- Test-driven development practices
- Testing error conditions and edge cases

## Background

Testing and documentation are crucial for maintainable software:
- **Unit Tests**: Test individual functions and methods
- **Documentation**: Explain code purpose and usage with examples
- **Test-Driven Development**: Write tests before implementation
- **Error Testing**: Verify error conditions work correctly

Substrate has extensive testing practices that ensure reliability.

## Challenge

Create a well-tested and documented simple account management system.

### Structures to Implement

#### **Basic Data Types:**
```rust
use std::collections::HashMap;

/// Represents a unique account identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub String);

/// Account information with balance and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: AccountId,
    pub balance: u64,
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

#### **Account Manager Structure:**
```rust
/// Manages blockchain accounts with balance tracking and validation
/// 
/// The `AccountManager` provides operations for:
/// - Creating and managing accounts
/// - Transferring funds between accounts
/// - Freezing/unfreezing accounts
/// - Querying account information
/// 
/// # Examples
/// 
/// ```
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

### Provided Implementations

#### **Basic Constructors:**
```rust
impl AccountId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Account {
    pub fn new(id: AccountId, balance: u64) -> Self {
        Self {
            id,
            balance,
            is_frozen: false,
        }
    }
}

impl AccountManager {
    /// Creates a new empty account manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            total_supply: 0,
        }
    }
    
    /// Returns the total supply of all accounts
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }
}
```

### Methods for You to Implement

#### **1. Create Account (`create_account`):**
```rust
impl AccountManager {
    /// Creates a new account with the specified initial balance
    /// 
    /// # Arguments
    /// 
    /// * `id` - The account identifier
    /// * `initial_balance` - The starting balance (must be > 0)
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Account created successfully
    /// * `Err(AccountError)` - Account already exists or invalid balance
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut manager = AccountManager::new();
    /// let alice = AccountId("alice".to_string());
    /// 
    /// manager.create_account(alice, 1000).unwrap();
    /// ```
    // TODO: Implement this method
    pub fn create_account(&mut self, id: AccountId, initial_balance: u64) -> Result<(), AccountError> {
        // IMPLEMENT:
        // 1. Check if initial_balance is 0 (return InvalidAmount error)
        // 2. Check if account already exists (return AccountError - use a custom variant)
        // 3. Create new Account and insert into accounts HashMap
        // 4. Update total_supply
        // 5. Return Ok(())
        todo!()
    }
}
```

#### **2. Get Account (`get_account`):**
```rust
impl AccountManager {
    /// Gets a reference to an account by ID
    /// 
    /// # Arguments
    /// 
    /// * `id` - The account identifier to look up
    /// 
    /// # Returns
    /// 
    /// * `Some(&Account)` - Account found
    /// * `None` - Account not found
    // TODO: Implement this method
    pub fn get_account(&self, id: &AccountId) -> Option<&Account> {
        // IMPLEMENT:
        // Return accounts.get(id)
        todo!()
    }
}
```

#### **3. Get Balance (`get_balance`):**
```rust
impl AccountManager {
    /// Gets the balance of an account
    /// 
    /// # Arguments
    /// 
    /// * `id` - The account identifier
    /// 
    /// # Returns
    /// 
    /// * `Ok(balance)` - Account balance
    /// * `Err(AccountNotFound)` - Account doesn't exist
    // TODO: Implement this method
    pub fn get_balance(&self, id: &AccountId) -> Result<u64, AccountError> {
        // IMPLEMENT:
        // 1. Get account using get_account
        // 2. Return Ok(account.balance) or Err(AccountNotFound)
        todo!()
    }
}
```

#### **4. Transfer Funds (`transfer`):**
```rust
impl AccountManager {
    /// Transfers funds between two accounts
    /// 
    /// # Arguments
    /// 
    /// * `from` - Source account ID
    /// * `to` - Destination account ID  
    /// * `amount` - Amount to transfer
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Transfer successful
    /// * `Err(AccountError)` - Transfer failed
    // TODO: Implement this method
    pub fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: u64) -> Result<(), AccountError> {
        // IMPLEMENT:
        // 1. Check if amount is 0 (return InvalidAmount)
        // 2. Check if both accounts exist (return AccountNotFound)
        // 3. Check if from account is frozen (return AccountFrozen)
        // 4. Check if from account has sufficient balance (return InsufficientBalance)
        // 5. Update balances: subtract from source, add to destination
        // 6. Return Ok(())
        todo!()
    }
}
```

#### **5. Freeze Account (`freeze_account`):**
```rust
impl AccountManager {
    /// Freezes an account, preventing transfers from it
    /// 
    /// # Arguments
    /// 
    /// * `id` - The account identifier to freeze
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Account frozen successfully  
    /// * `Err(AccountNotFound)` - Account doesn't exist
    // TODO: Implement this method
    pub fn freeze_account(&mut self, id: &AccountId) -> Result<(), AccountError> {
        // IMPLEMENT:
        // 1. Get mutable reference to account
        // 2. Set is_frozen to true
        // 3. Return Ok(()) or AccountNotFound error
        todo!()
    }
}
```

### Tests to Implement

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account_success() {
        // TODO: Implement this test
        // 1. Create AccountManager
        // 2. Create account with valid balance
        // 3. Verify account was created correctly
        // 4. Check total supply updated
        todo!()
    }

    #[test]
    fn test_create_account_invalid_balance() {
        // TODO: Implement this test
        // 1. Try to create account with 0 balance
        // 2. Verify it returns InvalidAmount error
        todo!()
    }

    #[test]
    fn test_get_balance_success() {
        // TODO: Implement this test
        // 1. Create account with known balance
        // 2. Get balance and verify it matches
        todo!()
    }

    #[test]
    fn test_get_balance_account_not_found() {
        // TODO: Implement this test  
        // 1. Try to get balance of non-existent account
        // 2. Verify it returns AccountNotFound error
        todo!()
    }

    #[test]
    fn test_transfer_success() {
        // TODO: Implement this test
        // 1. Create two accounts with balances
        // 2. Transfer amount between them
        // 3. Verify balances updated correctly
        // 4. Verify total supply unchanged
        todo!()
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        // TODO: Implement this test
        // 1. Create account with low balance
        // 2. Try to transfer more than available
        // 3. Verify it returns InsufficientBalance error
        todo!()
    }

    #[test]
    fn test_freeze_account() {
        // TODO: Implement this test
        // 1. Create account
        // 2. Freeze it
        // 3. Try to transfer from frozen account
        // 4. Verify transfer fails with AccountFrozen error
        todo!()
    }

    #[test]
    fn test_edge_cases() {
        // TODO: Implement this test
        // Test various edge cases:
        // - Transfer to same account
        // - Transfer 0 amount
        // - Very large balances
        todo!()
    }
}
```

### Testing Patterns

#### **1. Arrange-Act-Assert Pattern:**
```rust
#[test]
fn test_example() {
    // Arrange
    let mut manager = AccountManager::new();
    let alice = AccountId("alice".to_string());
    
    // Act
    let result = manager.create_account(alice.clone(), 1000);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(manager.get_balance(&alice).unwrap(), 1000);
}
```

#### **2. Error Testing:**
```rust
#[test]
fn test_error_conditions() {
    let manager = AccountManager::new();
    let non_existent = AccountId("does_not_exist".to_string());
    
    let result = manager.get_balance(&non_existent);
    assert_eq!(result, Err(AccountError::AccountNotFound(non_existent)));
}
```

#### **3. Test Helper Functions:**
```rust
fn create_test_accounts() -> (AccountManager, AccountId, AccountId) {
    let mut manager = AccountManager::new();
    let alice = AccountId("alice".to_string());
    let bob = AccountId("bob".to_string());
    
    manager.create_account(alice.clone(), 1000).unwrap();
    manager.create_account(bob.clone(), 500).unwrap();
    
    (manager, alice, bob)
}
```

### Example Usage

```rust
fn main() {
    let mut manager = AccountManager::new();
    
    // Create accounts
    let alice = AccountId("alice".to_string());
    let bob = AccountId("bob".to_string());
    
    manager.create_account(alice.clone(), 1000).unwrap();
    manager.create_account(bob.clone(), 500).unwrap();
    
    println!("Alice balance: {}", manager.get_balance(&alice).unwrap());
    println!("Bob balance: {}", manager.get_balance(&bob).unwrap());
    
    // Transfer funds
    manager.transfer(&alice, &bob, 200).unwrap();
    
    println!("After transfer:");
    println!("Alice balance: {}", manager.get_balance(&alice).unwrap());
    println!("Bob balance: {}", manager.get_balance(&bob).unwrap());
    
    // Test freezing
    manager.freeze_account(&alice).unwrap();
    let freeze_result = manager.transfer(&alice, &bob, 100);
    println!("Transfer from frozen account: {:?}", freeze_result);
}
```

### Expected Output

A well-tested account management system that:
- Demonstrates comprehensive unit testing patterns
- Includes clear documentation with examples
- Tests both success and error conditions
- Shows proper error handling and validation
- Uses descriptive test names and clear assertions

### Theoretical Context

**Testing Fundamentals:**
- **Unit Tests**: Test individual functions in isolation
- **Test Organization**: Group related tests in modules
- **Assertions**: Verify expected behavior with `assert!` macros
- **Error Testing**: Verify error conditions are handled correctly

**Documentation Patterns:**
- **Doc Comments**: Use `///` for public API documentation
- **Examples**: Include working code examples in docs
- **Arguments/Returns**: Document parameters and return values
- **Error Conditions**: Document when errors can occur

**Key Testing Principles:**
1. **Arrange-Act-Assert**: Structure tests clearly
2. **Single Responsibility**: One test per behavior
3. **Descriptive Names**: Test names explain what is being tested
4. **Edge Cases**: Test boundary conditions and error paths

**Substrate Connection:**
- All Substrate pallets include extensive unit tests
- Documentation with examples for all public APIs
- Test-driven development for runtime functionality
- Error handling patterns for blockchain operations

This challenge teaches essential testing and documentation patterns needed for professional Substrate development.

--- 
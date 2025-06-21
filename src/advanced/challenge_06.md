## Challenge 6: Unsigned Transactions with Validation

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement an unsigned transaction system with spam prevention and validation mechanisms. This challenge focuses on understanding how Substrate handles transactions that don't require signatures and how to implement proper validation to prevent network abuse.

**Main Concepts Covered:**
1. **Unsigned Transactions:** Transactions without cryptographic signatures
2. **Validation Logic:** Preventing spam and ensuring transaction validity
3. **Block Intervals:** Time-based transaction restrictions
4. **State Management:** Tracking transaction history and limits
5. **Error Handling:** Proper validation error management

### Detailed Structures to Implement:

#### **Validation Result Types:**
```rust
/// Result of transaction validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Transaction is valid and should be included
    Valid,
    /// Transaction is invalid and should be rejected
    Invalid(ValidationError),
}

/// Validation error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Too many transactions in current interval
    TooManyTransactions,
    /// Transaction submitted too early
    TooEarly,
    /// Invalid transaction data
    InvalidData(String),
    /// Duplicate transaction
    Duplicate,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TooManyTransactions => write!(f, "Too many transactions in current interval"),
            ValidationError::TooEarly => write!(f, "Transaction submitted too early"),
            ValidationError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ValidationError::Duplicate => write!(f, "Duplicate transaction"),
        }
    }
}
```

#### **Block Simulator:**
```rust
/// Simulates blockchain block progression
pub struct BlockSimulator {
    current_block: u64,
    block_time: u64, // seconds per block
}

impl BlockSimulator {
    pub fn new(block_time: u64) -> Self {
        Self {
            current_block: 1,
            block_time,
        }
    }
    
    /// Get current block number
    pub fn current_block(&self) -> u64 {
        self.current_block
    }
    
    /// Advance to next block
    pub fn next_block(&mut self) {
        self.current_block = self.current_block.saturating_add(1);
    }
    
    /// Advance multiple blocks
    pub fn advance_blocks(&mut self, count: u64) {
        self.current_block = self.current_block.saturating_add(count);
    }
    
    /// Check if enough blocks have passed since last block
    pub fn blocks_since(&self, last_block: u64) -> u64 {
        self.current_block.saturating_sub(last_block)
    }
    
    /// Get block time in seconds
    pub fn block_time(&self) -> u64 {
        self.block_time
    }
}
```

#### **Unsigned Transaction Definition:**
```rust
/// Unsigned transaction for data submission
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedTransaction<T> {
    /// Transaction data
    pub data: T,
    /// Block number when transaction was created
    pub block_number: u64,
    /// Unique identifier to prevent duplicates
    pub nonce: u64,
}

impl<T> UnsignedTransaction<T> {
    pub fn new(data: T, block_number: u64, nonce: u64) -> Self {
        Self {
            data,
            block_number,
            nonce,
        }
    }
}
```

#### **Transaction Validator:**
```rust
use std::collections::{HashMap, HashSet};

/// Validates unsigned transactions and prevents spam
pub struct TransactionValidator {
    /// Maximum transactions per interval
    max_per_interval: u32,
    /// Block interval for rate limiting
    interval_blocks: u64,
    /// Minimum blocks between submissions
    min_block_interval: u64,
    /// Track transactions per interval
    interval_counts: HashMap<u64, u32>,
    /// Track last submission block
    last_submission: Option<u64>,
    /// Track used nonces to prevent duplicates
    used_nonces: HashSet<u64>,
    /// Block simulator reference
    block_simulator: BlockSimulator,
}

impl TransactionValidator {
    pub fn new(
        max_per_interval: u32,
        interval_blocks: u64,
        min_block_interval: u64,
        block_simulator: BlockSimulator,
    ) -> Self {
        Self {
            max_per_interval,
            interval_blocks,
            min_block_interval,
            interval_counts: HashMap::new(),
            last_submission: None,
            used_nonces: HashSet::new(),
            block_simulator,
        }
    }
    
    /// Validate an unsigned transaction
    pub fn validate_transaction<T>(
        &mut self,
        transaction: &UnsignedTransaction<T>,
    ) -> ValidationResult
    where
        T: std::fmt::Debug,
    {
        let current_block = self.block_simulator.current_block();
        
        // Check minimum block interval
        if let Some(last_block) = self.last_submission {
            let blocks_since = self.block_simulator.blocks_since(last_block);
            if blocks_since < self.min_block_interval {
                return ValidationResult::Invalid(ValidationError::TooEarly);
            }
        }
        
        // Check for duplicate nonce
        if self.used_nonces.contains(&transaction.nonce) {
            return ValidationResult::Invalid(ValidationError::Duplicate);
        }
        
        // Check rate limiting
        let interval_start = self.get_interval_start(current_block);
        let current_count = self.interval_counts.get(&interval_start).unwrap_or(&0);
        
        if *current_count >= self.max_per_interval {
            return ValidationResult::Invalid(ValidationError::TooManyTransactions);
        }
        
        ValidationResult::Valid
    }
    
    /// Accept a valid transaction (update internal state)
    pub fn accept_transaction<T>(&mut self, transaction: &UnsignedTransaction<T>) -> Result<(), ValidationError>
    where
        T: std::fmt::Debug,
    {
        let current_block = self.block_simulator.current_block();
        
        // Validate first
        match self.validate_transaction(transaction) {
            ValidationResult::Valid => {},
            ValidationResult::Invalid(error) => return Err(error),
        }
        
        // Update state
        let interval_start = self.get_interval_start(current_block);
        let count = self.interval_counts.entry(interval_start).or_insert(0);
        *count += 1;
        
        self.last_submission = Some(current_block);
        self.used_nonces.insert(transaction.nonce);
        
        Ok(())
    }
    
    /// Get the start block of current interval
    fn get_interval_start(&self, block_number: u64) -> u64 {
        (block_number / self.interval_blocks) * self.interval_blocks
    }
    
    /// Clean up old interval data
    pub fn cleanup_old_intervals(&mut self) {
        let current_block = self.block_simulator.current_block();
        let cutoff = current_block.saturating_sub(self.interval_blocks * 2);
        
        self.interval_counts.retain(|&interval_start, _| interval_start > cutoff);
    }
    
    /// Get current interval statistics
    pub fn get_interval_stats(&self) -> (u64, u32, u32) {
        let current_block = self.block_simulator.current_block();
        let interval_start = self.get_interval_start(current_block);
        let current_count = self.interval_counts.get(&interval_start).unwrap_or(&0);
        
        (interval_start, *current_count, self.max_per_interval)
    }
    
    /// Get next allowed submission block
    pub fn next_allowed_block(&self) -> Option<u64> {
        self.last_submission.map(|last| last + self.min_block_interval)
    }
    
    /// Get blocks until next allowed submission
    pub fn blocks_until_allowed(&self) -> u64 {
        match self.next_allowed_block() {
            Some(next_allowed) => {
                let current = self.block_simulator.current_block();
                if next_allowed > current {
                    next_allowed - current
                } else {
                    0
                }
            },
            None => 0,
        }
    }
    
    /// Get mutable reference to block simulator
    pub fn block_simulator_mut(&mut self) -> &mut BlockSimulator {
        &mut self.block_simulator
    }
    
    /// Get reference to block simulator
    pub fn block_simulator(&self) -> &BlockSimulator {
        &self.block_simulator
    }
}
```

#### **Unsigned Transaction Pallet:**
```rust
/// Pallet that handles unsigned transactions
pub struct UnsignedPallet<T> {
    /// Transaction validator
    validator: TransactionValidator,
    /// Stored data from unsigned transactions
    data_store: HashMap<u64, T>,
    /// Transaction history
    transaction_history: Vec<(u64, u64)>, // (block_number, nonce)
}

impl<T: std::fmt::Debug + Clone> UnsignedPallet<T> {
    pub fn new(validator: TransactionValidator) -> Self {
        Self {
            validator,
            data_store: HashMap::new(),
            transaction_history: Vec::new(),
        }
    }
    
    /// Submit an unsigned transaction
    pub fn submit_unsigned(
        &mut self,
        data: T,
        nonce: u64,
    ) -> Result<(), ValidationError> {
        let current_block = self.validator.block_simulator().current_block();
        let transaction = UnsignedTransaction::new(data.clone(), current_block, nonce);
        
        // Validate and accept transaction
        self.validator.accept_transaction(&transaction)?;
        
        // Store data
        self.data_store.insert(nonce, data);
        self.transaction_history.push((current_block, nonce));
        
        Ok(())
    }
    
    /// Get data by nonce
    pub fn get_data(&self, nonce: u64) -> Option<&T> {
        self.data_store.get(&nonce)
    }
    
    /// Get all stored data
    pub fn get_all_data(&self) -> Vec<(u64, &T)> {
        self.data_store.iter().map(|(nonce, data)| (*nonce, data)).collect()
    }
    
    /// Get transaction history
    pub fn get_transaction_history(&self) -> &[(u64, u64)] {
        &self.transaction_history
    }
    
    /// Get validator statistics
    pub fn get_validator_stats(&self) -> (u64, u32, u32) {
        self.validator.get_interval_stats()
    }
    
    /// Get next allowed submission info
    pub fn next_submission_info(&self) -> (Option<u64>, u64) {
        let next_block = self.validator.next_allowed_block();
        let blocks_until = self.validator.blocks_until_allowed();
        (next_block, blocks_until)
    }
    
    /// Advance blockchain state
    pub fn advance_block(&mut self) {
        self.validator.block_simulator_mut().next_block();
        self.validator.cleanup_old_intervals();
    }
    
    /// Advance multiple blocks
    pub fn advance_blocks(&mut self, count: u64) {
        self.validator.block_simulator_mut().advance_blocks(count);
        self.validator.cleanup_old_intervals();
    }
    
    /// Get current block number
    pub fn current_block(&self) -> u64 {
        self.validator.block_simulator().current_block()
    }
}
```

#### **Transaction Factory:**
```rust
/// Helper for creating transactions with proper nonces
pub struct TransactionFactory {
    next_nonce: u64,
}

impl TransactionFactory {
    pub fn new() -> Self {
        Self { next_nonce: 1 }
    }
    
    /// Create a new transaction with auto-incrementing nonce
    pub fn create_transaction<T>(
        &mut self,
        data: T,
        block_number: u64,
    ) -> UnsignedTransaction<T> {
        let nonce = self.next_nonce;
        self.next_nonce = self.next_nonce.saturating_add(1);
        UnsignedTransaction::new(data, block_number, nonce)
    }
    
    /// Get next nonce that will be used
    pub fn peek_next_nonce(&self) -> u64 {
        self.next_nonce
    }
    
    /// Reset nonce counter
    pub fn reset(&mut self) {
        self.next_nonce = 1;
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Transaction Validation:**
   - Test valid transaction acceptance
   - Test spam prevention (too many transactions)
   - Test timing restrictions (too early submissions)
   - Test duplicate prevention

2. **Block Progression:**
   - Test rate limiting across block intervals
   - Test cleanup of old interval data
   - Test minimum block interval enforcement

3. **State Management:**
   - Test data storage and retrieval
   - Test transaction history tracking
   - Test validator state updates

4. **Edge Cases:**
   - Test boundary conditions for intervals
   - Test maximum capacity scenarios
   - Test cleanup after long periods

5. **Integration:**
   - Test full transaction lifecycle
   - Test multiple concurrent submissions
   - Test validator statistics accuracy

### Expected Output

A complete unsigned transaction system that:
- Validates unsigned transactions effectively
- Prevents spam through rate limiting
- Manages state properly
- Handles errors gracefully
- Demonstrates understanding of Substrate's unsigned transaction concepts

### Theoretical Context

**Unsigned Transactions in Substrate:**
- **Purpose:** Allow transactions without cryptographic signatures
- **Use Cases:** Oracle data, periodic maintenance, system operations
- **Validation:** Must implement custom validation logic to prevent spam
- **Security:** Relies on validation logic rather than cryptographic proof
- **Block Inclusion:** Subject to same block limits as signed transactions

**Validation Strategies:**
- **Rate Limiting:** Prevent too many transactions per time period
- **Timing Restrictions:** Enforce minimum intervals between submissions
- **Duplicate Prevention:** Use nonces or hashes to prevent replays
- **Data Validation:** Ensure transaction data meets requirements
- **Resource Limits:** Prevent excessive resource consumption

**Best Practices:**
- Always implement comprehensive validation
- Use multiple validation layers for security
- Consider economic incentives and penalties
- Monitor and adjust limits based on network conditions
- Provide clear error messages for rejected transactions

This system demonstrates how to safely implement unsigned transactions while preventing network abuse. 
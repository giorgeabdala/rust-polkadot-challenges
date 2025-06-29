## Challenge 10: Simplified Transaction Pool

**Difficulty Level:** Advanced
**Estimated Time:** 1 hour

### Objective Description

In this challenge, you will implement a basic Transaction Pool system that demonstrates how transactions are stored, prioritized, and selected for inclusion in blocks. The focus is on understanding basic prioritization mechanisms and nonce-based dependencies.

### Main Concepts Covered

1. **Transaction Pool**: Pool of pending transactions awaiting inclusion in blocks
2. **Prioritization**: Simple priority system based on fees
3. **Nonce Dependencies**: Sequential transaction ordering per account
4. **Block Building**: Transaction selection for block construction

### Structures to Implement

#### **`TransactionHash`:**
```rust
pub type TransactionHash = [u8; 32];

// Helper to generate simple hash
pub fn simple_hash(data: &[u8]) -> TransactionHash {
    let mut hash = [0u8; 32];
    for (i, byte) in data.iter().enumerate() {
        if i >= 32 { break; }
        hash[i] = *byte;
    }
    hash
}
```

#### **`Transaction` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub hash: TransactionHash,
    pub sender: String,
    pub nonce: u64,
    pub priority: u64,  // Fee-based priority
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn new(
        sender: String,
        nonce: u64,
        priority: u64,
        data: Vec<u8>,
    ) -> Self {
        let hash_input = format!("{}:{}:{}", sender, nonce, priority);
        let hash = simple_hash(hash_input.as_bytes());
        
        Self {
            hash,
            sender,
            nonce,
            priority,
            data,
        }
    }
}
```

#### **`PoolStatus` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum PoolStatus {
    Pending, // Waiting for previous nonce
    Ready,   // Ready for inclusion
}
```

#### **`PoolTransaction` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct PoolTransaction {
    pub transaction: Transaction,
    pub status: PoolStatus,
}

impl PoolTransaction {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            status: PoolStatus::Pending, // Initially pending
        }
    }
    
    pub fn can_be_included(&self) -> bool {
        matches!(self.status, PoolStatus::Ready)
    }
}
```

#### **`TransactionPool` Struct:**
```rust
use std::collections::HashMap;

pub struct TransactionPool {
    transactions: HashMap<TransactionHash, PoolTransaction>,
    // Tracks the highest included nonce per sender
    sender_nonces: HashMap<String, u64>,
    max_pool_size: usize,
}
```

### Provided Methods of `TransactionPool`

#### **Constructor and Utilities:**
```rust
impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            sender_nonces: HashMap::new(),
            max_pool_size,
        }
    }
    
    pub fn get_transaction(&self, hash: &TransactionHash) -> Option<&PoolTransaction> {
        self.transactions.get(hash)
    }
    
    pub fn get_ready_count(&self) -> usize {
        self.transactions
            .values()
            .filter(|pool_tx| pool_tx.can_be_included())
            .count()
    }
    
    pub fn get_total_count(&self) -> usize {
        self.transactions.len()
    }
    
    pub fn get_sender_next_expected_nonce(&self, sender: &str) -> u64 {
        self.sender_nonces.get(sender).copied().unwrap_or(0) + 1
    }
}
```

### Methods for You to Implement

#### **1. Status Update (`update_ready_status`):**
```rust
impl TransactionPool {
    // TODO: Implement this method
    // Should update transaction status based on nonce dependencies
    // A transaction is "Ready" if its nonce is the next expected for the sender
    fn update_ready_status(&mut self) {
        // IMPLEMENT:
        // 1. For each transaction in the pool
        // 2. Check if the nonce is the next expected for the sender
        // 3. Update status to Ready or Pending as appropriate
        todo!()
    }
}
```

#### **2. Transaction Submission (`submit_transaction`):**
```rust
impl TransactionPool {
    // TODO: Implement this method
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        // IMPLEMENT:
        // 1. Check if the pool is not full
        // 2. Check if the transaction doesn't already exist (duplicate hash)
        // 3. Validate that nonce > 0
        // 4. Insert the transaction into the pool
        // 5. Update status of all transactions
        todo!()
    }
}
```

#### **3. Block Building (`build_block`):**
```rust
impl TransactionPool {
    // TODO: Implement this method
    pub fn build_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        // IMPLEMENT:
        // 1. Filter only "Ready" transactions
        // 2. Sort by priority (highest first)
        // 3. Select up to max_transactions
        // 4. Mark selected transactions as included
        // 5. Update sender nonces
        // 6. Update status of remaining transactions
        // 7. Return list of block transactions
        todo!()
    }
}
```

### Tests to Implement

Create tests that cover:

#### **Test Scenarios:**

1. **Basic Operations:**
   - Valid transaction submission
   - Rejection of transaction with invalid nonce (0)
   - Rejection of duplicate transaction
   - Pool limit verification

2. **Nonce Dependencies:**
   - Transaction with nonce 1 should become Ready immediately
   - Transaction with nonce 2 should stay Pending until nonce 1 is included
   - Multiple senders with independent sequences

3. **Block Building:**
   - Priority-based ordering
   - Transaction limit per block
   - Correct nonce update after inclusion

### Example Usage

```rust
fn main() {
    let mut pool = TransactionPool::new(10);
    
    // Submit transactions for "alice"
    let tx1 = Transaction::new("alice".to_string(), 1, 100, vec![1, 2, 3]);
    let tx2 = Transaction::new("alice".to_string(), 2, 200, vec![4, 5, 6]);
    
    pool.submit_transaction(tx1.clone()).unwrap();
    pool.submit_transaction(tx2.clone()).unwrap();
    
    println!("Ready transactions: {}", pool.get_ready_count()); // 1 (only tx1)
    
    // Build block
    let block = pool.build_block(10);
    println!("Transactions in block: {}", block.len()); // 1
    
    println!("Ready transactions after block: {}", pool.get_ready_count()); // 1 (tx2 is now ready)
}
```

### Expected Output

A transaction pool system that:
- Manages basic nonce dependencies
- Implements priority-based ordering
- Builds blocks by selecting valid transactions
- Demonstrates fundamental transaction pool concepts

### Theoretical Context

**Transaction Pool Fundamentals:**
- **Purpose**: Buffer pending transactions before block inclusion
- **Nonce Ordering**: Ensures transactions from same sender execute in order
- **Priority**: Higher fee transactions get preference
- **Lifecycle**: Submitted → Pending → Ready → Included

**Simplifications in This Challenge:**
- **Two States**: Only Pending/Ready
- **Simple Prioritization**: Based only on numeric priority
- **No Expiration**: Transactions don't expire automatically
- **Sequential Nonce**: Nonces must be strictly sequential

This challenge teaches essential transaction pool concepts while maintaining focus on the fundamental mechanisms of transaction ordering and selection.

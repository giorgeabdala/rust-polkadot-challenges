## Challenge 10: Transaction Pool and Prioritization

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

In this challenge, you will implement a simplified Transaction Pool system that demonstrates how transactions are stored, prioritized, and selected for inclusion in blocks. The focus is on understanding basic prioritization mechanisms, nonce-based dependencies, and transaction lifecycle management.

### Main Concepts Covered

1. **Transaction Pool**: Pool of pending transactions awaiting inclusion in blocks
2. **Prioritization**: Simple priority system based on fees
3. **Nonce Dependencies**: Sequential transaction ordering per account
4. **Longevity**: Transaction lifetime in the pool
5. **Block Building**: Transaction selection for block construction

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
    pub sender: String, // Simplified AccountId
    pub nonce: u64,
    pub priority: u64,  // Fee-based priority
    pub longevity: u64, // Blocks the transaction remains valid
    pub data: Vec<u8>,  // Transaction data
}

impl Transaction {
    pub fn new(
        sender: String,
        nonce: u64,
        priority: u64,
        longevity: u64,
        data: Vec<u8>,
    ) -> Self {
        let hash_input = format!("{}:{}:{}", sender, nonce, priority);
        let hash = simple_hash(hash_input.as_bytes());
        
        Self {
            hash,
            sender,
            nonce,
            priority,
            longevity,
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
    pub inserted_at: u64, // Block when it was inserted
}

impl PoolTransaction {
    pub fn new(transaction: Transaction, current_block: u64) -> Self {
        Self {
            transaction,
            status: PoolStatus::Pending, // Initially pending
            inserted_at: current_block,
        }
    }
    
    pub fn is_expired(&self, current_block: u64) -> bool {
        current_block > self.inserted_at + self.transaction.longevity
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
    // Track highest included nonce per sender for dependency resolution
    sender_nonces: HashMap<String, u64>,
    current_block: u64,
    max_pool_size: usize,
}
```

### Required Methods of `TransactionPool`

#### **Constructor and Utilities:**
```rust
impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            sender_nonces: HashMap::new(),
            current_block: 0,
            max_pool_size,
        }
    }
    
    pub fn set_current_block(&mut self, block_number: u64) {
        self.current_block = block_number;
        self.cleanup_expired();
        self.update_ready_status();
    }
    
    fn cleanup_expired(&mut self) {
        let current_block = self.current_block;
        let expired_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| pool_tx.is_expired(current_block))
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in expired_hashes {
            self.transactions.remove(&hash);
        }
    }
    
    // Update transaction status based on nonce dependencies
    fn update_ready_status(&mut self) {
        let mut updates = Vec::new();
        
        for (hash, pool_tx) in &self.transactions {
            let sender = &pool_tx.transaction.sender;
            let nonce = pool_tx.transaction.nonce;
            
            // Get the highest included nonce for this sender
            let last_included_nonce = self.sender_nonces.get(sender).copied().unwrap_or(0);
            
            // Transaction is ready if its nonce is the next expected one
            let should_be_ready = if last_included_nonce == 0 {
                nonce == 1 // First transaction should have nonce 1
            } else {
                nonce == last_included_nonce + 1
            };
            
            let new_status = if should_be_ready {
                PoolStatus::Ready
            } else {
                PoolStatus::Pending
            };
            
            if pool_tx.status != new_status {
                updates.push((*hash, new_status));
            }
        }
        
        // Apply updates
        for (hash, new_status) in updates {
            if let Some(pool_tx) = self.transactions.get_mut(&hash) {
                pool_tx.status = new_status;
            }
        }
    }
}
```

#### **Transaction Management:**
```rust
impl TransactionPool {
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        // Check pool size limit
        if self.transactions.len() >= self.max_pool_size {
            return Err("Pool is full");
        }
        
        // Check if transaction already exists
        if self.transactions.contains_key(&transaction.hash) {
            return Err("Transaction already in pool");
        }
        
        // Validate nonce (must be > 0)
        if transaction.nonce == 0 {
            return Err("Invalid nonce: must be greater than 0");
        }
        
        let pool_transaction = PoolTransaction::new(transaction, self.current_block);
        let hash = pool_transaction.transaction.hash;
        
        // Insert transaction
        self.transactions.insert(hash, pool_transaction);
        
        // Update ready status for all transactions
        self.update_ready_status();
        
        Ok(())
    }
    
    pub fn remove_transaction(&mut self, hash: &TransactionHash) -> Option<PoolTransaction> {
        let removed = self.transactions.remove(hash);
        
        if removed.is_some() {
            self.update_ready_status();
        }
        
        removed
    }
    
    // Mark transaction as included (updates sender nonce tracking)
    pub fn mark_as_included(&mut self, hash: &TransactionHash) -> Option<PoolTransaction> {
        if let Some(pool_tx) = self.transactions.remove(hash) {
            let sender = &pool_tx.transaction.sender;
            let nonce = pool_tx.transaction.nonce;
            
            // Update sender's highest included nonce
            let current_nonce = self.sender_nonces.get(sender).copied().unwrap_or(0);
            if nonce > current_nonce {
                self.sender_nonces.insert(sender.clone(), nonce);
            }
            
            // Update ready status for remaining transactions
            self.update_ready_status();
            
            Some(pool_tx)
        } else {
            None
        }
    }
}
```

#### **Block Building:**
```rust
impl TransactionPool {
    pub fn build_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        // Get all ready transactions
        let mut ready_transactions: Vec<(TransactionHash, &PoolTransaction)> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| pool_tx.can_be_included())
            .collect();
        
        // Sort by priority (higher first), then by insertion time (older first)
        ready_transactions.sort_by(|(_, a), (_, b)| {
            match b.transaction.priority.cmp(&a.transaction.priority) {
                std::cmp::Ordering::Equal => a.inserted_at.cmp(&b.inserted_at),
                other => other,
            }
        });
        
        // Select up to max_transactions
        let selected_hashes: Vec<TransactionHash> = ready_transactions
            .into_iter()
            .take(max_transactions)
            .map(|(hash, _)| *hash)
            .collect();
        
        // Build block and mark transactions as included
        let mut block_transactions = Vec::new();
        for hash in selected_hashes {
            if let Some(pool_tx) = self.mark_as_included(&hash) {
                block_transactions.push(pool_tx.transaction);
            }
        }
        
        block_transactions
    }
    
    pub fn get_ready_transactions(&self) -> Vec<&Transaction> {
        self.transactions
            .values()
            .filter(|pool_tx| pool_tx.can_be_included())
            .map(|pool_tx| &pool_tx.transaction)
            .collect()
    }
    
    pub fn get_ready_count(&self) -> usize {
        self.transactions
            .values()
            .filter(|pool_tx| pool_tx.can_be_included())
            .count()
    }
    
    pub fn get_pending_count(&self) -> usize {
        self.transactions
            .values()
            .filter(|pool_tx| matches!(pool_tx.status, PoolStatus::Pending))
            .count()
    }
    
    pub fn get_total_count(&self) -> usize {
        self.transactions.len()
    }
}
```

#### **Query Methods:**
```rust
impl TransactionPool {
    pub fn get_transaction(&self, hash: &TransactionHash) -> Option<&PoolTransaction> {
        self.transactions.get(hash)
    }
    
    pub fn get_transactions_by_sender(&self, sender: &str) -> Vec<&PoolTransaction> {
        self.transactions
            .values()
            .filter(|pool_tx| pool_tx.transaction.sender == sender)
            .collect()
    }
    
    pub fn get_sender_next_nonce(&self, sender: &str) -> u64 {
        self.sender_nonces.get(sender).copied().unwrap_or(0) + 1
    }
    
    pub fn get_pool_stats(&self) -> (usize, usize, usize) {
        let total = self.get_total_count();
        let ready = self.get_ready_count();
        let pending = self.get_pending_count();
        (total, ready, pending)
    }
}
```

### Tests

Create comprehensive tests covering:

#### **Test Scenarios:**

1. **Basic Operations:**
   - Test transaction submission
   - Test pool size limits
   - Test duplicate transaction rejection
   - Test invalid nonce rejection

2. **Nonce Dependencies:**
   - Test sequential nonce ordering
   - Test transaction promotion to Ready when dependencies are met
   - Test multiple senders with independent nonce sequences

3. **Prioritization:**
   - Test priority-based ordering in block building
   - Test insertion time as tiebreaker for same priority
   - Test block construction with priority selection

4. **Expiration and Cleanup:**
   - Test transaction expiration based on longevity
   - Test automatic cleanup on block advancement
   - Test status updates after cleanup

5. **Block Building:**
   - Test block construction with transaction limit
   - Test transaction removal after inclusion
   - Test nonce tracking after inclusion

6. **Integration Tests:**
   - Test complete transaction lifecycle
   - Test multiple accounts with mixed priorities
   - Test pool behavior over multiple blocks

### Example Usage

```rust
fn main() {
    let mut pool = TransactionPool::new(100);
    
    // Submit transactions for account "alice"
    let tx1 = Transaction::new("alice".to_string(), 1, 100, 10, vec![1, 2, 3]);
    let tx2 = Transaction::new("alice".to_string(), 2, 200, 10, vec![4, 5, 6]);
    
    pool.submit_transaction(tx1.clone()).unwrap();
    pool.submit_transaction(tx2.clone()).unwrap();
    
    println!("Pool stats: {:?}", pool.get_pool_stats()); // (2, 1, 1)
    
    // Build a block
    let block = pool.build_block(10);
    println!("Block transactions: {}", block.len()); // 1 (only tx1 was ready)
    
    println!("Pool stats after block: {:?}", pool.get_pool_stats()); // (1, 1, 0)
}
```

### Expected Output

A complete transaction pool system that:
- Manages transaction lifecycle with simple nonce-based dependencies
- Implements priority-based transaction ordering
- Handles expiration and cleanup automatically
- Supports efficient block building
- Demonstrates core transaction pool concepts without unnecessary complexity

### Theoretical Context

**Transaction Pool Fundamentals:**
- **Purpose**: Buffer pending transactions before block inclusion
- **Nonce Ordering**: Ensures transactions from same sender execute in order
- **Priority**: Higher fee transactions get preference
- **Lifecycle**: Submitted → Pending → Ready → Included/Expired

**Key Simplifications:**
- **Nonce Dependencies**: Direct nonce sequencing instead of abstract tag system
- **Simple Prioritization**: Priority + insertion time instead of complex heap
- **Two States**: Pending/Ready instead of multiple complex states
- **Direct Block Building**: Straightforward selection algorithm

This challenge teaches essential transaction pool concepts while maintaining focus on the core mechanisms that drive transaction ordering and block construction in blockchain systems.

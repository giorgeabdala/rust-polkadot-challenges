## Challenge 10: Transaction Pool and Prioritization

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

In this challenge, you will implement a simplified Transaction Pool system that simulates how transactions are stored, prioritized, and selected for inclusion in blocks. The focus is on understanding prioritization mechanisms, transaction dependencies, and removal policies.

### Main Concepts Covered

1. **Transaction Pool**: Pool of pending transactions awaiting inclusion in blocks
2. **Prioritization**: Priority system based on fees and importance
3. **Dependencies**: Transactions that depend on others (sequential nonce)
4. **Longevity**: Transaction lifetime in the pool
5. **Block Building**: Transaction selection to form a block

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
    pub priority: u64,
    pub longevity: u64, // Blocks the transaction remains valid
    pub requires: Vec<TransactionHash>, // Dependencies
    pub provides: Vec<TransactionHash>, // What this transaction provides
    pub data: Vec<u8>, // Transaction data
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
        
        // Generate provides based on sender and nonce
        let provides_input = format!("{}:{}", sender, nonce);
        let provides = vec![simple_hash(provides_input.as_bytes())];
        
        // Generate requires based on previous nonce (if > 0)
        let requires = if nonce > 0 {
            let requires_input = format!("{}:{}", sender, nonce - 1);
            vec![simple_hash(requires_input.as_bytes())]
        } else {
            Vec::new()
        };
        
        Self {
            hash,
            sender,
            nonce,
            priority,
            longevity,
            requires,
            provides,
            data,
        }
    }
}
```

#### **`PoolStatus` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum PoolStatus {
    Ready,    // Ready for inclusion
    Future,   // Waiting for dependencies
    Invalid,  // Invalid (will be removed)
}
```

#### **`PoolTransaction` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct PoolTransaction {
    pub transaction: Transaction,
    pub status: PoolStatus,
    pub inserted_at: u64, // Block when it was inserted
    pub retries: u32,     // Inclusion attempts
}

impl PoolTransaction {
    pub fn new(transaction: Transaction, current_block: u64) -> Self {
        Self {
            transaction,
            status: PoolStatus::Future, // Initially Future, will be promoted if possible
            inserted_at: current_block,
            retries: 0,
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
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

pub struct TransactionPool {
    transactions: HashMap<TransactionHash, PoolTransaction>,
    ready_queue: BinaryHeap<ReadyTransaction>, // Heap for prioritization
    provided_tags: HashSet<TransactionHash>,   // Tags provided by ready transactions
    current_block: u64,
    max_pool_size: usize,
}

// Wrapper for use in BinaryHeap (max-heap by priority)
#[derive(Clone, Debug, PartialEq, Eq)]
struct ReadyTransaction {
    hash: TransactionHash,
    priority: u64,
    inserted_at: u64,
}

impl Ord for ReadyTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        // First by priority (higher is better)
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // In case of tie, older is better (FIFO)
                other.inserted_at.cmp(&self.inserted_at)
            }
            other => other,
        }
    }
}

impl PartialOrd for ReadyTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

### Required Methods of `TransactionPool`

#### **Constructor and Utilities:**
```rust
impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            ready_queue: BinaryHeap::new(),
            provided_tags: HashSet::new(),
            current_block: 0,
            max_pool_size,
        }
    }
    
    pub fn set_current_block(&mut self, block_number: u64) {
        self.current_block = block_number;
        self.cleanup_expired();
    }
    
    fn cleanup_expired(&mut self) {
        let current_block = self.current_block;
        let expired_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| pool_tx.is_expired(current_block))
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in expired_hashes {
            self.remove_transaction(&hash);
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
        
        let pool_transaction = PoolTransaction::new(transaction, self.current_block);
        let hash = pool_transaction.transaction.hash;
        
        // Insert transaction
        self.transactions.insert(hash, pool_transaction);
        
        // Try to promote to ready
        self.update_transaction_status(&hash);
        
        Ok(())
    }
    
    fn update_transaction_status(&mut self, hash: &TransactionHash) {
        if let Some(pool_tx) = self.transactions.get_mut(hash) {
            let can_be_ready = pool_tx.transaction.requires.iter()
                .all(|required| self.provided_tags.contains(required));
            
            if can_be_ready && matches!(pool_tx.status, PoolStatus::Future) {
                // Promote to ready
                pool_tx.status = PoolStatus::Ready;
                
                // Add to ready queue
                let ready_tx = ReadyTransaction {
                    hash: *hash,
                    priority: pool_tx.transaction.priority,
                    inserted_at: pool_tx.inserted_at,
                };
                self.ready_queue.push(ready_tx);
                
                // Add provided tags
                for provided in &pool_tx.transaction.provides {
                    self.provided_tags.insert(*provided);
                }
                
                // Check if other transactions can now be promoted
                self.promote_future_transactions();
            }
        }
    }
    
    fn promote_future_transactions(&mut self) {
        let future_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| matches!(pool_tx.status, PoolStatus::Future))
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in future_hashes {
            self.update_transaction_status(&hash);
        }
    }
    
    fn remove_transaction(&mut self, hash: &TransactionHash) {
        if let Some(pool_tx) = self.transactions.remove(hash) {
            // Remove from provided tags
            for provided in &pool_tx.transaction.provides {
                self.provided_tags.remove(provided);
            }
            
            // Remove from ready queue (will be filtered out when popped)
            // Note: BinaryHeap doesn't support efficient removal, so we'll filter when popping
        }
    }
}
```

#### **Block Building:**
```rust
impl TransactionPool {
    pub fn build_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        let mut block_transactions = Vec::new();
        let mut temp_ready_queue = BinaryHeap::new();
        
        // Extract transactions from ready queue
        while let Some(ready_tx) = self.ready_queue.pop() {
            // Check if transaction still exists and is ready
            if let Some(pool_tx) = self.transactions.get(&ready_tx.hash) {
                if pool_tx.can_be_included() && block_transactions.len() < max_transactions {
                    block_transactions.push(pool_tx.transaction.clone());
                    
                    // Remove from pool
                    self.remove_transaction(&ready_tx.hash);
                } else if pool_tx.can_be_included() {
                    // Put back in queue if we've reached max transactions
                    temp_ready_queue.push(ready_tx);
                }
                // If not ready anymore, just discard
            }
        }
        
        // Restore remaining ready transactions to queue
        self.ready_queue = temp_ready_queue;
        
        block_transactions
    }
    
    pub fn get_ready_count(&self) -> usize {
        self.ready_queue.len()
    }
    
    pub fn get_future_count(&self) -> usize {
        self.transactions.values()
            .filter(|pool_tx| matches!(pool_tx.status, PoolStatus::Future))
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
        self.transactions.values()
            .filter(|pool_tx| pool_tx.transaction.sender == sender)
            .collect()
    }
    
    pub fn get_ready_transactions(&self) -> Vec<&Transaction> {
        self.ready_queue.iter()
            .filter_map(|ready_tx| {
                self.transactions.get(&ready_tx.hash)
                    .map(|pool_tx| &pool_tx.transaction)
            })
            .collect()
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

2. **Dependency Management:**
   - Test sequential nonce dependencies
   - Test transaction promotion from Future to Ready
   - Test dependency chain resolution

3. **Prioritization:**
   - Test priority-based ordering in ready queue
   - Test FIFO ordering for same priority
   - Test block building with priority selection

4. **Expiration and Cleanup:**
   - Test transaction expiration based on longevity
   - Test automatic cleanup on block advancement
   - Test removal of expired transactions

5. **Block Building:**
   - Test block construction with max transaction limit
   - Test transaction removal after inclusion
   - Test ready queue management during block building

6. **Edge Cases:**
   - Test empty pool operations
   - Test full pool behavior
   - Test complex dependency chains
   - Test mixed priority scenarios

### Expected Output

A complete transaction pool system that:
- Manages transaction lifecycle from submission to inclusion
- Implements proper dependency resolution
- Provides priority-based transaction ordering
- Handles expiration and cleanup automatically
- Supports efficient block building
- Demonstrates understanding of transaction pool mechanics

### Theoretical Context

**Transaction Pool in Substrate:**
- **Purpose**: Store and manage pending transactions before block inclusion
- **Prioritization**: Based on fees, importance, and dependencies
- **Dependencies**: Transactions can depend on others (e.g., nonce ordering)
- **Lifecycle**: Submitted → Validated → Ready → Included/Expired

**Key Concepts:**
- **Ready Queue**: Transactions ready for immediate inclusion
- **Future Queue**: Transactions waiting for dependencies
- **Provides/Requires**: Tags system for dependency management
- **Longevity**: How long transactions remain valid

**Block Production:**
- Block producers select transactions from the ready queue
- Selection considers priority, dependencies, and block limits
- Included transactions are removed from the pool

This challenge demonstrates the critical infrastructure that manages transaction flow in blockchain systems, ensuring proper ordering and efficient block construction.

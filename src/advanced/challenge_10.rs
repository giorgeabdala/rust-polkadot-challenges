pub type TransactionHash = [u8; 32];

pub fn simple_hash(data: &[u8]) -> TransactionHash {
    let mut hash = [0u8; 32];
    for (i, byte) in data.iter().enumerate() {
        if i >= 32 { break; }
        hash[i] = *byte;
    }
    hash
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub hash: TransactionHash,
    pub sender: String,
    pub nonce: u64,
    pub priority: u64,  
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNonce,
    TransactionPoolFull,
    TransactionDuplicate
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PoolStatus {
    Pending,
    Ready
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolTransaction {
    pub transaction: Transaction,
    pub status: PoolStatus,
}

impl PoolTransaction {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            status: PoolStatus::Pending, 
        }
    }

    pub fn can_be_included(&self) -> bool {
        matches!(self.status, PoolStatus::Ready)
    }
}

use std::collections::HashMap;

pub struct TransactionPool {
    transactions: HashMap<TransactionHash, PoolTransaction>,
    sender_nonces: HashMap<String, u64>,
    max_pool_size: usize,
}


impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            sender_nonces: HashMap::new(),
            max_pool_size
        }
    }
    
    fn update_ready_status(&mut self) {
        for (_, pool_tx) in self.transactions.iter_mut() {
            let next_nonce = self.sender_nonces.get(&pool_tx.transaction.sender).copied().unwrap_or(0) + 1;
            if pool_tx.transaction.nonce == next_nonce {
                pool_tx.status = PoolStatus::Ready;
            }
        }
    }
    
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), Error> {
        if self.transactions.len() >= self.max_pool_size {return Err(Error::TransactionPoolFull)}
        if self.transactions.contains_key(&transaction.hash) {return Err(Error::TransactionDuplicate)}
        if transaction.nonce <= 0 {return Err(Error::InvalidNonce)}
        
        let hash = transaction.hash;
        let pool_tx = PoolTransaction::new(transaction);
        self.transactions.insert(hash, pool_tx);
        self.update_ready_status();
        Ok(())
    }

    pub fn build_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        let mut ready_hashes: Vec<(TransactionHash, u64)> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| pool_tx.status == PoolStatus::Ready)
            .map(|(hash, pool_tx)| (*hash, pool_tx.transaction.priority))
            .collect();
        
        ready_hashes.sort_by_key(|(_, priority)| std::cmp::Reverse(*priority));
        
        let mut selected_transactions = Vec::new();
        for (hash, _) in ready_hashes.iter().take(max_transactions) {
            if let Some(pool_tx) = self.transactions.get(hash) {
                selected_transactions.push(pool_tx.transaction.clone());
            }
        }
        for transaction in &selected_transactions {
            // Remover do pool
            self.transactions.remove(&transaction.hash);
            
            // Atualizar o nonce do sender
            let current_nonce = self.sender_nonces.get(&transaction.sender).copied().unwrap_or(0);
            self.sender_nonces.insert(transaction.sender.clone(), current_nonce.max(transaction.nonce));
        }
        self.update_ready_status();

        selected_transactions
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

#[cfg(test)]

mod tests {
    use crate::advanced::challenge_10::{Error, PoolStatus, Transaction, TransactionPool};

    #[test]
    fn submit_transaction_test() {
        let mut pool = TransactionPool::new(10);
        let tx1 = Transaction::new("alice".to_string(), 1, 100, vec![1,2,3]);
        let tx2 = Transaction::new("alice".to_string(), 2, 100, vec![1,2,3]);
        let result = pool.submit_transaction(tx1.clone());
        assert!(result.is_ok());
        let result = pool.submit_transaction(tx2.clone());
        assert!(result.is_ok());
        assert_eq!(pool.get_ready_count(), 1);
    }
    
    #[test]
    fn submit_transaction_invalid_nonce_fail() {
        let mut pool = TransactionPool::new(10);
        let tx1 = Transaction::new("alice".to_string(), 0, 100, vec![1,2,3]);
        let result = pool.submit_transaction(tx1.clone());
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidNonce));
    }
    
    #[test]
    fn submit_transaction_transaction_duplicate_fail() {
        let mut pool = TransactionPool::new(10);
        let tx1 = Transaction::new("alice".to_string(), 1, 100, vec![1,2,3]);
        let result = pool.submit_transaction(tx1.clone());
        assert!(result.is_ok());
        let result = pool.submit_transaction(tx1.clone());
        assert!(result.is_err());
        assert_eq!(result, Err(Error::TransactionDuplicate));
    }
    
    #[test]
    fn submit_transaction_transaction_pool_full_fail() {
        let mut pool = TransactionPool::new(1);
        let tx1 = Transaction::new("alice".to_string(), 1, 100, vec![1,2,3]);
        let result = pool.submit_transaction(tx1.clone());
        assert!(result.is_ok());
        let tx2 = Transaction::new("alice".to_string(), 2, 100, vec![1,2,3]);
        let result = pool.submit_transaction(tx2.clone());
        assert_eq!(result, Err(Error::TransactionPoolFull));
    }
    
    #[test]
    fn nonce_dependence_test() {
        let mut pool = TransactionPool::new(10);
        let tx1 = Transaction::new("alice".to_string(), 1, 100, vec![1,2,3]);
        let tx2 = Transaction::new("alice".to_string(), 2, 100, vec![1,2,3]);

        let tx1_bob = Transaction::new("bob".to_string(), 1, 100, vec![1,2,3]);
        let tx2_bob = Transaction::new("bob".to_string(), 2, 100, vec![1,2,3]);

        let _ = pool.submit_transaction(tx1.clone());
        let _ = pool.submit_transaction(tx2.clone());
        let _ = pool.submit_transaction(tx1_bob.clone());
        let _ = pool.submit_transaction(tx2_bob.clone());

        {
            let tx1_pool_result = pool.get_transaction(&tx1.hash);
            let tx2_pool_result = pool.get_transaction(&tx2.hash);
            let tx1_bob_pool_result = pool.get_transaction(&tx1_bob.hash);
            let tx2_bob_pool_result = pool.get_transaction(&tx2_bob.hash);
            assert!(tx1_pool_result.is_some());
            assert!(tx2_pool_result.is_some());
            assert!(tx1_bob_pool_result.is_some());
            assert!(tx2_bob_pool_result.is_some());
            let tx_pool = tx1_pool_result.unwrap();
            let tx2_pool = tx2_pool_result.unwrap();
            let tx_bob_pool = tx1_pool_result.unwrap();
            let tx2_bob_pool = tx2_pool_result.unwrap();
            assert_eq!(tx_pool.status, PoolStatus::Ready);
            assert_eq!(tx2_pool.status.clone(), PoolStatus::Pending);
            assert_eq!(tx_bob_pool.status, PoolStatus::Ready);
            assert_eq!(tx2_bob_pool.status.clone(), PoolStatus::Pending);
        }

        let block = pool.build_block(1);
        let tx2_pool_after_opt = pool.get_transaction(&tx2.hash);
        let tx2_bob_pool_after_opt = pool.get_transaction(&tx2_bob.hash);
        assert!(tx2_pool_after_opt.is_some());
        assert!(tx2_bob_pool_after_opt.is_some());
        let tx2_pool_after = tx2_pool_after_opt.unwrap();
        let tx2_bob_pool_after = tx2_pool_after_opt.unwrap();
        assert_eq!(tx2_pool_after.status, PoolStatus::Ready);
        assert_eq!(tx2_bob_pool_after.status, PoolStatus::Ready);
    }
     #[test]
        fn test_build_block_selects_by_priority_and_updates_state() {
            // --- ARRANGE ---
            // Setup a pool with enough capacity.
            let mut pool = TransactionPool::new(10);

            // Create transactions with varying priorities. All have nonce 1 to be "Ready".
            let tx_high_priority = Transaction::new("alice".to_string(), 1, 200, vec![1]);
            let tx_medium_priority = Transaction::new("bob".to_string(), 1, 150, vec![2]);
            let tx_low_priority = Transaction::new("charlie".to_string(), 1, 100, vec![3]);

            // Create a pending transaction with a high priority. 
            // This should NOT be included in the block because its status is not "Ready".
            let tx_pending = Transaction::new("alice".to_string(), 2, 999, vec![4]);

            // Submit all transactions to the pool.
            pool.submit_transaction(tx_high_priority.clone()).unwrap();
            pool.submit_transaction(tx_medium_priority.clone()).unwrap();
            pool.submit_transaction(tx_low_priority.clone()).unwrap();
            pool.submit_transaction(tx_pending.clone()).unwrap();

            // Initial sanity check: 3 transactions should be ready for inclusion.
            assert_eq!(pool.get_ready_count(), 3, "There should be 3 ready transactions before building the block");

            // --- ACT ---
            // Build a block with a limit of 2 transactions.
            let block = pool.build_block(2);

            // --- ASSERT ---
            // 1. Test: Transaction limit per block.
            // The block should contain exactly 2 transactions.
            assert_eq!(block.len(), 2, "Block should be limited to 2 transactions");

            // 2. Test: Priority-based ordering.
            // The block should contain the transactions with the highest priority, in order.
            assert_eq!(block[0].hash, tx_high_priority.hash, "First transaction in block should be the one with highest priority");
            assert_eq!(block[1].hash, tx_medium_priority.hash, "Second transaction in block should be the one with medium priority");

            // 3. Test: Correct nonce update after inclusion.
            // The nonces for the senders of the included transactions must be updated.
            assert_eq!(pool.get_sender_next_expected_nonce("alice"), 2, "Next expected nonce for 'alice' should be 2");
            assert_eq!(pool.get_sender_next_expected_nonce("bob"), 2, "Next expected nonce for 'bob' should be 2");

            // The nonce for 'charlie' should NOT be updated, as its transaction was not included.
            assert_eq!(pool.get_sender_next_expected_nonce("charlie"), 1, "Next expected nonce for 'charlie' should remain 1");

            // Final state check: The pool should now contain the two remaining transactions.
            assert_eq!(pool.get_total_count(), 2, "Pool should have 2 transactions remaining");
            // After including 'alice's' nonce 1, 'tx_pending' (nonce 2) should now be ready.
            assert_eq!(pool.get_ready_count(), 2, "The 2 remaining transactions should now be ready");
        }
    }


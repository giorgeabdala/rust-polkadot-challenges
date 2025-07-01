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

#[derive(Clone, Debug, PartialEq)]
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
    
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        if self.transactions.len() >= self.max_pool_size {return Err("Transaction Pool full")}
        if self.transactions.contains_key(&transaction.hash) {return Err("Transaction already exist")}
        if transaction.nonce <= 0 {return Err("Invalid Nonce")}
        
        let pool_tx = PoolTransaction::new(transaction.clone());
        self.transactions.insert(transaction.hash, pool_tx);
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
    use crate::advanced::challenge_10::{Transaction, TransactionPool};

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
}
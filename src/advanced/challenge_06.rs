use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(ValidationError)
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooManyTransactions,
    TooEarly,
    InvalidData(String),
    Duplicate,
}


impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TooManyTransactions => write!(f, "Too many transactions in current interval"),
            ValidationError::TooEarly => write!(f, "Transaction submitted too early"),
            ValidationError::InvalidData(msg) => write!(f, "invalid Data: {}", msg),
            ValidationError::Duplicate => write!(f, "Duplicate transaction")


        }
    }
}

pub struct BlockSimulator {
    current_block: u64,
    block_time: u64,
}

impl BlockSimulator {
    pub fn new(block_time: u64) -> Self {
        Self {
            current_block: 1,
            block_time
        }
    }

    pub fn current_block(&self) -> u64{
        self.current_block
    }

    pub fn next_block(&mut self) {
        self.current_block = self.current_block.saturating_add(1);
    }

    pub fn advance_blocks(&mut self, count: u64) {
        self.current_block = self.current_block.saturating_add(count);
    }

    pub fn blocks_since(&self, last_block: u64) -> u64 {
        self.current_block.saturating_sub(last_block)
    }

    pub fn block_time(&self) -> u64 {
        self.block_time
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedTransaction<T> {
    pub data: T,
    pub block_number: u64,
    pub nonce: u64
}

impl <T> UnsignedTransaction<T> {
    pub fn new(data: T, block_number: u64, nonce: u64) -> Self {
        Self {
            data,
            block_number,
            nonce
        }
    }

}

pub struct TransactionValidator {
    max_per_interval: u32,
    interval_blocks: u64,
    min_block_interval: u64,
    interval_counts: HashMap<u64, u32>,
    last_submission: Option<u64>,
    used_nonces: HashSet<u64>,
    block_simulator: BlockSimulator
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

    pub fn validate_transaction<T>
    (&self, transaction: &UnsignedTransaction<T>) -> ValidationResult
    where
        T: std::fmt::Debug
    {
        let current_block = self.block_simulator.current_block();

        if let Some(last_block) = self.last_submission {
            let blocks_since = self.block_simulator.blocks_since(last_block);
            if blocks_since < self.min_block_interval {
                return ValidationResult::Invalid(ValidationError::TooEarly);
            }
        }

        if self.used_nonces.contains(&transaction.nonce) {
            return ValidationResult::Invalid(ValidationError::Duplicate);
        }

        let interval_start = self.get_interval_start(current_block);
        let current_count = self.interval_counts.get(&interval_start).unwrap_or(&0);

        if *current_count >= self.max_per_interval {
            return ValidationResult::Invalid(ValidationError::TooManyTransactions);
        }

        ValidationResult::Valid
    }

    pub fn accept_transaction<T>(&mut self, transaction: &UnsignedTransaction<T>) -> Result<(), ValidationError>
    where
        T: std::fmt::Debug,

    {
        let current_block = self.block_simulator.current_block();
        match self.validate_transaction(transaction) {
            ValidationResult::Valid => {},
            ValidationResult::Invalid(error) => return Err(error)
        }

        let interval_start = self.get_interval_start(current_block);
        let count = self.interval_counts.entry(interval_start).or_insert(0);
        *count += 1;

        self.last_submission = Some(current_block);
        self.used_nonces.insert(transaction.nonce);
        Ok(())
    }

    fn get_interval_start(&self, block_number: u64) -> u64 {
        (block_number / self.interval_blocks) * self.interval_blocks
    }


    pub fn cleanup_old_intervals(&mut self) {
        let current_block = self.block_simulator.current_block();
        let current_interval_start = self.get_interval_start(current_block);
        let cutoff = current_interval_start.saturating_sub(self.interval_blocks);
        self.interval_counts.retain(|&interval_start, _| interval_start >= cutoff);
    }


    pub fn get_interval_stats(&self) -> (u64, u32, u32) {
        let current_block = self.block_simulator.current_block();
        let interval_start = self.get_interval_start(current_block);
        let current_count = self.interval_counts.get(&interval_start).unwrap_or(&0);
        (interval_start, *current_count, self.max_per_interval)
    }

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

    fn next_allowed_block(&self) -> Option<u64> {
        if let Some(last_block) = self.last_submission {
            Some(last_block + self.min_block_interval)
        } else {
            None
        }
    }

    pub fn block_simulator_mut(&mut self) -> &mut BlockSimulator {
        &mut self.block_simulator
    }

    pub fn block_simulator(&self) -> &BlockSimulator {
        &self.block_simulator
    }
}

pub struct UnsignedPallet<T> {
    validator: TransactionValidator,
    data_store: HashMap<u64, T>,
    transaction_history: Vec<(u64, u64)>,
}

impl <T: std::fmt::Debug + Clone> UnsignedPallet<T> {
    pub fn new(validator: TransactionValidator) -> Self {
        Self {
            validator,
            data_store: HashMap::new(),
            transaction_history: Vec::new(),
        }
    }

    pub fn submit_unsigned(&mut self, data: T, nonce: u64) -> Result<(), ValidationError> {
        let current_block = self.validator.block_simulator().current_block();
        let transaction = UnsignedTransaction::new(data.clone(), current_block, nonce);

        self.validator.accept_transaction(&transaction)?;

        self.data_store.insert(nonce, data);
        self.transaction_history.push((current_block, nonce));
        Ok(())
    }

    pub fn get_data(&self, nonce: u64) -> Option<&T> {
        self.data_store.get(&nonce)
    }

    /// Obtém todos os dados armazenados
    pub fn get_all_data(&self) -> Vec<(u64, &T)> {
        self.data_store.iter().map(|(nonce, data)| (*nonce, data)).collect()
    }

    /// Obtém o histórico de transações
    pub fn get_transaction_history(&self) -> &[(u64, u64)] {
        &self.transaction_history
    }

    /// Obtém as estatísticas do validador
    pub fn get_validator_stats(&self) -> (u64, u32, u32) {
        self.validator.get_interval_stats()
    }

    pub fn next_submission_info(&self) -> (Option<u64>, u64) {
        let next_block = self.validator.next_allowed_block();
        let blocks_until = self.validator.blocks_until_allowed();
        (next_block, blocks_until)
    }

    pub fn advance_block(&mut self) {
        self.validator.block_simulator_mut().next_block();
        self.validator.cleanup_old_intervals();
    }

    pub fn advance_blocks(&mut self, count: u64) {
        self.validator.block_simulator_mut().advance_blocks(count);
        self.validator.cleanup_old_intervals();
    }

    pub fn current_block(&self) -> u64 {
        self.validator.block_simulator().current_block()
    }
}

pub struct TransactionFactory {
    next_nonce: u64,
}

impl TransactionFactory {
    pub fn new() -> Self {
        Self { next_nonce: 1 }
    }

    pub fn create_transaction<T>(
        &mut self,
        data: T,
        block_number: u64) -> UnsignedTransaction<T> {
        let nonce = self.next_nonce;
        self.next_nonce = self.next_nonce.saturating_add(1);
        UnsignedTransaction::new(data, block_number, nonce)
    }

    pub fn peek_next_nonce(&self) -> u64 {
        self.next_nonce
    }

    pub fn reset(&mut self) {
        self.next_nonce = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: u32 = 2;
    const INTERVAL: u64 = 10;
    const MIN_INTERVAL: u64 = 2;

    fn create_transactions() -> [UnsignedTransaction<&'static str>; 3] {
        let tx_1 = UnsignedTransaction::new("transaction 1", 1, 1);
        let tx_2 = UnsignedTransaction::new("transaction 2", 2, 2);
        let tx_3 = UnsignedTransaction::new("transaction 2", 3, 3);
        [tx_1, tx_2, tx_3]
    }

    fn create_validator() -> TransactionValidator {
        let block_simulator = BlockSimulator::new(6);
        TransactionValidator::new(MAX, INTERVAL, MIN_INTERVAL, block_simulator)
    }

    fn create_pallet() -> UnsignedPallet<&'static str> {
        let validator = create_validator();
        UnsignedPallet::new(validator)
    }

    #[test]
    fn validate_transaction_test() {
        let mut validator = create_validator();
        let [tx_1, tx_2, tx_3] = create_transactions();

        let result = validator.validate_transaction(&tx_1);
        assert_eq!(result, ValidationResult::Valid);
        let _ = validator.accept_transaction(&tx_1);

        validator.block_simulator_mut().next_block();
        assert_eq!(validator.block_simulator.current_block(), 2);
        let result = validator.validate_transaction(&tx_2);
        assert_eq!(result, ValidationResult::Invalid(ValidationError::TooEarly));

        validator.block_simulator_mut().next_block();
        let result = validator.validate_transaction(&tx_1);
        assert_eq!(result, ValidationResult::Invalid(ValidationError::Duplicate));

        let _ = validator.accept_transaction(&tx_2);
        validator.block_simulator_mut().next_block();
        validator.block_simulator_mut().next_block();
        let result = validator.validate_transaction(&tx_3);
        assert_eq!(result, ValidationResult::Invalid(ValidationError::TooManyTransactions));
    }

    #[test]
    fn accept_transaction_test() {
        let mut validator = create_validator();
        let [tx_1, tx_2, tx_3] = create_transactions();
        let _ = validator.validate_transaction(&tx_1);
        let result = validator.accept_transaction(&tx_1);
        assert!(result.is_ok());

        validator.block_simulator_mut().next_block();
        let result = validator.accept_transaction(&tx_2);
        assert_eq!(result, Err(ValidationError::TooEarly));

        validator.block_simulator_mut().next_block();
        let result = validator.accept_transaction(&tx_1);
        assert_eq!(result, Err(ValidationError::Duplicate));

        let result = validator.accept_transaction(&tx_2);
        assert!(result.is_ok());
        validator.block_simulator_mut().next_block();
        validator.block_simulator_mut().next_block();
        let result = validator.accept_transaction(&tx_3);
        assert_eq!(result, Err(ValidationError::TooManyTransactions));
    }

    #[test]
    // Exemplo de teste refatorado
    #[test]
    fn cleanup_old_intervals_test() {
        let mut validator = create_validator();
        assert!(validator.accept_transaction(&UnsignedTransaction::new("tx1", 1, 1)).is_ok());

        validator.block_simulator_mut().advance_blocks(INTERVAL);
        assert!(validator.accept_transaction(&UnsignedTransaction::new("tx2", 11, 2)).is_ok());

        validator.block_simulator_mut().advance_blocks(INTERVAL);
        assert!(validator.accept_transaction(&UnsignedTransaction::new("tx3", 21, 3)).is_ok());

        assert_eq!(validator.interval_counts.len(), 3);

        validator.cleanup_old_intervals();

        assert_eq!(validator.interval_counts.len(), 2);
        assert!(validator.interval_counts.contains_key(&10));
        assert!(validator.interval_counts.contains_key(&20));
        assert!(!validator.interval_counts.contains_key(&0));
    }


    #[test]
    fn get_interval_stats() {
        let mut validator = create_validator();
        let [tx_1, _, _] = create_transactions();

        let _ = validator.accept_transaction(&tx_1);
        assert_eq!(validator.get_interval_stats(), (0, 1, 2));
        validator.block_simulator.current_block = 15;
        assert_eq!(validator.get_interval_stats(), (10, 0, 2));
    }

    #[test]
    fn next_allowed_block_test() {
        let mut validator = create_validator();
        let [tx_1, _, _] = create_transactions();
        let _ = validator.accept_transaction(&tx_1);
        assert!(validator.next_allowed_block().is_some());
        assert_eq!(validator.next_allowed_block().unwrap_or_default(), 3);
    }

    #[test]
    fn block_until_allowed_test() {
        let mut validator = create_validator();
        let [tx_1, _, _] = create_transactions();
        let _ = validator.accept_transaction(&tx_1);
        assert_eq!(validator.blocks_until_allowed(), 2);
    }

    #[test]
    fn submit_unsigned_test() {
        let [tx_1, tx_2, tx_3] = ["tx_1", "tx_2", "tx_3"];
        let mut pallet = create_pallet();
        let result = pallet.submit_unsigned(tx_1, 1);
        assert!(result.is_ok());
        assert_eq!(pallet.get_transaction_history(), &[(1,1)]);
        

        pallet.advance_block();
        let result = pallet.submit_unsigned(tx_1, 2);
        assert_eq!(result, Err(ValidationError::TooEarly));
        assert_eq!(pallet.get_transaction_history(), &[(1,1)]);

        pallet.advance_block();
        let result = pallet.submit_unsigned(tx_1, 1);
        assert_eq!(result, Err(ValidationError::Duplicate));
        assert_eq!(pallet.get_transaction_history(), &[(1,1)]);

        let result = pallet.submit_unsigned(tx_2, 2);
        assert!(result.is_ok());
        assert_eq!(pallet.get_transaction_history(), &[(1,1),(3,2)]);

        pallet.advance_block();
        pallet.advance_block();
        let result = pallet.submit_unsigned(tx_3, 3);
        assert_eq!(result, Err(ValidationError::TooManyTransactions));
        assert_eq!(pallet.get_transaction_history(), &[(1,1),(3,2)]);
        
        let mut all_data = pallet.get_all_data();
        all_data.sort_by_key(|k| k.0);
        assert_eq!(all_data, vec![(1, &"tx_1"), (2, &"tx_2")]);
        
    }
    

}

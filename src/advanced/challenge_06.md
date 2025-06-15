## Challenge 6: Simple Off-chain Worker for Price Quotes

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a simple off-chain worker that periodically fetches external price data and submits it to the blockchain via unsigned transactions. This challenge focuses on understanding how off-chain workers operate, how they interact with external APIs, and how they submit data back to the runtime.

**Main Concepts Covered:**
1. **Off-chain Workers:** Background processes that run alongside the runtime
2. **External API Integration:** Fetching data from external sources
3. **Unsigned Transaction Submission:** Sending data back to the blockchain
4. **Periodic Execution:** Running tasks at regular intervals
5. **Data Validation:** Ensuring external data integrity

### Detailed Structures to Implement:

#### **Price Data Structure:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: u64, // Price in cents to avoid floating point
    pub timestamp: u64,
}

impl PriceData {
    pub fn new(symbol: String, price: u64, timestamp: u64) -> Self {
        Self { symbol, price, timestamp }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.symbol.is_empty() && self.price > 0 && self.timestamp > 0
    }
}
```

#### **External API Client (Simulated):**
```rust
use std::collections::HashMap;

/// Simulates an external price API
pub struct MockPriceApi {
    prices: HashMap<String, u64>,
}

impl MockPriceApi {
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        prices.insert("BTC".to_string(), 4500000); // $45,000.00 in cents
        prices.insert("ETH".to_string(), 300000);  // $3,000.00 in cents
        prices.insert("DOT".to_string(), 700);     // $7.00 in cents
        
        Self { prices }
    }
    
    /// Simulate fetching price from external API
    pub fn fetch_price(&self, symbol: &str) -> Result<PriceData, &'static str> {
        match self.prices.get(symbol) {
            Some(&price) => {
                let timestamp = Self::current_timestamp();
                Ok(PriceData::new(symbol.to_string(), price, timestamp))
            },
            None => Err("Symbol not found"),
        }
    }
    
    /// Simulate getting current timestamp
    fn current_timestamp() -> u64 {
        // In real implementation, this would be actual timestamp
        1234567890
    }
    
    /// Update price (for testing)
    pub fn update_price(&mut self, symbol: String, price: u64) {
        self.prices.insert(symbol, price);
    }
}
```

#### **Off-chain Worker Configuration:**
```rust
pub trait Config {
    type BlockNumber: Clone + Copy + PartialEq + PartialOrd + core::fmt::Debug;
    type Call: From<Call<Self>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Call<T: Config> {
    SubmitPrice { price_data: PriceData },
    _Phantom(core::marker::PhantomData<T>),
}
```

#### **Pallet with Off-chain Worker:**
```rust
pub struct Pallet<T: Config> {
    /// Stored price data
    prices: std::collections::HashMap<String, PriceData>,
    /// Events emitted
    events: Vec<Event>,
    /// Last update block
    last_update_block: Option<T::BlockNumber>,
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    PriceUpdated { symbol: String, price: u64, timestamp: u64 },
    PriceSubmissionFailed { symbol: String, error: String },
    OffchainWorkerExecuted { block_number: u64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidPriceData,
    PriceAlreadyExists,
    ExternalApiFailed,
    SubmissionFailed,
}
```

### Off-chain Worker Implementation:

#### **Off-chain Worker Trait:**
```rust
pub trait OffchainWorker<T: Config> {
    /// Main off-chain worker function called every block
    fn offchain_worker(block_number: T::BlockNumber) -> Result<(), Error>;
    
    /// Fetch prices from external API
    fn fetch_external_prices() -> Result<Vec<PriceData>, Error>;
    
    /// Submit price data via unsigned transaction
    fn submit_price_unsigned(price_data: PriceData) -> Result<(), Error>;
    
    /// Check if we should run the worker this block
    fn should_run_worker(block_number: T::BlockNumber) -> bool;
}
```

#### **Off-chain Worker Implementation:**
```rust
impl<T: Config> OffchainWorker<T> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) -> Result<(), Error> {
        // Check if we should run this block (e.g., every 10 blocks)
        if !Self::should_run_worker(block_number) {
            return Ok(());
        }
        
        // Fetch prices from external API
        let prices = Self::fetch_external_prices()?;
        
        // Submit each price via unsigned transaction
        for price_data in prices {
            if let Err(e) = Self::submit_price_unsigned(price_data.clone()) {
                // Log error but continue with other prices
                log::warn!("Failed to submit price for {}: {:?}", price_data.symbol, e);
            }
        }
        
        Ok(())
    }
    
    fn fetch_external_prices() -> Result<Vec<PriceData>, Error> {
        let api = MockPriceApi::new();
        let symbols = vec!["BTC", "ETH", "DOT"];
        let mut prices = Vec::new();
        
        for symbol in symbols {
            match api.fetch_price(symbol) {
                Ok(price_data) => {
                    if price_data.is_valid() {
                        prices.push(price_data);
                    }
                },
                Err(_) => {
                    // Continue with other symbols if one fails
                    continue;
                }
            }
        }
        
        if prices.is_empty() {
            Err(Error::ExternalApiFailed)
        } else {
            Ok(prices)
        }
    }
    
    fn submit_price_unsigned(price_data: PriceData) -> Result<(), Error> {
        // In real implementation, this would create and submit an unsigned transaction
        // For simulation, we'll just validate the data
        if !price_data.is_valid() {
            return Err(Error::InvalidPriceData);
        }
        
        // Simulate transaction submission
        // let call = Call::SubmitPrice { price_data };
        // submit_unsigned_transaction(call)?;
        
        Ok(())
    }
    
    fn should_run_worker(block_number: T::BlockNumber) -> bool {
        // Run every 10 blocks (in real implementation, this would be configurable)
        // For simulation, we'll use a simple modulo check
        true // Simplified for this challenge
    }
}
```

#### **Pallet Implementation:**
```rust
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            prices: std::collections::HashMap::new(),
            events: Vec::new(),
            last_update_block: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Handle price submission from off-chain worker
    pub fn submit_price(
        &mut self,
        price_data: PriceData,
        block_number: T::BlockNumber,
    ) -> Result<(), Error> {
        // Validate price data
        if !price_data.is_valid() {
            return Err(Error::InvalidPriceData);
        }
        
        // Store price data
        self.prices.insert(price_data.symbol.clone(), price_data.clone());
        self.last_update_block = Some(block_number);
        
        // Emit event
        self.events.push(Event::PriceUpdated {
            symbol: price_data.symbol,
            price: price_data.price,
            timestamp: price_data.timestamp,
        });
        
        Ok(())
    }
    
    /// Get current price for a symbol
    pub fn get_price(&self, symbol: &str) -> Option<&PriceData> {
        self.prices.get(symbol)
    }
    
    /// Get all stored prices
    pub fn get_all_prices(&self) -> Vec<&PriceData> {
        self.prices.values().collect()
    }
    
    /// Check if price data is recent (within last N blocks)
    pub fn is_price_recent(&self, symbol: &str, current_block: T::BlockNumber, max_age: T::BlockNumber) -> bool {
        if let (Some(price_data), Some(last_update)) = (self.get_price(symbol), self.last_update_block) {
            current_block <= last_update || (current_block - last_update) <= max_age
        } else {
            false
        }
    }
    
    /// Take events for testing
    pub fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
}
```

### Unsigned Transaction Validation:

#### **ValidateUnsigned Implementation:**
```rust
pub trait ValidateUnsigned<T: Config> {
    fn validate_unsigned(call: &Call<T>) -> Result<ValidTransaction, TransactionValidityError>;
    fn pre_dispatch(call: &Call<T>) -> Result<(), TransactionValidityError>;
}

impl<T: Config> ValidateUnsigned<T> for Pallet<T> {
    fn validate_unsigned(call: &Call<T>) -> Result<ValidTransaction, TransactionValidityError> {
        match call {
            Call::SubmitPrice { price_data } => {
                // Validate price data
                if !price_data.is_valid() {
                    return Err(TransactionValidityError::Invalid);
                }
                
                // Create valid transaction
                Ok(ValidTransaction {
                    priority: 100, // Medium priority
                    requires: vec![],
                    provides: vec![format!("price_{}", price_data.symbol).into_bytes()],
                    longevity: 10, // Valid for 10 blocks
                    propagate: true,
                })
            },
            _ => Err(TransactionValidityError::Invalid),
        }
    }
    
    fn pre_dispatch(call: &Call<T>) -> Result<(), TransactionValidityError> {
        match call {
            Call::SubmitPrice { price_data } => {
                if price_data.is_valid() {
                    Ok(())
                } else {
                    Err(TransactionValidityError::Invalid)
                }
            },
            _ => Err(TransactionValidityError::Invalid),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionValidityError {
    Invalid,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct ValidTransaction {
    pub priority: u64,
    pub requires: Vec<Vec<u8>>,
    pub provides: Vec<Vec<u8>>,
    pub longevity: u64,
    pub propagate: bool,
}
```

### Tests

Create comprehensive tests covering:

1. **Price Data Validation:**
   - Test valid price data creation
   - Test invalid price data rejection
   - Test price data serialization/deserialization

2. **External API Simulation:**
   - Test successful price fetching
   - Test handling of missing symbols
   - Test API error handling

3. **Off-chain Worker:**
   - Test worker execution logic
   - Test price fetching and submission flow
   - Test error handling in worker

4. **Pallet Operations:**
   - Test price submission and storage
   - Test price retrieval
   - Test event emission
   - Test price freshness validation

5. **Unsigned Transaction Validation:**
   - Test valid price submission validation
   - Test invalid data rejection
   - Test transaction properties

### Expected Output

A complete off-chain worker system that:
- Simulates external API integration
- Implements proper off-chain worker patterns
- Handles unsigned transaction submission
- Validates external data before storage
- Demonstrates error handling and recovery
- Shows understanding of off-chain/on-chain interaction

### Theoretical Context

**Off-chain Workers in Substrate:**
- **Purpose:** Execute long-running tasks without blocking block production
- **Capabilities:** HTTP requests, local storage access, cryptographic operations
- **Isolation:** Run in separate threads, cannot directly modify runtime state
- **Communication:** Submit unsigned transactions or signed transactions back to runtime

**Use Cases:**
- **Oracle Data:** Fetching external price feeds, weather data, etc.
- **Computation:** Heavy calculations that would be too expensive on-chain
- **Monitoring:** Watching external events and triggering on-chain actions
- **Maintenance:** Periodic cleanup or optimization tasks

**Security Considerations:**
- Off-chain workers are not consensus-critical
- Data must be validated when submitted back to runtime
- Multiple nodes may run the same worker (idempotency important)
- External API failures must be handled gracefully

This challenge demonstrates the powerful pattern of extending blockchain capabilities through off-chain computation while maintaining security and decentralization.
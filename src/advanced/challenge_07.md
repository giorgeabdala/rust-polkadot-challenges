## Challenge 7: Inherent Data Provider

**Difficulty Level:** Advanced
**Estimated Time:** 1 hour

### Objective Description

You will implement a simplified inherent data provider system that demonstrates how external data is included in blocks during block construction. This challenge focuses on understanding Substrate's inherent transaction patterns and how external data sources integrate into blockchain operations.

### Main Concepts Covered

1. **Inherent Data**: External data that must be included in blocks
2. **Provider Pattern**: Systems that supply data for block construction
3. **Block Construction**: How inherents are integrated during block building
4. **Data Validation**: Ensuring inherent data meets requirements
5. **Timestamp Inherents**: Time-based data essential for blockchain operation

### Structures to Implement

#### **Inherent Data Container:**
```rust
use std::collections::HashMap;

/// Container for all inherent data in a block
#[derive(Debug, Clone)]
pub struct InherentData {
    /// Map of inherent identifier to data
    data: HashMap<String, Vec<u8>>,
}

impl InherentData {
    /// Create new empty inherent data container
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Put data into the container
    pub fn put_data(&mut self, identifier: &str, data: Vec<u8>) {
        self.data.insert(identifier.to_string(), data);
    }
    
    /// Get data from the container
    pub fn get_data(&self, identifier: &str) -> Option<&Vec<u8>> {
        self.data.get(identifier)
    }
    
    /// Check if identifier exists
    pub fn has_data(&self, identifier: &str) -> bool {
        self.data.contains_key(identifier)
    }
    
    /// Get all identifiers
    pub fn identifiers(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}
```

#### **Inherent Data Provider Trait:**
```rust
/// Trait for providing inherent data
pub trait InherentDataProvider {
    /// Identifier for this provider's data
    const INHERENT_IDENTIFIER: &'static str;
    
    /// Provide data for block construction
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str>;
    
    /// Check if inherent data is required
    fn is_inherent_required(&self) -> bool {
        true
    }
    
    /// Get error message for missing inherent
    fn error_message(&self) -> &'static str {
        "Required inherent data missing"
    }
}
```

#### **Timestamp Provider:**
```rust
use std::time::{SystemTime, UNIX_EPOCH};

/// Timestamp data structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    /// Milliseconds since Unix epoch
    pub millis: u64,
}

impl Timestamp {
    /// Create timestamp from current time
    pub fn now() -> Self {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self { millis }
    }
    
    /// Create timestamp from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        Self { millis }
    }
    
    /// Get milliseconds
    pub fn as_millis(&self) -> u64 {
        self.millis
    }
    
    /// Convert to bytes for storage
    pub fn to_bytes(&self) -> Vec<u8> {
        self.millis.to_le_bytes().to_vec()
    }
    
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 8 {
            return Err("Invalid timestamp bytes length");
        }
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        let millis = u64::from_le_bytes(array);
        Ok(Self::from_millis(millis))
    }
}

/// Provides timestamp inherent data
pub struct TimestampProvider {
    /// Custom timestamp (for testing)
    custom_timestamp: Option<Timestamp>,
}

impl TimestampProvider {
    /// Create new timestamp provider
    pub fn new() -> Self {
        Self {
            custom_timestamp: None,
        }
    }
    
    /// Set custom timestamp for testing
    pub fn with_custom_timestamp(mut self, timestamp: Timestamp) -> Self {
        self.custom_timestamp = Some(timestamp);
        self
    }
    
    /// Get current timestamp
    fn get_timestamp(&self) -> Timestamp {
        match self.custom_timestamp {
            Some(ts) => ts,
            None => Timestamp::now(),
        }
    }
}

impl InherentDataProvider for TimestampProvider {
    const INHERENT_IDENTIFIER: &'static str = "timestamp";
    
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut inherent_data = InherentData::new();
        let timestamp = self.get_timestamp();
        inherent_data.put_data(Self::INHERENT_IDENTIFIER, timestamp.to_bytes());
        Ok(inherent_data)
    }
    
    fn error_message(&self) -> &'static str {
        "Timestamp inherent is required for block construction"
    }
}
```

#### **Block Constructor:**
```rust
/// Simulates block construction with inherents
pub struct BlockConstructor {
    /// Registered inherent providers
    providers: Vec<Box<dyn InherentDataProvider>>,
    /// Block number
    block_number: u64,
}

impl BlockConstructor {
    /// Create new block constructor
    pub fn new(block_number: u64) -> Self {
        Self {
            providers: Vec::new(),
            block_number,
        }
    }
    
    /// Register an inherent data provider
    pub fn register_provider(&mut self, provider: Box<dyn InherentDataProvider>) {
        self.providers.push(provider);
    }
    
    /// Get block number
    pub fn block_number(&self) -> u64 {
        self.block_number
    }
    
    /// Get number of registered providers
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}
```

#### **Block Structure:**
```rust
/// Simplified block structure
#[derive(Debug, Clone)]
pub struct Block {
    /// Block number
    pub block_number: u64,
    /// Inherent data included in block
    pub inherent_data: InherentData,
    /// Block timestamp
    pub timestamp: u64,
}

impl Block {
    /// Get timestamp from inherent data
    pub fn get_inherent_timestamp(&self) -> Result<Option<Timestamp>, &'static str> {
        if let Some(timestamp_bytes) = self.inherent_data.get_data("timestamp") {
            let timestamp = Timestamp::from_bytes(timestamp_bytes)?;
            Ok(Some(timestamp))
        } else {
            Ok(None)
        }
    }
    
    /// Basic block validation
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.block_number == 0 {
            return Err("Block number cannot be zero");
        }
        
        if self.timestamp == 0 {
            return Err("Block timestamp cannot be zero");
        }
        
        Ok(())
    }
}
```

### Methods for You to Implement

#### **1. Collect Inherent Data (`collect_inherent_data`):**
```rust
impl BlockConstructor {
    // TODO: Implement this method
    /// Collect all inherent data from registered providers
    pub fn collect_inherent_data(&self) -> Result<InherentData, &'static str> {
        // IMPLEMENT:
        // 1. Create new empty InherentData
        // 2. Iterate through all providers
        // 3. For each provider that is_inherent_required():
        //    - Call provide_inherent_data()
        //    - Merge the data into combined_data
        //    - Check for duplicate identifiers (return error if found)
        // 4. Return the combined InherentData
        todo!()
    }
}
```

#### **2. Validate Inherents (`validate_inherents`):**
```rust
impl BlockConstructor {
    // TODO: Implement this method
    /// Validate that all required inherents are present
    pub fn validate_inherents(&self, inherent_data: &InherentData) -> Result<(), &'static str> {
        // IMPLEMENT:
        // 1. Check that all required inherents are present
        // 2. For each provider that is_inherent_required():
        //    - Check if inherent_data.has_data(provider's identifier)
        //    - Return provider.error_message() if missing
        // 3. Validate timestamp if present:
        //    - Get timestamp bytes and convert to Timestamp
        //    - Check if timestamp > 0
        //    - Basic drift check (not too far in future)
        // 4. Return Ok(()) if all validations pass
        todo!()
    }
}
```

#### **3. Build Block (`build_block`):**
```rust
impl BlockConstructor {
    // TODO: Implement this method
    /// Build complete block with inherents
    pub fn build_block(&self) -> Result<Block, &'static str> {
        // IMPLEMENT:
        // 1. Call collect_inherent_data() to get all inherents
        // 2. Call validate_inherents() to ensure data is valid
        // 3. Extract timestamp from inherents for block header
        // 4. Create and return Block with:
        //    - block_number from self
        //    - inherent_data collected
        //    - timestamp extracted (or current time if missing)
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
    fn test_basic_functionality() {
        // TODO: Implement this test
        // 1. Create InherentData container
        // 2. Test put_data with some sample data
        // 3. Test get_data retrieves the same data
        // 4. Test has_data returns true for existing identifier
        // 5. Test identifiers() returns correct list
        todo!()
    }

    #[test]
    fn test_timestamp_provider() {
        // TODO: Implement this test
        // 1. Create TimestampProvider with custom timestamp
        // 2. Call provide_inherent_data()
        // 3. Verify data contains "timestamp" identifier
        // 4. Extract timestamp bytes and decode back to Timestamp
        // 5. Verify decoded timestamp matches original
        todo!()
    }

    #[test]
    fn test_block_construction_success() {
        // TODO: Implement this test
        // 1. Create BlockConstructor
        // 2. Register TimestampProvider
        // 3. Call build_block() and verify it succeeds
        // 4. Check block contains timestamp in inherent_data
        // 5. Verify block.validate() passes
        todo!()
    }

    #[test]
    fn test_missing_required_inherent() {
        // TODO: Implement this test
        // 1. Create BlockConstructor without any providers
        // 2. Try to build_block() and verify it fails
        // 3. Check error message is about missing timestamp
        todo!()
    }
}
```

### Example Usage

```rust
fn main() {
    let mut constructor = BlockConstructor::new(1);
    
    // Register timestamp provider
    let timestamp_provider = Box::new(TimestampProvider::new());
    constructor.register_provider(timestamp_provider);
    
    println!("Registered {} providers", constructor.provider_count());
    
    // Build block
    match constructor.build_block() {
        Ok(block) => {
            println!("Built block #{}", block.block_number);
            
            if let Ok(Some(timestamp)) = block.get_inherent_timestamp() {
                println!("Block timestamp: {}", timestamp.as_millis());
            }
            
            match block.validate() {
                Ok(()) => println!("Block validation: PASSED"),
                Err(e) => println!("Block validation: FAILED - {}", e),
            }
        }
        Err(e) => println!("Block construction failed: {}", e),
    }
}
```

### Expected Output

A complete inherent data system that:
- Demonstrates Substrate's inherent provider pattern
- Handles external data integration during block construction
- Validates required inherents are present
- Shows proper timestamp handling essential in blockchain systems
- Implements error handling for missing or invalid data

### Theoretical Context

**Inherents in Substrate:**
- **Purpose**: Include external data that must be in every block
- **Block Construction**: Inherents are added during block authoring phase
- **Validation**: Checked both at construction and block import time
- **Essential Data**: Timestamps, validator sets, oracle feeds, etc.

**Provider Pattern:**
- **Modularity**: Different providers handle different types of data
- **Registration**: Providers register with block construction system
- **Lifecycle**: Called during each block construction cycle
- **Reliability**: System continues even if optional providers fail

**Timestamp Inherents:**
- **Universal Requirement**: Every Substrate block needs a timestamp
- **Consensus**: Provides time reference for consensus mechanisms
- **Validation**: Prevents blocks with invalid timestamps
- **Drift Protection**: Basic checks against unrealistic timestamps

**Key Design Patterns:**
1. **Provider Registration**: Modular data source management
2. **Data Collection**: Aggregating multiple data sources
3. **Validation Pipeline**: Ensuring data integrity before block creation
4. **Error Isolation**: Individual provider failures don't break entire system

This challenge teaches essential Substrate concepts for understanding how external data integrates into blockchain operations and the critical role of inherents in block construction. 
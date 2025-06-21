## Challenge 7: Inherent Data Provider

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement an inherent data provider system that simulates how external data is included in blocks during the block construction process. This challenge focuses on understanding how Substrate handles inherent transactions and how external data sources are integrated into the blockchain.

**Main Concepts Covered:**
1. **Inherent Data:** External data that must be included in blocks
2. **Provider Pattern:** Systems that supply data for block construction
3. **Block Construction:** How inherents are integrated during block building
4. **Data Validation:** Ensuring inherent data meets requirements
5. **Timestamp Inherents:** Time-based data that's essential for blockchain operation

### Project Setup

Before starting, you will need to configure the necessary dependencies:

#### **Cargo.toml:**
```toml
[package]
name = "inherents-challenge"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### **How to configure (choose one option):**

**Option 1 - Using cargo add (recommended):**
```bash
cargo add serde --features derive
cargo add serde_json
```

**Option 2 - Editing Cargo.toml manually:**
```bash
# Edit the Cargo.toml file above and then run:
cargo build
```

### Detailed Structures to Implement:

#### **Inherent Data Container:**
```rust
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Container for all inherent data in a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InherentData {
    /// Map of inherent identifier to JSON-encoded data
    data: HashMap<String, String>,
}

impl InherentData {
    /// Create new empty inherent data container
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Put data into the container
    pub fn put_data<T: Serialize>(&mut self, identifier: &str, data: &T) -> Result<(), &'static str> {
        let json_data = serde_json::to_string(data)
            .map_err(|_| "Failed to serialize data")?;
        self.data.insert(identifier.to_string(), json_data);
        Ok(())
    }
    
    /// Get data from the container
    pub fn get_data<T: for<'de> Deserialize<'de>>(&self, identifier: &str) -> Result<Option<T>, &'static str> {
        match self.data.get(identifier) {
            Some(json_data) => {
                let data = serde_json::from_str(json_data)
                    .map_err(|_| "Failed to deserialize data")?;
                Ok(Some(data))
            },
            None => Ok(None),
        }
    }
    
    /// Check if identifier exists
    pub fn has_data(&self, identifier: &str) -> bool {
        self.data.contains_key(identifier)
    }
    
    /// Get all identifiers
    pub fn identifiers(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    
    /// Get total data size
    pub fn total_size(&self) -> usize {
        self.data.values().map(|v| v.len()).sum()
    }
    
    /// Get raw JSON data for debugging
    pub fn get_raw_data(&self, identifier: &str) -> Option<&String> {
        self.data.get(identifier)
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
use serde::{Serialize, Deserialize};

/// Timestamp data structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        inherent_data.put_data(Self::INHERENT_IDENTIFIER, &timestamp)?;
        Ok(inherent_data)
    }
    
    fn error_message(&self) -> &'static str {
        "Timestamp inherent is required for block construction"
    }
}
```

#### **Block Construction Simulator:**
```rust
/// Simulates block construction with inherents
pub struct BlockConstructor {
    /// Registered inherent providers
    providers: Vec<Box<dyn InherentDataProvider>>,
    /// Block number
    block_number: u64,
    /// Parent hash (simplified)
    parent_hash: [u8; 32],
}

impl BlockConstructor {
    /// Create new block constructor
    pub fn new(block_number: u64, parent_hash: [u8; 32]) -> Self {
        Self {
            providers: Vec::new(),
            block_number,
            parent_hash,
        }
    }
    
    /// Register an inherent data provider
    pub fn register_provider(&mut self, provider: Box<dyn InherentDataProvider>) {
        self.providers.push(provider);
    }
    
    /// Collect all inherent data
    pub fn collect_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut combined_data = InherentData::new();
        
        for provider in &self.providers {
            if provider.is_inherent_required() {
                let provider_data = provider.provide_inherent_data()?;
                
                // Merge data from this provider
                for identifier in provider_data.identifiers() {
                    if combined_data.has_data(&identifier) {
                        return Err("Duplicate inherent identifier");
                    }
                    
                    // Get raw JSON data and copy it
                    if let Some(raw_data) = provider_data.get_raw_data(&identifier) {
                        combined_data.data.insert(identifier, raw_data.clone());
                    }
                }
            }
        }
        
        Ok(combined_data)
    }
    
    /// Validate inherent data
    pub fn validate_inherents(&self, inherent_data: &InherentData) -> Result<(), &'static str> {
        // Check that all required inherents are present
        for provider in &self.providers {
            if provider.is_inherent_required() {
                let identifier = Self::get_provider_identifier(provider.as_ref());
                if !inherent_data.has_data(&identifier) {
                    return Err(provider.error_message());
                }
            }
        }
        
        // Validate timestamp if present
        if inherent_data.has_data("timestamp") {
            let timestamp: Timestamp = inherent_data.get_data("timestamp")?
                .ok_or("Timestamp data corrupted")?;
            
            // Basic validation: timestamp should be reasonable
            if timestamp.as_millis() == 0 {
                return Err("Invalid timestamp: cannot be zero");
            }
            
            // Check if timestamp is not too far in the future (basic drift check)
            let now = Timestamp::now();
            let max_drift = 60_000; // 60 seconds in milliseconds
            
            if timestamp.as_millis() > now.as_millis() + max_drift {
                return Err("Timestamp too far in the future");
            }
        }
        
        Ok(())
    }
    
    /// Get provider identifier (helper method)
    fn get_provider_identifier(provider: &dyn InherentDataProvider) -> String {
        // This is a simplified way to get the identifier
        // In real implementation, this would use the const INHERENT_IDENTIFIER
        "timestamp".to_string() // For simplicity, assuming timestamp
    }
    
    /// Build block with inherents
    pub fn build_block(&self) -> Result<Block, &'static str> {
        let inherent_data = self.collect_inherent_data()?;
        self.validate_inherents(&inherent_data)?;
        
        // Get timestamp from inherents for block header
        let block_timestamp = if let Some(timestamp) = inherent_data.get_data::<Timestamp>("timestamp")? {
            timestamp.as_millis()
        } else {
            Timestamp::now().as_millis()
        };
        
        Ok(Block {
            block_number: self.block_number,
            parent_hash: self.parent_hash,
            inherent_data,
            timestamp: block_timestamp,
        })
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block number
    pub block_number: u64,
    /// Parent block hash
    pub parent_hash: [u8; 32],
    /// Inherent data included in block
    pub inherent_data: InherentData,
    /// Block timestamp
    pub timestamp: u64,
}

impl Block {
    /// Get timestamp from inherent data
    pub fn get_inherent_timestamp(&self) -> Result<Option<Timestamp>, &'static str> {
        self.inherent_data.get_data("timestamp")
    }
    
    /// Validate block inherents
    pub fn validate(&self) -> Result<(), &'static str> {
        // Basic validation
        if self.block_number == 0 {
            return Err("Block number cannot be zero");
        }
        
        // Check timestamp
        if let Some(timestamp) = self.get_inherent_timestamp()? {
            if timestamp.as_millis() == 0 {
                return Err("Block timestamp cannot be zero");
            }
        }
        
        Ok(())
    }
    
    /// Get block hash (simplified)
    pub fn hash(&self) -> [u8; 32] {
        // Simple hash based on block number and timestamp
        let mut hash = [0u8; 32];
        let combined = self.block_number.wrapping_add(self.timestamp);
        let bytes = combined.to_le_bytes();
        
        for (i, &byte) in bytes.iter().enumerate() {
            hash[i % 32] ^= byte;
        }
        
        hash
    }
    
    /// Serialize block to JSON
    pub fn to_json(&self) -> Result<String, &'static str> {
        serde_json::to_string_pretty(self)
            .map_err(|_| "Failed to serialize block")
    }
    
    /// Deserialize block from JSON
    pub fn from_json(json: &str) -> Result<Self, &'static str> {
        serde_json::from_str(json)
            .map_err(|_| "Failed to deserialize block")
    }
}
```

#### **Custom Inherent Provider Example:**
```rust
/// Custom data structure for inherents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomData {
    pub message: String,
    pub value: u64,
    pub active: bool,
}

/// Custom inherent provider for demonstration
pub struct CustomDataProvider {
    data: CustomData,
    required: bool,
}

impl CustomDataProvider {
    pub fn new(message: String, value: u64, active: bool, required: bool) -> Self {
        Self { 
            data: CustomData { message, value, active },
            required,
        }
    }
}

impl InherentDataProvider for CustomDataProvider {
    const INHERENT_IDENTIFIER: &'static str = "custom_data";
    
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut inherent_data = InherentData::new();
        inherent_data.put_data(Self::INHERENT_IDENTIFIER, &self.data)?;
        Ok(inherent_data)
    }
    
    fn is_inherent_required(&self) -> bool {
        self.required
    }
    
    fn error_message(&self) -> &'static str {
        "Custom inherent data is required"
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Inherent Data Container:**
   - Test data insertion and retrieval with serde
   - Test JSON serialization/deserialization
   - Test error handling for invalid JSON data

2. **Timestamp Provider:**
   - Test timestamp generation and serialization
   - Test custom timestamp setting
   - Test inherent data format with JSON

3. **Block Construction:**
   - Test provider registration
   - Test inherent data collection and merging
   - Test validation of required inherents

4. **Block Validation:**
   - Test valid block construction with JSON serialization
   - Test missing required inherents
   - Test duplicate inherent identifiers

5. **Integration:**
   - Test full block construction process with serde
   - Test multiple providers working together
   - Test JSON export/import of complete blocks

### Expected Output

A complete inherent data system that:
- Uses serde for JSON serialization/deserialization
- Provides external data for block construction
- Validates required inherents are present
- Handles encoding/decoding of inherent data
- Demonstrates understanding of Substrate's inherent system
- Shows proper error handling and validation

### Theoretical Context

**Inherents in Substrate:**
- **Purpose:** Include external data that must be in every block
- **Examples:** Timestamps, validator set changes, oracle data
- **Block Construction:** Inherents are added during block authoring
- **Validation:** Both at construction time and when importing blocks
- **Providers:** External systems that supply inherent data

**Serialization in Substrate:**
- **serde:** Standard Rust serialization framework
- **JSON Format:** Human-readable format for debugging and testing
- **Encoding:** Data must be encoded for storage in blocks
- **Decoding:** Blocks must be decoded when imported/validated

**Inherent Data Flow:**
1. **Collection:** Providers supply data during block construction
2. **Serialization:** Data is serialized to JSON format
3. **Validation:** Data is validated before inclusion
4. **Inclusion:** Inherent transactions are added to block
5. **Verification:** Imported blocks are validated for required inherents

**Best Practices:**
- Always validate inherent data before inclusion
- Handle serialization errors gracefully
- Ensure required inherents are always present
- Use unique identifiers to prevent conflicts
- Provide clear error messages for debugging

This system demonstrates how external data is reliably included in blockchain blocks through Substrate's inherent mechanism with proper serialization support. 
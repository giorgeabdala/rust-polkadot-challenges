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

This challenge uses only Rust's standard library:

```toml
[package]
name = "inherents-challenge"
version = "0.1.0"
edition = "2021"

[dependencies]
# No external dependencies required!
```

**Focus on essential concepts:**
- Basic inherent data container
- Provider trait pattern
- Timestamp provider implementation
- Simple validation logic
- Essential testing

### Detailed Structures to Implement:

#### **Inherent Data Container:**
```rust
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple container for inherent data
#[derive(Debug, Clone)]
pub struct InherentData {
    /// Map of identifier to data (as simple strings)
    data: HashMap<String, String>,
}

impl InherentData {
    /// Create new empty container
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Insert data into container
    pub fn put_data(&mut self, identifier: &str, data: &str) {
        self.data.insert(identifier.to_string(), data.to_string());
    }
    
    /// Get data from container
    pub fn get_data(&self, identifier: &str) -> Option<&String> {
        self.data.get(identifier)
    }
    
    /// Check if identifier has data
    pub fn has_data(&self, identifier: &str) -> bool {
        self.data.contains_key(identifier)
    }
    
    /// List all identifiers
    pub fn identifiers(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}
```

#### **Inherent Data Provider Trait:**
```rust
/// Trait for providing inherent data
pub trait InherentDataProvider {
    /// Get provider identifier
    fn get_identifier(&self) -> &'static str;
    
    /// Provide inherent data for block construction
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str>;
    
    /// Check if this inherent is required
    fn is_required(&self) -> bool {
        true
    }
}
```

#### **Timestamp Provider:**
```rust
/// Simple timestamp data structure
#[derive(Debug, Clone, Copy, PartialEq)]
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
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        self.millis.to_string()
    }
    
    /// Parse timestamp from string
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        s.parse::<u64>()
            .map(|millis| Self { millis })
            .map_err(|_| "Invalid timestamp format")
    }
}

/// Timestamp inherent data provider
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
    
    /// Get current or custom timestamp
    fn get_timestamp(&self) -> Timestamp {
        self.custom_timestamp.unwrap_or_else(Timestamp::now)
    }
}

impl InherentDataProvider for TimestampProvider {
    fn get_identifier(&self) -> &'static str {
        "timestamp"
    }
    
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut inherent_data = InherentData::new();
        let timestamp = self.get_timestamp();
        inherent_data.put_data(self.get_identifier(), &timestamp.to_string());
        Ok(inherent_data)
    }
}
```

#### **Inherent Data Collector:**
```rust
/// Collector for inherent data from multiple providers
pub struct InherentCollector {
    providers: Vec<Box<dyn InherentDataProvider>>,
}

impl InherentCollector {
    /// Create new collector
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    
    /// Add a provider to the collector
    pub fn add_provider(&mut self, provider: Box<dyn InherentDataProvider>) {
        self.providers.push(provider);
    }
    
    /// Collect all inherent data from registered providers
    pub fn collect_inherents(&self) -> Result<InherentData, &'static str> {
        let mut combined = InherentData::new();
        
        for provider in &self.providers {
            if provider.is_required() {
                let data = provider.provide_inherent_data()?;
                
                // Check for duplicates
                let identifier = provider.get_identifier();
                if combined.has_data(identifier) {
                    return Err("Duplicate inherent identifier");
                }
                
                // Copy data
                if let Some(value) = data.get_data(identifier) {
                    combined.put_data(identifier, value);
                }
            }
        }
        
        Ok(combined)
    }
    
    /// Validate inherent data
    pub fn validate_inherents(&self, inherents: &InherentData) -> Result<(), &'static str> {
        // Check if timestamp exists and is valid
        if let Some(timestamp_str) = inherents.get_data("timestamp") {
            let timestamp = Timestamp::from_string(timestamp_str)?;
            
            if timestamp.millis == 0 {
                return Err("Timestamp cannot be zero");
            }
            
            // Check for reasonable drift (simple check)
            let now = Timestamp::now();
            let max_drift = 60_000; // 60 seconds
            
            if timestamp.millis > now.millis + max_drift {
                return Err("Timestamp too far in the future");
            }
        }
        
        Ok(())
    }
    
    /// Get number of registered providers
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}
```

#### **Simple Block Structure:**
```rust
/// Simple block structure with inherent data
#[derive(Debug, Clone)]
pub struct SimpleBlock {
    pub block_number: u64,
    pub inherent_data: InherentData,
    pub timestamp: u64,
}

impl SimpleBlock {
    /// Create new block with inherent data
    pub fn new(block_number: u64, inherents: InherentData) -> Result<Self, &'static str> {
        let timestamp = if let Some(ts_str) = inherents.get_data("timestamp") {
            Timestamp::from_string(ts_str)?.millis
        } else {
            Timestamp::now().millis
        };
        
        Ok(Self {
            block_number,
            inherent_data: inherents,
            timestamp,
        })
    }
    
    /// Validate block data
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.block_number == 0 {
            return Err("Block number cannot be zero");
        }
        
        if self.timestamp == 0 {
            return Err("Timestamp cannot be zero");
        }
        
        Ok(())
    }
}
```

### Tests

Create 5 essential tests covering:

1. **Inherent Data Container:**
   - Basic data insertion and retrieval
   - Identifier existence verification

2. **Timestamp Provider:**
   - Timestamp generation testing
   - Custom timestamp testing

3. **Inherent Collector:**
   - Provider registration testing
   - Inherent data collection testing

4. **Simple Block:**
   - Valid block creation testing
   - Block validation testing

5. **Timestamp Validation:**
   - Basic timestamp validation
   - Invalid timestamp detection

### Expected Output

A functional inherent data system that:
- Uses only Rust's standard library
- Provides external data for block construction
- Validates essential inherent data (timestamps)
- Demonstrates understanding of Substrate's inherent concepts
- Shows proper error handling and validation
- **Focus:** Essential concepts with clean implementation

### Theoretical Context

**Inherents in Substrate:**
- **Purpose:** Include external data that must be in every block
- **Examples:** Timestamps, validator set changes, oracle data
- **Block Construction:** Inherents are added during block authoring
- **Validation:** Both at construction time and when importing blocks
- **Providers:** External systems that supply inherent data

**Data Encoding:**
- **String Format:** Simple format for learning the concepts
- **Conversion:** Data converted to/from strings for storage
- **Note:** In production, Substrate uses SCALE codec, not simple strings

**Inherent Data Flow:**
1. **Collection:** Providers supply data during block construction
2. **Encoding:** Data is converted to string format
3. **Validation:** Data is validated before inclusion
4. **Inclusion:** Inherent data is added to block
5. **Verification:** Imported blocks are validated for required inherents

**Best Practices:**
- Always validate inherent data before inclusion
- Handle serialization errors gracefully
- Ensure required inherents are always present
- Use unique identifiers to prevent conflicts
- Provide clear error messages for debugging

This system demonstrates how external data is reliably included in blockchain blocks through Substrate's inherent mechanism, focusing on essential concepts with a clean, understandable implementation. 
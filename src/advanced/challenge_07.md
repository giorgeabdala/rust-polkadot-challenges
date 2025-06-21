## Challenge 7: Inherents - Timestamp and External Data (Simplified)

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a simplified inherent data system that demonstrates how mandatory information (like timestamps) is included in every block. This challenge focuses on understanding the core concept of inherents with essential external dependencies.

**Main Concepts Covered:**
1. **Inherent Data:** Mandatory data included in every block
2. **Timestamp Inherents:** Block timestamp management
3. **Data Validation:** Ensuring inherent data correctness
4. **Block Construction:** How inherents are included during block building

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
bincode = "1.3"
```

#### **How to configure (choose one option):**

**Option 1 - Using cargo add (recommended):**
```bash
cargo add serde --features derive
cargo add bincode
```

**Option 2 - Editing Cargo.toml manually:**
```bash
# Edit the Cargo.toml file above and then run:
cargo build
```

### Detailed Structures to Implement:

#### **Simple Inherent Data System:**
```rust
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Identifier for inherent data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InherentIdentifier(pub [u8; 8]);

impl InherentIdentifier {
    pub const TIMESTAMP: InherentIdentifier = InherentIdentifier(*b"timstap0");
}

/// Container for inherent data
#[derive(Debug, Clone, PartialEq)]
pub struct InherentData {
    data: HashMap<InherentIdentifier, Vec<u8>>,
}

impl InherentData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Put inherent data
    pub fn put_data<T: Serialize>(&mut self, identifier: InherentIdentifier, data: &T) -> Result<(), &'static str> {
        let encoded = bincode::serialize(data).map_err(|_| "Failed to encode inherent data")?;
        self.data.insert(identifier, encoded);
        Ok(())
    }
    
    /// Get inherent data
    pub fn get_data<T: for<'de> Deserialize<'de>>(&self, identifier: &InherentIdentifier) -> Result<Option<T>, &'static str> {
        match self.data.get(identifier) {
            Some(data) => {
                match bincode::deserialize(data) {
                    Ok(decoded) => Ok(Some(decoded)),
                    Err(_) => Err("Failed to decode inherent data"),
                }
            },
            None => Ok(None),
        }
    }
    
    /// Check if inherent data exists
    pub fn has_data(&self, identifier: &InherentIdentifier) -> bool {
        self.data.contains_key(identifier)
    }
}
```

#### **Timestamp Inherent:**
```rust
/// Timestamp in milliseconds since Unix epoch
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn now() -> Self {
        Self(1640995200000) // Simulated timestamp
    }
    
    pub fn as_millis(&self) -> u64 {
        self.0
    }
}

/// Timestamp inherent data provider
pub struct TimestampInherentDataProvider {
    timestamp: Timestamp,
}

impl TimestampInherentDataProvider {
    pub fn new() -> Self {
        Self {
            timestamp: Timestamp::now(),
        }
    }
    
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        Self { timestamp }
    }
    
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
    
    /// Provide timestamp inherent data
    pub fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), &'static str> {
        inherent_data.put_data(InherentIdentifier::TIMESTAMP, &self.timestamp)
    }
    
    /// Check if the timestamp inherent data is valid
    pub fn check_inherent(&self, inherent_data: &InherentData) -> Result<(), &'static str> {
        let timestamp: Option<Timestamp> = inherent_data.get_data(&InherentIdentifier::TIMESTAMP)?;
        
        match timestamp {
            Some(ts) => {
                // Check if timestamp is reasonable (not too far in past/future)
                let current = Timestamp::now();
                let max_drift = 60_000; // 60 seconds in milliseconds
                
                if ts.as_millis() > current.as_millis() + max_drift {
                    Err("Timestamp too far in the future")
                } else if current.as_millis() > ts.as_millis() + max_drift {
                    Err("Timestamp too far in the past")
                } else {
                    Ok(())
                }
            },
            None => Err("Missing timestamp inherent"),
        }
    }
}
```

#### **Block Construction with Inherents:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub inherents: InherentData,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    pub number: u64,
    pub parent_hash: [u8; 32],
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: u64,
    pub data: Vec<u8>,
}

pub struct BlockBuilder {
    timestamp_provider: TimestampInherentDataProvider,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            timestamp_provider: TimestampInherentDataProvider::new(),
        }
    }
    
    /// Build a new block with inherent data
    pub fn build_block(
        &self,
        number: u64,
        parent_hash: [u8; 32],
        transactions: Vec<Transaction>,
    ) -> Result<Block, &'static str> {
        // Create inherent data
        let mut inherents = InherentData::new();
        self.timestamp_provider.provide_inherent_data(&mut inherents)?;
        
        // Extract timestamp from inherents
        let timestamp: Timestamp = inherents.get_data(&InherentIdentifier::TIMESTAMP)?
            .ok_or("Missing timestamp in inherents")?;
        
        // Create block header
        let header = BlockHeader {
            number,
            parent_hash,
            timestamp,
        };
        
        // Create complete block
        let block = Block {
            header,
            inherents,
            transactions,
        };
        
        Ok(block)
    }
    
    /// Validate a block's inherent data
    pub fn validate_block(&self, block: &Block) -> Result<(), &'static str> {
        // Validate inherent data
        self.timestamp_provider.check_inherent(&block.inherents)?;
        
        // Validate header consistency with inherents
        let inherent_timestamp: Timestamp = block.inherents.get_data(&InherentIdentifier::TIMESTAMP)?
            .ok_or("Missing timestamp in inherents")?;
        
        if block.header.timestamp != inherent_timestamp {
            return Err("Header timestamp doesn't match inherent timestamp");
        }
        
        Ok(())
    }
}
```

### Implementation Requirements:

1. **`InherentData`**: Implement the container with serde/bincode encoding
2. **`Timestamp`**: Implement with proper serialization
3. **`TimestampInherentDataProvider`**: Implement data provision and validation
4. **`BlockBuilder`**: Implement block construction and validation

### Test Configuration:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inherent_data_serialization_works() {
        let mut inherents = InherentData::new();
        let timestamp = Timestamp(1234567890);
        
        inherents.put_data(InherentIdentifier::TIMESTAMP, &timestamp).unwrap();
        assert!(inherents.has_data(&InherentIdentifier::TIMESTAMP));
        
        let stored_timestamp: Timestamp = inherents.get_data(&InherentIdentifier::TIMESTAMP).unwrap().unwrap();
        assert_eq!(timestamp, stored_timestamp);
    }

    #[test]
    fn timestamp_validation_works() {
        let provider = TimestampInherentDataProvider::new();
        let mut inherents = InherentData::new();
        
        // Valid timestamp
        provider.provide_inherent_data(&mut inherents).unwrap();
        assert!(provider.check_inherent(&inherents).is_ok());
        
        // Invalid timestamp (too far in future)
        let future_timestamp = Timestamp(Timestamp::now().as_millis() + 120_000);
        let future_provider = TimestampInherentDataProvider::from_timestamp(future_timestamp);
        let mut future_inherents = InherentData::new();
        future_provider.provide_inherent_data(&mut future_inherents).unwrap();
        assert!(future_provider.check_inherent(&future_inherents).is_err());
    }

    #[test]
    fn block_construction_works() {
        let builder = BlockBuilder::new();
        let transactions = vec![
            Transaction { id: 1, data: vec![1, 2, 3] },
            Transaction { id: 2, data: vec![4, 5, 6] },
        ];
        
        let block = builder.build_block(1, [0u8; 32], transactions).unwrap();
        
        assert_eq!(block.header.number, 1);
        assert!(block.inherents.has_data(&InherentIdentifier::TIMESTAMP));
        assert_eq!(block.transactions.len(), 2);
    }

    #[test]
    fn block_validation_works() {
        let builder = BlockBuilder::new();
        let block = builder.build_block(1, [0u8; 32], vec![]).unwrap();
        
        assert!(builder.validate_block(&block).is_ok());
    }

    #[test]
    fn missing_timestamp_fails() {
        let mut inherents = InherentData::new();
        let provider = TimestampInherentDataProvider::new();
        
        assert_eq!(provider.check_inherent(&inherents), Err("Missing timestamp inherent"));
    }
}
```

### Expected Output

A functional inherent data system that:
- Compiles without errors
- Passes all unit tests
- Demonstrates timestamp inherent data handling with serde/bincode
- Shows block construction with inherents
- Implements proper validation

### Theoretical Context

**Inherents in Substrate:** Inherent data is mandatory information that must be included in every block, such as timestamps, block authors, and other consensus-related data. Unlike transactions, inherents are not signed and are automatically included by block authors.

**Serialization in Substrate:** Substrate uses efficient binary serialization for all data structures. The `serde` and `bincode` dependencies are essential for understanding how data is encoded and decoded in the blockchain.

**Key Benefits:**
- Ensures critical data is always present
- Prevents spam through automatic inclusion
- Enables consensus mechanisms
- Provides block metadata

This simplified version focuses on the core concept of inherents using timestamps as the primary example, while maintaining the essential external dependencies for proper serialization. 
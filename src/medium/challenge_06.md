# Challenge 6: SCALE Codec Basics (Simplified)

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** SCALE Codec, Serialization, Basic Encoding/Decoding

## Learning Objectives

By completing this challenge, you will understand:
- Basic SCALE codec principles
- Manual implementation of Encode/Decode traits
- Little-endian byte encoding
- Simple error handling in codecs

## Background

SCALE codec is Substrate's serialization format. This simplified version focuses on the core concepts without complex enums or detailed error handling.

## Challenge

Implement basic SCALE encoding/decoding for simple types.

### Structures to Implement

#### **Codec Traits:**
```rust
trait Encode {
    fn encode(&self) -> Vec<u8>;
}

trait Decode: Sized {
    fn decode(input: &[u8]) -> Result<Self, String>;
}
```

#### **Account Structure:**
```rust
#[derive(Debug, PartialEq)]
struct Account {
    id: u32,
    balance: u64,
}
```

### Provided Implementations

#### **Basic Types:**
```rust
impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u32 {
    fn decode(input: &[u8]) -> Result<Self, String> {
        if input.len() < 4 {
            return Err("Not enough data for u32".to_string());
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&input[..4]);
        Ok(u32::from_le_bytes(bytes))
    }
}

impl Encode for u64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u64 {
    fn decode(input: &[u8]) -> Result<Self, String> {
        if input.len() < 8 {
            return Err("Not enough data for u64".to_string());
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[..8]);
        Ok(u64::from_le_bytes(bytes))
    }
}
```

### Your Implementation

#### **Account Encoding:**
```rust
impl Encode for Account {
    fn encode(&self) -> Vec<u8> {
        // TODO: Implement
        // 1. Create empty result vector
        // 2. Extend with id.encode()
        // 3. Extend with balance.encode()
        // 4. Return result
        todo!()
    }
}
```

#### **Account Decoding:**
```rust
impl Decode for Account {
    fn decode(input: &[u8]) -> Result<Self, String> {
        // TODO: Implement
        // 1. Check if input has at least 12 bytes (4 + 8)
        // 2. Decode id from first 4 bytes
        // 3. Decode balance from next 8 bytes
        // 4. Return Ok(Account { id, balance })
        todo!()
    }
}
```

### Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_encoding() {
        let account = Account { id: 42, balance: 1000 };
        let encoded = account.encode();
        
        // Should be 12 bytes: 4 for id + 8 for balance
        assert_eq!(encoded.len(), 12);
        
        // Decode and verify
        let decoded = Account::decode(&encoded).unwrap();
        assert_eq!(decoded, account);
    }

    #[test]
    fn test_basic_types() {
        let value: u32 = 42;
        let encoded = value.encode();
        let decoded = u32::decode(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_insufficient_data() {
        let result = Account::decode(&[1, 2, 3]); // Only 3 bytes
        assert!(result.is_err());
    }
}
```

## Key Learning Points

- **Little-endian encoding**: Bytes are stored in little-endian order
- **Concatenation**: Complex types are encoded by concatenating fields
- **No metadata**: SCALE doesn't include type information in the data
- **Deterministic**: Same input always produces same output

## Expected Behavior

```rust
fn main() {
    let account = Account { id: 123, balance: 50000 };

    // Encode
    let encoded = account.encode();
    println!("Encoded: {:?}", encoded); // [123, 0, 0, 0, 80, 195, 0, 0, 0, 0, 0, 0]
    
    // Decode
    let decoded = Account::decode(&encoded).unwrap();
    println!("Decoded: {:?}", decoded); // Account { id: 123, balance: 50000 }
    
    assert_eq!(account, decoded);
}
```

This simplified version covers the essential SCALE codec concepts used throughout Substrate! 
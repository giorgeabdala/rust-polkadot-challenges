# Challenge 6: SCALE Codec Basics

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** SCALE Codec, Serialization, Deserialization, Custom Encoding

## Learning Objectives

By completing this challenge, you will understand:
- SCALE (Simple Concatenated Aggregate Little-Endian) codec principles
- Manual implementation of encoding/decoding
- Custom serialization for simple types
- Error handling in codec operations

## Background

SCALE codec is Substrate's serialization format, designed for:
- **Efficiency**: Minimal overhead, no metadata in encoded data
- **Determinism**: Same input always produces same output
- **Simplicity**: Easy to implement across languages
- **Compactness**: Optimized for blockchain storage

SCALE is used throughout Substrate for storage, extrinsics, and runtime communication.

## Challenge

Create a simplified serialization system that demonstrates basic SCALE codec principles.

### Structures to Implement

#### **Basic Codec Traits:**
```rust
trait Encode {
    fn encode(&self) -> Vec<u8>;
}

trait Decode: Sized {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError>;
}

#[derive(Debug, PartialEq)]
enum CodecError {
    NotEnoughData,
    InvalidData(String),
}
```

#### **Basic Data Structures:**
```rust
#[derive(Debug, PartialEq)]
struct Account {
    id: u32,
    balance: u64,
    is_active: bool,
}

#[derive(Debug, PartialEq)]
enum TransactionType {
    Transfer { to: u32, amount: u64 },
    Stake { amount: u64 },
    Vote { proposal_id: u32 },
}
```

### Provided Implementations

#### **Basic Type Encodings:**
```rust
impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u32 {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.len() < 4 {
            return Err(CodecError::NotEnoughData);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&input[..4]);
        *input = &input[4..];
        Ok(u32::from_le_bytes(bytes))
    }
}

impl Encode for u64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u64 {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.len() < 8 {
            return Err(CodecError::NotEnoughData);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[..8]);
        *input = &input[8..];
        Ok(u64::from_le_bytes(bytes))
    }
}

impl Encode for bool {
    fn encode(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
}

impl Decode for bool {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.is_empty() {
            return Err(CodecError::NotEnoughData);
        }
        let value = input[0];
        *input = &input[1..];
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(CodecError::InvalidData("Invalid bool value".to_string())),
        }
    }
}
```

### Methods for You to Implement

#### **1. Account Encoding (`Encode for Account`):**
```rust
impl Encode for Account {
    // TODO: Implement this method
    fn encode(&self) -> Vec<u8> {
        // IMPLEMENT:
        // 1. Create empty result vector
        // 2. Extend with id.encode()
        // 3. Extend with balance.encode()
        // 4. Extend with is_active.encode()
        // 5. Return result
        todo!()
    }
}
```

#### **2. Account Decoding (`Decode for Account`):**
```rust
impl Decode for Account {
    // TODO: Implement this method
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        // IMPLEMENT:
        // 1. Decode id using u32::decode(input)?
        // 2. Decode balance using u64::decode(input)?
        // 3. Decode is_active using bool::decode(input)?
        // 4. Return Ok(Account { id, balance, is_active })
        todo!()
    }
}
```

#### **3. Transaction Type Encoding (`Encode for TransactionType`):**
```rust
impl Encode for TransactionType {
    // TODO: Implement this method
    fn encode(&self) -> Vec<u8> {
        // IMPLEMENT:
        // Use discriminant + data pattern:
        // 1. Transfer: discriminant 0 + to + amount
        // 2. Stake: discriminant 1 + amount  
        // 3. Vote: discriminant 2 + proposal_id
        // Remember to encode discriminant as u8
        todo!()
    }
}
```

#### **4. Transaction Type Decoding (`Decode for TransactionType`):**
```rust
impl Decode for TransactionType {
    // TODO: Implement this method
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        // IMPLEMENT:
        // 1. Check if input has at least 1 byte for discriminant
        // 2. Read discriminant and advance input
        // 3. Match discriminant:
        //    - 0: decode to + amount for Transfer
        //    - 1: decode amount for Stake  
        //    - 2: decode proposal_id for Vote
        //    - other: return InvalidData error
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
    fn test_basic_types() {
        // TODO: Implement this test
        // Test u32, u64, bool round-trip encoding/decoding
        todo!()
    }

    #[test]
    fn test_account_codec() {
        // TODO: Implement this test
        // 1. Create test account
        // 2. Encode it
        // 3. Decode it back
        // 4. Assert they're equal
        todo!()
    }

    #[test]
    fn test_transaction_type_codec() {
        // TODO: Implement this test
        // Test each TransactionType variant round-trip
        todo!()
    }

    #[test]
    fn test_decode_errors() {
        // TODO: Implement this test
        // Test NotEnoughData and InvalidData errors
        todo!()
    }
}
```

### Example Usage

```rust
fn main() {
    // Basic type encoding
    let value: u32 = 42;
    let encoded = value.encode();
    let mut input = encoded.as_slice();
    let decoded = u32::decode(&mut input).unwrap();
    assert_eq!(value, decoded);

    // Custom struct encoding
    let account = Account {
        id: 123,
        balance: 1000000,
        is_active: true,
    };

    let encoded = account.encode();
    let mut input = encoded.as_slice();
    let decoded = Account::decode(&mut input).unwrap();
    assert_eq!(account, decoded);

    println!("Account encoding successful!");
}
```

### Expected Output

A basic SCALE codec system that:
- Demonstrates fundamental encoding/decoding patterns
- Handles simple data types (u32, u64, bool)
- Implements custom serialization for structs and enums
- Provides proper error handling for malformed data
- Shows round-trip encoding correctness

### Theoretical Context

**SCALE Codec Principles:**
- **Little-Endian**: All multi-byte integers use little-endian byte order
- **Concatenation**: Struct fields are simply concatenated
- **Discriminants**: Enums use byte discriminants followed by variant data
- **No Metadata**: Encoded data contains no type information
- **Deterministic**: Same input always produces identical output

**Key Patterns:**
1. **Struct Encoding**: Concatenate all field encodings
2. **Enum Encoding**: Discriminant byte + variant data
3. **Error Handling**: Graceful handling of insufficient/invalid data
4. **Round-trip Testing**: Ensure encode(decode(x)) == x

**Substrate Connection:**
- Storage items are SCALE-encoded
- Extrinsic parameters use SCALE
- Runtime API calls encode/decode with SCALE
- Cross-chain messages use SCALE format

This challenge teaches essential SCALE concepts needed for Substrate development. 
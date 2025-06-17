# Challenge 6: SCALE Codec and Serialization

**Estimated Time:** 35 minutes  
**Difficulty:** Medium  
**Topics:** SCALE Codec, Serialization, Deserialization, Custom Encoding

## Learning Objectives

By completing this challenge, you will understand:
- SCALE (Simple Concatenated Aggregate Little-Endian) codec principles
- Manual implementation of encoding/decoding
- Compact encoding for integers
- Custom serialization for complex types
- Error handling in codec operations

## Background

SCALE codec is Substrate's serialization format, designed for:
- **Efficiency**: Minimal overhead, no metadata in encoded data
- **Determinism**: Same input always produces same output
- **Simplicity**: Easy to implement across languages
- **Compactness**: Optimized for blockchain storage

SCALE is used throughout Substrate for storage, extrinsics, and runtime communication.

## Challenge

Create a custom serialization system that demonstrates SCALE codec principles.

### Requirements

1. **Create basic codec traits:**
   ```rust
   trait Encode {
       fn encode(&self) -> Vec<u8>;
       fn size_hint(&self) -> usize {
           0
       }
   }

   trait Decode: Sized {
       fn decode(input: &mut &[u8]) -> Result<Self, CodecError>;
   }

   #[derive(Debug, PartialEq)]
   enum CodecError {
       NotEnoughData,
       InvalidData(String),
       UnexpectedEnd,
   }
   ```

2. **Implement basic type encodings:**
   - `u8`, `u16`, `u32`, `u64` (little-endian)
   - `bool` (0x00 for false, 0x01 for true)
   - `Option<T>` (0x00 for None, 0x01 + encoded T for Some)
   - `Vec<T>` (compact length + encoded elements)
   - `String` (as Vec<u8> of UTF-8 bytes)

3. **Create a `Compact<T>` wrapper for efficient integer encoding:**
   ```rust
   struct Compact<T>(pub T);
   
   // Compact encoding rules:
   // 0b00 prefix: single byte mode (0-63)
   // 0b01 prefix: two-byte mode (64-16383)
   // 0b10 prefix: four-byte mode (16384-1073741823)
   // 0b11 prefix: big-integer mode (1073741824+)
   ```

4. **Create custom data structures:**
   ```rust
   #[derive(Debug, PartialEq)]
   struct Account {
       id: u32,
       balance: u64,
       nonce: u32,
       is_active: bool,
   }

   #[derive(Debug, PartialEq)]
   enum TransactionType {
       Transfer { to: u32, amount: u64 },
       Stake { amount: u64 },
       Unstake,
       Vote { proposal_id: u32, approve: bool },
   }

   #[derive(Debug, PartialEq)]
   struct Transaction {
       from: u32,
       tx_type: TransactionType,
       nonce: Compact<u32>,
       signature: Vec<u8>,
   }
   ```

5. **Implement codec for custom types:**
   - Manual `Encode` and `Decode` implementations
   - Proper error handling for malformed data
   - Round-trip testing (encode then decode should equal original)

### Expected Behavior

```rust
// Basic type encoding
let value: u32 = 42;
let encoded = value.encode();
assert_eq!(encoded, vec![42, 0, 0, 0]); // little-endian

// Compact encoding
let compact = Compact(300u32);
let encoded = compact.encode();
// Should use 2-byte mode: 0b01 prefix + 300 in little-endian

// Custom struct encoding
let account = Account {
    id: 123,
    balance: 1000000,
    nonce: 5,
    is_active: true,
};

let encoded = account.encode();
let mut input = encoded.as_slice();
let decoded = Account::decode(&mut input).unwrap();
assert_eq!(account, decoded);

// Complex enum encoding
let tx = Transaction {
    from: 456,
    tx_type: TransactionType::Transfer { to: 789, amount: 500 },
    nonce: Compact(10),
    signature: vec![1, 2, 3, 4],
};

let encoded = tx.encode();
let mut input = encoded.as_slice();
let decoded = Transaction::decode(&mut input).unwrap();
assert_eq!(tx, decoded);
```

## Advanced Requirements

1. **Implement a generic `Codec` derive-like functionality:**
   ```rust
   fn derive_encode_for_struct(fields: &[(&str, Box<dyn Encode>)]) -> Vec<u8> {
       // Generic struct encoding
   }
   ```

2. **Create a `EncodeAppend` trait for efficient appending:**
   ```rust
   trait EncodeAppend<T> {
       fn encode_append(&mut self, item: &T);
   }
   ```

3. **Implement versioned encoding:**
   ```rust
   #[derive(Debug, PartialEq)]
   struct VersionedAccount {
       version: u8,
       data: AccountData,
   }
   
   enum AccountData {
       V1(AccountV1),
       V2(AccountV2),
   }
   ```

## Testing

Write comprehensive tests that demonstrate:
- Round-trip encoding/decoding for all types
- Compact integer encoding efficiency
- Error handling for malformed data
- Enum variant encoding/decoding
- Complex nested structure handling

```rust
#[test]
fn test_compact_encoding() {
    // Test different compact encoding modes
    assert_eq!(Compact(0u32).encode(), vec![0x00]);
    assert_eq!(Compact(63u32).encode(), vec![0xFC]);
    assert_eq!(Compact(64u32).encode(), vec![0x01, 0x01]);
    assert_eq!(Compact(16383u32).encode(), vec![0xFD, 0xFF]);
}

#[test]
fn test_enum_encoding() {
    let transfer = TransactionType::Transfer { to: 100, amount: 200 };
    let encoded = transfer.encode();
    
    let mut input = encoded.as_slice();
    let decoded = TransactionType::decode(&mut input).unwrap();
    assert_eq!(transfer, decoded);
}
```

## Codec Implementation Patterns

1. **Struct Encoding (concatenation):**
   ```rust
   impl Encode for Account {
       fn encode(&self) -> Vec<u8> {
           let mut result = Vec::new();
           result.extend(self.id.encode());
           result.extend(self.balance.encode());
           result.extend(self.nonce.encode());
           result.extend(self.is_active.encode());
           result
       }
   }
   ```

2. **Enum Encoding (discriminant + data):**
   ```rust
   impl Encode for TransactionType {
       fn encode(&self) -> Vec<u8> {
           match self {
               TransactionType::Transfer { to, amount } => {
                   let mut result = vec![0]; // discriminant
                   result.extend(to.encode());
                   result.extend(amount.encode());
                   result
               }
               // ... other variants
           }
       }
   }
   ```

3. **Safe Decoding:**
   ```rust
   impl Decode for Account {
       fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
           let id = u32::decode(input)?;
           let balance = u64::decode(input)?;
           let nonce = u32::decode(input)?;
           let is_active = bool::decode(input)?;
           
           Ok(Account { id, balance, nonce, is_active })
       }
   }
   ```

## Tips

- Always use little-endian byte order
- Handle partial reads gracefully
- Use compact encoding for frequently used integers
- Test with edge cases (empty data, maximum values)
- Consider backwards compatibility for versioned data

## Key Learning Points

- **SCALE Principles**: Efficiency, determinism, simplicity
- **Compact Encoding**: Space-efficient integer representation
- **Error Handling**: Robust decoding with proper error types
- **Custom Serialization**: Implementing codec for complex types
- **Testing Strategy**: Ensuring round-trip correctness

## Substrate Connection

SCALE codec in Substrate:
- `parity-scale-codec` crate provides derive macros
- Storage items are SCALE-encoded
- Extrinsics use SCALE for parameters
- Runtime API calls encode/decode with SCALE
- Cross-chain messages (XCM) use SCALE

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Custom derive macros** - Understand procedural macros used in Substrate
2. **Performance-critical encoding** - Optimize for blockchain storage efficiency 
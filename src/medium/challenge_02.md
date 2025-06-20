# Challenge 2: Generics and Traits

**Estimated Time:** 60 minutes  
**Difficulty:** Medium  
**Topics:** Generic Types, Trait Bounds, Associated Types, Where Clauses

## Learning Objectives

By completing this challenge, you will understand:
- Generic type parameters and constraints
- Trait bounds and where clauses
- Associated types vs generic parameters
- Default trait implementations
- Generic collections and abstractions

## Background

Generics and traits form the backbone of Rust's type system, enabling powerful abstractions without runtime cost. Understanding these concepts is crucial for Substrate development, where you'll work with generic pallets, associated types in Config traits, and complex trait bounds.

## Challenge

Create a generic storage system that demonstrates key generic programming patterns used throughout the Rust ecosystem and Substrate.

### Requirements

1. **Create a `Storable` trait** with:
   - `fn to_bytes(&self) -> Vec<u8>`
   - `fn from_bytes(data: &[u8]) -> Result<Self, String>` where `Self: Sized`
   - Default implementation for `fn storage_key(&self) -> String { "default".to_string() }`

2. **Create a generic `Storage<T>` struct** where `T: Storable + Clone`:
   - `items: Vec<T>`
   - `capacity: usize`

3. **Implement methods for `Storage<T>`:**
   - `new(capacity: usize) -> Self`
   - `store(&mut self, item: T) -> Result<usize, String>` (returns index)
   - `retrieve(&self, index: usize) -> Option<&T>`
   - `len(&self) -> usize`
   - `is_full(&self) -> bool`

4. **Create a `StorageMap` trait with associated types:**
   ```rust
   trait StorageMap {
       type Item: Storable;
       type Key;
       
       fn get(&self, key: &Self::Key) -> Option<&Self::Item>;
       fn insert(&mut self, key: Self::Key, value: Self::Item) -> Option<Self::Item>;
   }
   ```

5. **Create a `KeyValueStorage<K, V>` that implements `StorageMap`:**
   ```rust
   use std::collections::HashMap;
   
   struct KeyValueStorage<K, V> {
       data: HashMap<K, V>,
   }
   
   impl<K, V> KeyValueStorage<K, V> {
       fn new() -> Self {
           Self {
               data: HashMap::new(),
           }
       }
   }
   
   impl<K, V> StorageMap for KeyValueStorage<K, V> 
   where 
       K: Clone + std::hash::Hash + Eq,
       V: Storable + Clone,
   {
       type Key = K;
       type Item = V;
       
       fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
           self.data.get(key)
       }
       
       fn insert(&mut self, key: Self::Key, value: Self::Item) -> Option<Self::Item> {
           self.data.insert(key, value)
       }
   }
   ```

6. **Implement `Storable` for `String`** (use UTF-8 bytes)

### Expected Behavior

```rust
// Generic storage works with any Storable type
let mut storage: Storage<String> = Storage::new(10);

// Store and retrieve with type safety
let idx = storage.store("Hello".to_string())?;
let text = storage.retrieve(idx).unwrap();
assert_eq!(text, "Hello");

// StorageMap with associated types
let mut kv_storage: KeyValueStorage<u32, String> = KeyValueStorage::new();
kv_storage.insert(1, "World".to_string());
let value = kv_storage.get(&1).unwrap();
```

## Testing

Write tests that demonstrate:
- Generic storage with String type
- Storage capacity limits
- Default trait implementations
- Associated types in StorageMap
- KeyValue storage operations
- Trait bounds on generic structs


## Tips

- Use `#[derive(Clone)]` for simple types
- For `String` implementation: `self.as_bytes().to_vec()` and `String::from_utf8(data.to_vec())`
- Remember that associated types define a "one-to-one" relationship
- For `KeyValueStorage`, implement `StorageMap` with appropriate associated types
- Associated types in traits create a strong relationship between the trait and implementing type

## Key Learning Points

- **Zero-Cost Abstractions**: Generics are resolved at compile time
- **Trait Bounds**: Constraining generic types with required behavior
- **Associated Types vs Generics**: When to use each approach
- **Where Clauses**: Managing complex trait relationships
- **Generic Collections**: Building reusable data structures

## Substrate Connection

These patterns are fundamental in Substrate:
- **Generic pallets**: Work with different `Config` trait implementations
- **Storage abstractions**: `StorageMap<K, V>`, `StorageValue<V>` use similar patterns
- **Associated types**: Config traits use them extensively (`type AccountId`, `type Balance`)
- **Trait bounds**: Pallets have complex where clauses constraining associated types
- **Generic collections**: Runtime storage is built on generic abstractions

Understanding these concepts prepares you for Substrate's advanced generic patterns!

---

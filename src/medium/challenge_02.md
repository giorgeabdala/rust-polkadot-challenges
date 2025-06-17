# Challenge 2: Generics and Traits

**Estimated Time:** 45 minutes  
**Difficulty:** Medium  
**Topics:** Generic Types, Trait Bounds, Associated Types, Default Implementations

## Learning Objectives

By completing this challenge, you will understand:
- Generic type parameters and constraints
- Trait bounds and where clauses
- Associated types vs generic parameters
- Default trait implementations
- Generic trait objects

## Background

Generics and traits are Rust's primary abstraction mechanisms. They enable:
- **Code reuse** without runtime cost (zero-cost abstractions)
- **Type safety** with compile-time guarantees
- **Flexible APIs** that work with multiple types
- **Trait bounds** to constrain generic behavior

This is essential for Substrate development where generic pallets work with different runtime configurations.

## Challenge

Create a generic storage system that can work with different data types and serialization formats.

### Requirements

1. **Create a `Serializable` trait** with:
   - `fn serialize(&self) -> Vec<u8>`
   - `fn deserialize(data: &[u8]) -> Result<Self, String>` where `Self: Sized`
   - Default implementation for `fn size_hint(&self) -> usize { 0 }`

2. **Create a generic `Storage<T>` struct** where `T: Serializable + Clone`:
   - `items: Vec<T>`
   - `capacity: usize`

3. **Implement methods for `Storage<T>`:**
   - `new(capacity: usize) -> Self`
   - `store(&mut self, item: T) -> Result<usize, String>` (returns index)
   - `retrieve(&self, index: usize) -> Option<&T>`
   - `update(&mut self, index: usize, item: T) -> Result<(), String>`
   - `remove(&mut self, index: usize) -> Option<T>`
   - `len(&self) -> usize`
   - `is_full(&self) -> bool`

4. **Create a `Cacheable` trait** that extends `Serializable`:
   - `fn cache_key(&self) -> String`
   - `fn is_expired(&self) -> bool`

5. **Implement `Serializable` for basic types:**
   - `String`
   - `u32`
   - A custom `User` struct

### Expected Behavior

```rust
// Generic storage works with any Serializable type
let mut string_storage: Storage<String> = Storage::new(10);
let mut number_storage: Storage<u32> = Storage::new(5);

// Store different types
let idx1 = string_storage.store("Hello".to_string())?;
let idx2 = number_storage.store(42)?;

// Retrieve with type safety
let text = string_storage.retrieve(idx1).unwrap();
let number = number_storage.retrieve(idx2).unwrap();

// Works with custom types
let mut user_storage: Storage<User> = Storage::new(100);
let user = User { id: 1, name: "Alice".to_string() };
let user_idx = user_storage.store(user)?;
```

## Testing

Write tests that demonstrate:
- Generic storage with different types
- Trait bound enforcement at compile time
- Serialization/deserialization round trips
- Storage capacity limits
- Default trait implementations

## Advanced Requirements

1. **Create a `CachedStorage<T>` struct** where `T: Cacheable`:
   - Extends basic storage with caching logic
   - Automatically removes expired items
   - Implements cache key-based lookup

2. **Add trait bounds using `where` clauses:**
   ```rust
   impl<T> Storage<T> 
   where 
       T: Serializable + Clone + PartialEq,
   {
       fn find(&self, item: &T) -> Option<usize> {
           // Implementation
       }
   }
   ```

## Tips

- Use `#[derive(Clone)]` for simple types
- Implement `Serializable` using `serde` concepts (but manually)
- Use `Box<dyn Trait>` for trait objects when needed
- Remember that generic functions are monomorphized at compile time

## Key Learning Points

- **Zero-Cost Abstractions**: Generics have no runtime overhead
- **Trait Bounds**: Constraining generic types with required behavior
- **Associated Types**: When to use them vs generic parameters
- **Coherence Rules**: Why Rust's trait system prevents conflicts

## Substrate Connection

This mirrors Substrate's architecture:
- `Config` trait with associated types for runtime configuration
- Generic pallets that work with different `Config` implementations
- Storage traits like `StorageMap<K, V>` and `StorageValue<V>`
- Codec trait for serialization (similar to our `Serializable`)

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Generic error types with associated type bounds** - Practice complex trait relationships  
2. **Trait object patterns** - Use `Box<dyn Trait>` for dynamic dispatch similar to Substrate's approach

---

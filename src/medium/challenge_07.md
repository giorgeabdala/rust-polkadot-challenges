# Challenge 7: Macros and Metaprogramming

**Estimated Time:** 60 minutes  
**Difficulty:** Medium  
**Topics:** Declarative Macros, Procedural Macros, Code Generation, Macro Patterns, Performance Benchmarking

## Learning Objectives

By completing this challenge, you will understand:
- Declarative macros with `macro_rules!`
- Pattern matching in macros
- Repetition and recursion in macros
- Procedural macro concepts
- Code generation and metaprogramming patterns

## Background

Macros enable code generation at compile time, providing:
- **Zero-cost abstractions**: Generated code has no runtime overhead
- **DRY principle**: Reduce code duplication
- **Domain-specific languages**: Create custom syntax
- **Compile-time computation**: Generate code based on input

Substrate uses macros extensively for pallets, storage, and runtime configuration.

## Challenge

Create a macro system for generating blockchain storage and event types.

### Requirements

1. **Create a `storage!` declarative macro** that generates storage structs:
   ```rust
   macro_rules! storage {
       (
           $(#[$meta:meta])*
           $vis:vis struct $name:ident {
               $(
                   $(#[$field_meta:meta])*
                   $field_name:ident: $field_type:ty = $default:expr,
               )*
           }
       ) => {
           // Generate storage struct with getters/setters
       };
   }
   ```

2. **Create an `event!` macro** for generating event enums:
   ```rust
   macro_rules! event {
       (
           $(#[$meta:meta])*
           $vis:vis enum $name:ident {
               $(
                   $(#[$variant_meta:meta])*
                   $variant:ident $({ $($field:ident: $field_type:ty),* })?,
               )*
           }
       ) => {
           // Generate event enum with emit functionality
       };
   }
   ```

3. **Create a `pallet!` macro** that combines storage and events:
   ```rust
   macro_rules! pallet {
       (
           name: $pallet_name:ident,
           storage: {
               $($storage_items:tt)*
           },
           events: {
               $($event_items:tt)*
           },
           calls: {
               $($call_items:tt)*
           }
       ) => {
           // Generate complete pallet structure
       };
   }
   ```

4. **Create a simple `benchmark!` macro** for performance testing:
   ```rust
   macro_rules! benchmark {
       (
           $name:ident: $setup:block => $code:block
       ) => {
           fn $name() {
               $setup
               let start = std::time::Instant::now();
               $code
               let duration = start.elapsed();
               println!("{}: {:?}", stringify!($name), duration);
           }
       };
   }
   ```

5. **Create utility macros:**
   - `impl_getter!` for generating getter methods
   - `impl_setter!` for generating setter methods
   - `count_items!` for counting macro arguments
   - `generate_id!` for creating unique identifiers

### Expected Behavior

```rust
// Storage generation
storage! {
    /// Account storage for the pallet
    pub struct AccountStorage {
        /// Account balances
        balances: HashMap<u32, u64> = HashMap::new(),
        /// Total supply
        total_supply: u64 = 0,
        /// Next account ID
        next_id: u32 = 1,
    }
}

// Event generation
event! {
    /// Events emitted by the pallet
    pub enum PalletEvent {
        /// Account created
        AccountCreated { id: u32, initial_balance: u64 },
        /// Transfer completed
        Transfer { from: u32, to: u32, amount: u64 },
        /// Account balance updated
        BalanceUpdated { id: u32, new_balance: u64 },
    }
}

// Usage
let mut storage = AccountStorage::new();
storage.set_total_supply(1000000);
assert_eq!(storage.get_total_supply(), 1000000);

let event = PalletEvent::AccountCreated { id: 1, initial_balance: 100 };
event.emit();
```

## Advanced Requirements

1. **Create a `derive_codec!` macro** for automatic serialization:
   ```rust
   macro_rules! derive_codec {
       ($($type:ty),*) => {
           $(
               impl Encode for $type {
                   // Generated encode implementation
               }
               
               impl Decode for $type {
                   // Generated decode implementation
               }
           )*
       };
   }
   ```

2. **Create recursive macros:**
   ```rust
   macro_rules! count_tts {
       () => { 0 };
       ($head:tt $($tail:tt)*) => { 1 + count_tts!($($tail)*) };
   }
   
   macro_rules! reverse_list {
       ([] $($reversed:tt)*) => { ($($reversed)*) };
       ([$head:tt $($tail:tt)*] $($reversed:tt)*) => {
           reverse_list!([$($tail)*] $head $($reversed)*)
       };
   }
   ```

## Testing

Write tests that demonstrate:
- Macro expansion with different inputs
- Generated code functionality
- Error handling in macros
- Recursive macro behavior
- Integration between different macros

```rust
#[test]
fn test_storage_macro() {
    storage! {
        pub struct TestStorage {
            value: u32 = 42,
            name: String = "test".to_string(),
        }
    }
    
    let mut storage = TestStorage::new();
    assert_eq!(storage.get_value(), 42);
    
    storage.set_value(100);
    assert_eq!(storage.get_value(), 100);
}

#[test]
fn test_event_macro() {
    event! {
        pub enum TestEvent {
            Simple,
            WithData { value: u32 },
        }
    }
    
    let event = TestEvent::WithData { value: 123 };
    // Test event emission
}
```

## Macro Patterns

1. **Pattern Matching:**
   ```rust
   macro_rules! match_type {
       (u32) => { "unsigned 32-bit integer" };
       (String) => { "string type" };
       ($other:ty) => { "other type" };
   }
   ```

2. **Repetition:**
   ```rust
   macro_rules! create_functions {
       ($($name:ident),*) => {
           $(
               fn $name() {
                   println!("Function {}", stringify!($name));
               }
           )*
       };
   }
   ```

3. **Conditional Generation:**
   ```rust
   macro_rules! maybe_impl {
       ($type:ty, true) => {
           impl Display for $type {
               fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                   write!(f, "{:?}", self)
               }
           }
       };
       ($type:ty, false) => {};
   }
   ```

## Tips

- Use `cargo expand` to see macro expansions
- Start with simple patterns and build complexity
- Use `$crate::` for hygiene in library macros
- Test macros with various input patterns
- Consider error messages for invalid macro usage

## Key Learning Points

- **Macro Syntax**: Understanding token trees and patterns
- **Code Generation**: Creating repetitive code programmatically
- **Hygiene**: Avoiding variable name conflicts
- **Debugging**: Techniques for debugging macro expansions
- **Performance**: Compile-time vs runtime trade-offs

## Substrate Connection

Substrate's macro usage:
- `#[pallet::pallet]` for pallet definition
- `#[pallet::storage]` for storage items
- `#[pallet::event]` for event definitions
- `#[pallet::call]` for dispatchable functions
- `construct_runtime!` for runtime composition

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Procedural macro concepts** - Understand how Substrate's derive macros work
2. **Advanced macro patterns** - Practice complex token manipulation for runtime generation

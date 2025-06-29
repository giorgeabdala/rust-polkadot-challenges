# Challenge 7: Basic Macros

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Declarative Macros, Pattern Matching, Code Generation

## Learning Objectives

By completing this challenge, you will understand:
- Declarative macros with `macro_rules!`
- Basic pattern matching in macros
- Simple code generation patterns
- Macro hygiene and debugging

## Background

Macros enable code generation at compile time, providing:
- **Zero-cost abstractions**: Generated code has no runtime overhead
- **DRY principle**: Reduce code duplication  
- **Code generation**: Create repetitive code programmatically

Substrate uses macros extensively for pallets and runtime configuration.

## Challenge

Create basic macros for generating simple blockchain structures.

### Structures to Implement

#### **Basic Data Types:**
```rust
#[derive(Debug, Clone, PartialEq)]
struct Account {
    id: u32,
    balance: u64,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
enum Event {
    AccountCreated { id: u32, name: String },
    Transfer { from: u32, to: u32, amount: u64 },
    BalanceUpdated { id: u32, new_balance: u64 },
}
```

### Methods for You to Implement

#### **1. Create Simple Storage Macro (`create_getter`):**
```rust
// TODO: Implement this macro
macro_rules! create_getter {
    ($struct_name:ident, $field:ident, $field_type:ty) => {
        // IMPLEMENT:
        // Generate a getter method for the specified field
        // Should create: fn get_$field(&self) -> &$field_type
        todo!()
    };
}
```

#### **2. Create Event Builder Macro (`event`):**
```rust
// TODO: Implement this macro  
macro_rules! event {
    ($variant:ident { $($field:ident: $value:expr),* }) => {
        // IMPLEMENT:
        // Generate Event::$variant { $field: $value, ... }
        // Should create the enum variant with provided fields
        todo!()
    };
}
```

#### **3. Create Function Generator Macro (`create_functions`):**
```rust
// TODO: Implement this macro
macro_rules! create_functions {
    ($($name:ident),*) => {
        // IMPLEMENT:
        // For each name, generate:
        // fn $name() { println!("Function {}", stringify!($name)); }
        todo!()
    };
}
```

#### **4. Create Impl Block Macro (`impl_default`):**
```rust
// TODO: Implement this macro
macro_rules! impl_default {
    ($struct_name:ident { $($field:ident: $default_value:expr),* }) => {
        // IMPLEMENT:
        // Generate Default implementation for struct:
        // impl Default for $struct_name {
        //     fn default() -> Self {
        //         Self { $field: $default_value, ... }
        //     }
        // }
        todo!()
    };
}
```

### Expected Usage

```rust
// Test the create_getter macro
impl Account {
    create_getter!(Account, id, u32);
    create_getter!(Account, balance, u64);
    create_getter!(Account, name, String);
}

// Test the event macro
let transfer_event = event!(Transfer { from: 1, to: 2, amount: 100 });
let account_event = event!(AccountCreated { id: 1, name: "Alice".to_string() });

// Test function creation
create_functions!(process_transfer, validate_account, update_balance);

// Test default implementation
impl_default!(Account { 
    id: 0, 
    balance: 0, 
    name: "".to_string() 
});

let default_account = Account::default();
```

### Tests to Implement

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getter_macro() {
        // TODO: Implement this test
        // 1. Create account with test data
        // 2. Use generated getter methods
        // 3. Verify they return correct values
        todo!()
    }

    #[test] 
    fn test_event_macro() {
        // TODO: Implement this test
        // 1. Create events using event! macro
        // 2. Verify they match expected Event enum variants
        todo!()
    }

    #[test]
    fn test_function_generation() {
        // TODO: Implement this test
        // 1. Call generated functions
        // 2. Verify they exist and can be called
        todo!()
    }

    #[test]
    fn test_default_impl() {
        // TODO: Implement this test
        // 1. Create default account
        // 2. Verify default values are correct
        todo!()
    }
}
```

### Macro Patterns to Learn

#### **1. Pattern Matching:**
```rust
macro_rules! match_type {
    (u32) => { "unsigned 32-bit integer" };
    (String) => { "string type" };
    ($other:ty) => { "other type" };
}
```

#### **2. Repetition:**
```rust
macro_rules! print_all {
    ($($item:expr),*) => {
        $(
            println!("{}", $item);
        )*
    };
}
```

#### **3. Token Tree Basics:**
```rust
macro_rules! example {
    // Match different token patterns
    (struct $name:ident) => { /* struct pattern */ };
    ($expr:expr) => { /* expression pattern */ };
    ($($tt:tt)*) => { /* catch-all pattern */ };
}
```

### Example Usage

```rust
fn main() {
    // Create account using default
    let mut account = Account::default();
    println!("Default account: {:?}", account);
    
    // Use getters (generated by macro)
    println!("Account ID: {}", account.get_id());
    println!("Account balance: {}", account.get_balance());
    
    // Create events using macro
    let event1 = event!(AccountCreated { id: 1, name: "Alice".to_string() });
    let event2 = event!(Transfer { from: 1, to: 2, amount: 100 });
    
    println!("Event 1: {:?}", event1);
    println!("Event 2: {:?}", event2);
    
    // Call generated functions
    process_transfer();
    validate_account();
    update_balance();
}
```

### Expected Output

A basic macro system that:
- Generates getter methods for struct fields
- Creates event instances with clean syntax
- Generates multiple functions from a list
- Implements Default trait automatically
- Demonstrates fundamental macro patterns

### Theoretical Context

**Macro Fundamentals:**
- **Declarative Macros**: Pattern-based code generation with `macro_rules!`
- **Token Trees**: Basic units of Rust syntax that macros manipulate
- **Hygiene**: Macros avoid variable name conflicts automatically
- **Expansion**: Macros are expanded before compilation

**Key Patterns:**
1. **Repetition**: `$(...)*` and `$(...),*` for handling lists
2. **Substitution**: `$name:ident` for identifiers, `$expr:expr` for expressions  
3. **Code Generation**: Creating structs, functions, and implementations
4. **Pattern Matching**: Different macro arms for different input patterns

**Substrate Connection:**
- `#[pallet::storage]` generates storage items
- `#[pallet::event]` generates event enums
- `#[pallet::call]` generates dispatchable functions
- `construct_runtime!` composes pallets into a runtime

This challenge teaches essential macro concepts needed for understanding Substrate's code generation patterns.

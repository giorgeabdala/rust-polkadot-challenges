# Challenge 3: Lifetimes and References

**Estimated Time:** 50 minutes  
**Difficulty:** Medium  
**Topics:** Lifetime Parameters, Borrowing Rules, Lifetime Elision, Static Lifetimes

## Learning Objectives

By completing this challenge, you will understand:
- Explicit lifetime annotations and their necessity
- Relationship between lifetimes and borrowing
- Lifetime elision rules and when they apply
- Static lifetimes and their use cases
- Common lifetime patterns in real applications

## Background

Lifetimes are Rust's secret weapon for memory safety without garbage collection. They track how long references are valid, preventing entire classes of bugs that plague other systems languages. Understanding lifetimes deeply is crucial for Substrate development where zero-copy operations and safe concurrency are essential.

### 🎯 **What Are Lifetimes Really?**

Lifetimes are **annotations about relationships between references**, not durations:

```rust
// This doesn't mean 'a lives longer than 'b
fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // We're saying: "output lives as long as x"
}
```

**Key Insight:** Lifetimes describe **constraints**, not **timelines**.

### 🔒 **The Borrowing Rules Foundation**

Understanding borrowing rules is essential before diving into lifetimes:

#### **Rule 1: Exclusive Access**
```rust
let mut data = vec![1, 2, 3];
let mutable_ref = &mut data;     // Exclusive mutable access
// let immutable_ref = &data;    // ❌ Cannot borrow as immutable
mutable_ref.push(4);             // ✅ Can modify through mutable ref
```

#### **Rule 2: Shared Immutable Access**
```rust
let data = vec![1, 2, 3];
let ref1 = &data;                // ✅ First immutable borrow
let ref2 = &data;                // ✅ Multiple immutable borrows OK
// let mut_ref = &mut data;      // ❌ Cannot borrow mutably while immutable refs exist
println!("{:?} {:?}", ref1, ref2);
```

#### **Rule 3: References Must Be Valid**
```rust
let reference;
{
    let value = String::from("hello");
    reference = &value;          // ❌ 'value' dropped at end of scope
}
// println!("{}", reference);   // ❌ Use of dangling reference
```

### ⏳ **Lifetime Annotations: When and Why**

Rust tries to infer lifetimes, but sometimes needs help:

#### **When Lifetime Annotations Are Required:**

1. **Multiple input references with unclear relationships:**
```rust
// Ambiguous: which input does output relate to?
fn longest(x: &str, y: &str) -> &str {  // ❌ Won't compile
    if x.len() > y.len() { x } else { y }
}

// Clear: output relates to both inputs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ✅ Works
    if x.len() > y.len() { x } else { y }
}
```

2. **Structs holding references:**
```rust
// Must specify how long the reference lives
struct TextProcessor<'a> {
    text: &'a str,    // This reference must live as long as the struct
    delimiter: char,
}
```

### 🔄 **Lifetime Elision Rules**

Rust automatically infers lifetimes in common patterns:

#### **Rule 1: Each input parameter gets its own lifetime**
```rust
// What you write:
fn first_word(s: &str) -> &str

// What Rust sees:
fn first_word<'a>(s: &'a str) -> &'a str
```

#### **Rule 2: If exactly one input lifetime, it's assigned to all outputs**
```rust
// What you write:
fn get_slice(data: &Vec<i32>, index: usize) -> &i32

// What Rust sees:
fn get_slice<'a>(data: &'a Vec<i32>, index: usize) -> &'a i32
```

#### **Rule 3: If multiple inputs but one is `&self` or `&mut self`, its lifetime is assigned to outputs**
```rust
impl<'a> Parser<'a> {
    // What you write:
    fn parse(&self, input: &str) -> &str
    
    // What Rust sees:
    fn parse(&self, input: &str) -> &str  // Output gets &self's lifetime
}
```

#### **When Elision Fails:**
```rust
// Multiple inputs, no &self - ambiguous!
fn combine(x: &str, y: &str) -> &str {  // ❌ Needs explicit lifetimes
    if x.len() > y.len() { x } else { y }
}
```

### 🏗️ **Common Lifetime Patterns**

#### **Pattern 1: Borrowing from Input**
```rust
fn extract_domain<'a>(email: &'a str) -> Option<&'a str> {
    email.split('@').nth(1)  // Returns slice of input
}
```

#### **Pattern 2: Multiple Independent Lifetimes**
```rust
fn log_comparison<'a, 'b>(
    primary: &'a str, 
    secondary: &'b str
) -> bool {
    println!("Comparing {} vs {}", primary, secondary);
    primary.len() > secondary.len()  // Return owned data, not reference
}
```

### 🌟 **Static Lifetime**

#### **String Literals:**
```rust
let s: &'static str = "Hello, world!";  // Lives for entire program
```

#### **Global Constants:**
```rust
static GLOBAL_CONFIG: &str = "production";  // 'static lifetime
```

### 💡 **Common Lifetime Issues and Solutions**

#### **1. "borrowed value does not live long enough"**
```rust
// Problem:
fn get_name() -> &str {
    let name = String::from("Alice");
    &name  // ❌ name dropped at end of function
}

// Solution: Return owned data
fn get_name() -> String {
    String::from("Alice")  // ✅ Return owned String
}
```

#### **2. "cannot borrow as mutable"**
```rust
// Problem:
let data = vec![1, 2, 3];
let shared1 = &data;
let shared2 = &data;
let mutable = &mut data;  // ❌ Cannot mutably borrow while immutably borrowed

// Solution: Ensure exclusive access
let mut data = vec![1, 2, 3];
{
    let shared1 = &data;
    let shared2 = &data;
    // shared references dropped here
}
let mutable = &mut data;  // ✅ Now we can mutably borrow
```

## Challenge

Create a simple configuration system that demonstrates practical lifetime usage.

### Basic Requirements

1. **Create a `ConfigValue` enum** that can hold different types:
   ```rust
   enum ConfigValue<'a> {
       Text(&'a str),
       Number(i64),
       Boolean(bool),
   }
   ```

2. **Create a `Config` struct** that manages configuration:
   ```rust
   struct Config<'a> {
       name: &'a str,
       values: HashMap<&'a str, ConfigValue<'a>>,
   }
   ```

3. **Implement ESSENTIAL methods for `Config<'a>`:**
   - `new(name: &'a str) -> Self`
   - `set(&mut self, key: &'a str, value: ConfigValue<'a>)`
   - `get(&self, key: &str) -> Option<&ConfigValue<'a>>`

### Usage Example

```rust
// String literals have 'static lifetime
let config_name = "database";
let mut config = Config::new(config_name);

// Setting different types of values
config.set("host", ConfigValue::Text("localhost"));
config.set("port", ConfigValue::Number(5432));
config.set("ssl", ConfigValue::Boolean(true));

// Retrieving values
if let Some(ConfigValue::Text(host)) = config.get("host") {
    println!("Database host: {}", host);
}
```

## Advanced Requirements (Optional)

If you want to practice more lifetime concepts:

1. **Function that compares lifetimes:**
   ```rust
   fn longest_config_name<'a>(c1: &'a Config, c2: &'a Config) -> &'a str {
       if c1.values.len() >= c2.values.len() {
           c1.name
       } else {
           c2.name
       }
   }
   ```

2. **Helper method for text:**
   ```rust
   impl<'a> Config<'a> {
       fn get_text(&self, key: &str) -> Option<&str> {
           match self.get(key)? {
               ConfigValue::Text(s) => Some(s),
               _ => None,
           }
       }
   }
   ```

## Testing

Write tests that demonstrate:
- Configs with string literals ('static lifetime)
- Configs with borrowed strings (explicit lifetimes)
- Correct use of references with different lifetimes

## Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_with_static_lifetimes() {
        let mut config = Config::new("test");
        config.set("app", ConfigValue::Text("MyApp"));
        config.set("version", ConfigValue::Number(1));
        
        assert_eq!(config.get_text("app"), Some("MyApp"));
    }

    #[test]
    fn test_longest_config_name() {
        let mut config1 = Config::new("short");
        config1.set("key1", ConfigValue::Boolean(true));
        
        let mut config2 = Config::new("longer_name");
        config2.set("key1", ConfigValue::Number(1));
        config2.set("key2", ConfigValue::Text("value"));
        
        assert_eq!(longest_config_name(&config1, &config2), "longer_name");
    }
}
```

## Tips

- Start with lifetime elision (let compiler infer)
- Add explicit lifetimes only when compiler requires them
- Use `'static` for string literals and global data
- Remember: lifetimes are about **relationships**, not durations
- Use `cargo check` to understand lifetime errors

## Key Learning Points

- **Lifetime Annotations**: How to express relationships between references
- **Borrowing Rules**: How lifetimes enforce memory safety
- **Elision Rules**: When lifetimes can be omitted
- **Practical Concepts**: Real-world application of lifetimes in systems

## Substrate Connection

Substrate uses lifetimes for:
- Runtime API calls with borrowed data
- Storage iterators that borrow from storage
- Event and error types with borrowed strings
- Pallet configurations with static references

---
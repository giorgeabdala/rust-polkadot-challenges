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

Lifetimes ensure memory safety by tracking how long references are valid. They prevent:
- **Dangling pointers**: References to deallocated memory
- **Use after free**: Accessing freed memory
- **Data races**: Concurrent access violations

Substrate uses lifetimes extensively for zero-copy operations and safe runtime interactions.

## Challenge

Create a configuration management system that demonstrates practical lifetime usage.

### Requirements

1. **Create a `ConfigValue` enum** that can hold different types:
   ```rust
   enum ConfigValue<'a> {
       Text(&'a str),
       Number(i64),
       Boolean(bool),
       List(Vec<&'a str>),
   }
   ```

2. **Create a `Config` struct** that manages configuration:
   ```rust
   struct Config<'a> {
       name: &'a str,
       values: HashMap<&'a str, ConfigValue<'a>>,
   }
   ```

3. **Implement methods for `Config<'a>`:**
   - `new(name: &'a str) -> Self`
   - `set_text(&mut self, key: &'a str, value: &'a str)`
   - `set_number(&mut self, key: &'a str, value: i64)`
   - `set_boolean(&mut self, key: &'a str, value: bool)`
   - `set_list(&mut self, key: &'a str, values: Vec<&'a str>)`
   - `get(&self, key: &str) -> Option<&ConfigValue<'a>>`
   - `get_text(&self, key: &str) -> Option<&str>`
   - `get_number(&self, key: &str) -> Option<i64>`

4. **Create a `ConfigManager` struct** that manages multiple configs:
   ```rust
   struct ConfigManager<'a> {
       configs: Vec<Config<'a>>,
       default_config: Option<&'a Config<'a>>,
   }
   ```

5. **Implement methods for `ConfigManager<'a>`:**
   - `new() -> Self`
   - `add_config(&mut self, config: Config<'a>)`
   - `find_config(&self, name: &str) -> Option<&Config<'a>>`
   - `set_default(&mut self, config: &'a Config<'a>)`
   - `get_value(&self, config_name: &str, key: &str) -> Option<&ConfigValue<'a>>`

### Expected Behavior

```rust
// String literals have 'static lifetime
let config_name = "database";
let host_key = "host";
let host_value = "localhost";

let mut config = Config::new(config_name);
config.set_text(host_key, host_value);
config.set_number("port", 5432);
config.set_boolean("ssl", true);

// References must live as long as the config
let db_hosts = vec!["primary", "secondary", "backup"];
config.set_list("replicas", db_hosts);

let mut manager = ConfigManager::new();
manager.add_config(config);

// Borrowing with explicit lifetimes
let found_config = manager.find_config("database").unwrap();
let host = found_config.get_text("host").unwrap();
```

## Advanced Requirements

1. **Create a function that returns the longest-lived reference:**
   ```rust
   fn longest_config_name<'a>(c1: &'a Config, c2: &'a Config) -> &'a str {
       // Return the name of the config with more values
   }
   ```

2. **Create a struct that holds references with different lifetimes:**
   ```rust
   struct ConfigSnapshot<'config, 'data> {
       config: &'config Config<'data>,
       timestamp: &'config str,
   }
   ```

3. **Implement a method with lifetime bounds:**
   ```rust
   impl<'a> Config<'a> {
       fn merge_from<'b>(&mut self, other: &Config<'b>) 
       where 
           'b: 'a  // 'b outlives 'a
       {
           // Merge configurations
       }
   }
   ```

## Testing

Write tests that demonstrate:
- Configs with string literals ('static lifetime)
- Configs with borrowed strings (explicit lifetimes)
- Lifetime relationships between structs
- Compilation failures when lifetimes don't match
- Successful borrowing patterns

## Common Lifetime Patterns

1. **Input/Output Lifetimes:**
   ```rust
   fn get_config_value<'a>(config: &'a Config, key: &str) -> Option<&'a str>
   ```

2. **Multiple Lifetime Parameters:**
   ```rust
   fn compare_configs<'a, 'b>(c1: &'a Config, c2: &'b Config) -> bool
   ```

3. **Lifetime Bounds:**
   ```rust
   fn process_config<'a, 'b>(config: &'a Config<'b>) where 'b: 'a
   ```

## Tips

- Start with lifetime elision (let compiler infer)
- Add explicit lifetimes only when compiler requires them
- Use `'static` for string literals and global data
- Remember: lifetimes are about relationships, not durations
- Use `cargo check` to understand lifetime errors

## Key Learning Points

- **Lifetime Annotations**: Expressing relationships between references
- **Borrowing Rules**: How lifetimes enforce memory safety
- **Elision Rules**: When lifetimes can be omitted
- **Lifetime Bounds**: Constraining lifetime relationships

## Substrate Connection

Substrate uses lifetimes for:
- Runtime API calls with borrowed data
- Storage iterators that borrow from storage
- Event and error types with borrowed strings
- Pallet configurations with static references

## Bonus Challenges

⚠️ **For Advanced Exploration**

1. **Complex lifetime relationships** - Practice with multiple lifetime parameters and bounds
2. **Lifetime elision rules** - Understand when explicit annotations are needed
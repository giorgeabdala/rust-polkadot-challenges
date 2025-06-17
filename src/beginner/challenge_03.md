# Challenge 3: Enums and Pattern Matching

**Estimated Time:** 30 minutes  
**Difficulty:** Beginner  
**Topics:** Enums, Pattern Matching, Match Expressions

## Learning Objectives

By completing this challenge, you will understand:
- How to define and use enums in Rust
- Pattern matching with `match` expressions
- Destructuring enum variants
- Using `if let` for simple pattern matching

## Background

Enums in Rust are powerful data types that can hold different variants, each potentially containing different data. Combined with pattern matching, they provide a safe way to handle different states and conditions.

## Challenge

Create simple color and message systems using enums to demonstrate pattern matching.

### Requirements

1. **Create a `Color` enum** with variants:
   - `Red`
   - `Green` 
   - `Blue`
   - `Custom(u8, u8, u8)` - RGB values

2. **Create a `Message` enum** with variants:
   - `Text(String)` - text message
   - `Number(i32)` - numeric message
   - `Warning` - simple warning

3. **Implement functions:**
   - `describe_color(color: Color) -> String`
   - `process_message(msg: Message) -> String`

### Expected Behavior

```rust
// Working with colors
let red = Color::Red;
let custom = Color::Custom(255, 128, 0);

println!("{}", describe_color(red)); // "Primary color: Red"
println!("{}", describe_color(custom)); // "Custom color: RGB(255, 128, 0)"

// Working with messages
let text_msg = Message::Text("Hello World".to_string());
let num_msg = Message::Number(42);
let warning = Message::Warning;

println!("{}", process_message(text_msg)); // "Text: Hello World"
println!("{}", process_message(num_msg)); // "Number: 42"
println!("{}", process_message(warning)); // "Warning received!"
```

## Testing

Your tests should cover:
- Each color variant and its description
- Each message type and its processing

Example test structure:
```rust
#[test]
fn test_colors() {
    let red = Color::Red;
    let result = describe_color(red);
    assert_eq!(result, "Primary color: Red");
    
    let custom = Color::Custom(100, 150, 200);
    let result = describe_color(custom);
    assert_eq!(result, "Custom color: RGB(100, 150, 200)");
}

#[test]
fn test_messages() {
    let msg = Message::Text("Test".to_string());
    let result = process_message(msg);
    assert_eq!(result, "Text: Test");
}
```

## Tips

- Always handle all enum variants in match expressions
- Use `_` as a catch-all pattern when needed
- Extract data from variants using pattern matching
- Use descriptive variant names

## Key Learning Points

- **Enum Variants**: Different ways to store data in enums
- **Pattern Matching**: Using `match` to handle all possible cases
- **Destructuring**: Extracting data from enum variants
- **Exhaustive Matching**: Compiler ensures all cases are handled

## Common Patterns to Practice

1. **Simple matching**:
   ```rust
   match color {
       Color::Red => "Red color",
       Color::Green => "Green color",
       _ => "Other color",
   }
   ```

2. **Data extraction**:
   ```rust
   match message {
       Message::Text(content) => format!("Text: {}", content),
       Message::Number(n) => format!("Number: {}", n),
       Message::Warning => "Warning!".to_string(),
   }
   ```

The goal is to become comfortable with Rust's enum system and pattern matching, which are fundamental to writing idiomatic Rust code.

## Bonus Challenges

⚠️ **Optional - For Deeper Exploration Only**

1. **Advanced pattern matching** - Practice complex destructuring patterns used in blockchain state machines 
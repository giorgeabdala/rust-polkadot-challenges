# Challenge 5: Result<T, E> - Error Handling

**Estimated Time:** 35 minutes  
**Difficulty:** Beginner  
**Topics:** Result<T, E>, Error Handling, Pattern Matching

## Learning Objectives

By completing this challenge, you will understand:
- How `Result<T, E>` represents operations that can fail
- Pattern matching with `Ok` and `Err`
- Basic error handling patterns
- When to use `Result<T, E>` vs `Option<T>`

## Background

Rust uses `Result<T, E>` for error handling instead of exceptions:
- `Ok(T)` - contains the successful result of type T
- `Err(E)` - contains the error of type E

This makes errors explicit and forces you to handle them.

## Challenge

Create simple operations that demonstrate error handling with `Result<T, E>`.

### Requirements

1. **Create a `SimpleError` enum** with variants:
   - `InvalidInput`
   - `OutOfRange`

2. **Implement functions that return `Result<T, E>`:**
   - `safe_divide(a: i32, b: i32) -> Result<i32, SimpleError>`
   - `parse_positive(s: &str) -> Result<i32, SimpleError>`

### Expected Behavior

```rust
// Successful operation
match safe_divide(10, 2) {
    Ok(result) => println!("Result: {}", result), // "Result: 5"
    Err(e) => println!("Error: {:?}", e),
}

// Error case
match safe_divide(10, 0) {
    Ok(result) => println!("Result: {}", result),
    Err(SimpleError::InvalidInput) => println!("Cannot divide by zero!"),
    Err(e) => println!("Other error: {:?}", e),
}

// Parsing examples
match parse_positive("42") {
    Ok(num) => println!("Parsed: {}", num), // "Parsed: 42"
    Err(e) => println!("Parse error: {:?}", e),
}

match parse_positive("-5") {
    Ok(num) => println!("Parsed: {}", num),
    Err(SimpleError::OutOfRange) => println!("Number must be positive!"),
    Err(e) => println!("Other error: {:?}", e),
}

// Using unwrap_or for defaults
let safe_result = safe_divide(10, 0).unwrap_or(0);
println!("Safe result: {}", safe_result); // "Safe result: 0"
```

## Testing

Your tests should demonstrate:
- Successful operations returning `Ok(value)`
- Error cases returning `Err(error)`
- Pattern matching on different error types
- Using `unwrap_or` for safe defaults

Example test approach:
```rust
#[test]
fn test_successful_division() {
    let result = safe_divide(10, 2);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_division_by_zero() {
    let result = safe_divide(10, 0);
    match result {
        Err(SimpleError::InvalidInput) => {}, // Expected
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_positive_parsing() {
    let result = parse_positive("42");
    assert_eq!(result, Ok(42));
    
    let result = parse_positive("-5");
    match result {
        Err(SimpleError::OutOfRange) => {}, // Expected
        _ => panic!("Expected OutOfRange error"),
    }
}
```

## Tips

- Use `Result<T, E>` for operations that can fail in expected ways
- Create specific error types rather than using generic strings
- Use pattern matching to handle different error cases
- Use `unwrap_or()` to provide safe defaults

## Key Learning Points

- **Explicit Error Handling**: All failures are represented as `Result<T, E>`
- **Pattern Matching**: Handle both success and error cases
- **Error Types**: Custom enums for different error conditions
- **Safe Defaults**: Using `unwrap_or` for fallback values

## Bonus Challenges

1. **Add more error variants** like `Overflow` or `InvalidFormat`
2. **Create a chain of operations** using the `?` operator
3. **Implement error conversion** using `From` trait
4. **Add context to errors** with additional information

## Common Patterns

1. **Basic Error Handling**:
   ```rust
   match operation() {
       Ok(value) => println!("Success: {}", value),
       Err(error) => println!("Failed: {:?}", error),
   }
   ```

2. **Safe Defaults**:
   ```rust
   let result = risky_operation().unwrap_or(default_value);
   ```

3. **Error Propagation** (bonus):
   ```rust
   fn multiple_operations() -> Result<i32, SimpleError> {
       let a = parse_positive("10")?;
       let b = parse_positive("2")?;
       safe_divide(a, b)
   }
   ```

The goal is to become comfortable with Rust's explicit error handling and learn to write robust code that gracefully handles failure cases. 
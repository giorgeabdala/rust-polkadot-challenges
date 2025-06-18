# Challenge 5: Result<T, E> - Error Handling

**Estimated Time:** 45 minutes  
**Difficulty:** Beginner  
**Topics:** Result<T, E>, Error Handling, Pattern Matching

## Learning Objectives

- Understand how `Result<T, E>` represents operations that can fail
- Practice pattern matching with `Ok` and `Err`
- Learn basic error handling patterns
- Use the `?` operator for error propagation

## What You Need to Implement

### Step 1: Create an Error Type
Create a `SimpleError` enum with these variants:
```rust
#[derive(Debug, PartialEq)]
enum SimpleError {
    InvalidInput,
    OutOfRange,
}
```

### Step 2: Implement Core Functions
Implement these **exact** functions:

```rust
// Returns Err(SimpleError::InvalidInput) if b is 0, otherwise Ok(a/b)
fn safe_divide(a: i32, b: i32) -> Result<i32, SimpleError>

// Returns:
// - Err(SimpleError::InvalidInput) if string can't be parsed to i32
// - Err(SimpleError::OutOfRange) if number is negative or zero
// - Ok(number) if valid positive integer
fn parse_positive(s: &str) -> Result<i32, SimpleError>

// Chains operations using ? operator - parse two strings and divide them
fn calculate_average(numbers: &[&str]) -> Result<i32, SimpleError>
```

### Step 3: Implement Error Conversion
Make `SimpleError` convertible from `std::num::ParseIntError`:
```rust
impl From<std::num::ParseIntError> for SimpleError {
    fn from(_: std::num::ParseIntError) -> Self {
        SimpleError::InvalidInput
    }
}
```

## Exact Implementation Requirements

**`safe_divide` function:**
- Input: two i32 numbers
- Return `Err(SimpleError::InvalidInput)` if second number is 0
- Return `Ok(result)` with integer division otherwise

**`parse_positive` function:**
- Input: string slice
- Parse the string to i32
- Return `Err(SimpleError::InvalidInput)` if parsing fails
- Return `Err(SimpleError::OutOfRange)` if number ≤ 0
- Return `Ok(number)` if number > 0

**`calculate_average` function:**
- Input: slice of string 
- Parse both strings using `parse_positive`
- Add them together and divide by 2 using `safe_divide`
- Use `?` operator to propagate any errors
- Return the final result

## Required Tests

Create these **exact** tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_success() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(7, 3), Ok(2)); // Integer division
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(10, 0), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_parse_positive_success() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("1"), Ok(1));
    }

    #[test]
    fn test_parse_positive_invalid_string() {
        assert_eq!(parse_positive("abc"), Err(SimpleError::InvalidInput));
        assert_eq!(parse_positive("12.5"), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_parse_positive_out_of_range() {
        assert_eq!(parse_positive("-5"), Err(SimpleError::OutOfRange));
        assert_eq!(parse_positive("0"), Err(SimpleError::OutOfRange));
    }

    #[test]
    fn test_calculate_average_success() {
        assert_eq!(calculate_average(&["10", "20"]), Ok(15));
        assert_eq!(calculate_average(&["3", "7"]), Ok(5));
    }

    #[test]
    fn test_calculate_average_parse_error() {
        assert_eq!(calculate_average(&["abc", "5"]), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_calculate_average_range_error() {
        assert_eq!(calculate_average(&["-1", "5"]), Err(SimpleError::OutOfRange));
    }
}
```

## Success Criteria

✅ All tests pass  
✅ Functions handle errors correctly  
✅ `?` operator works for error propagation  
✅ Pattern matching works on different error types  

## Key Concepts You'll Practice

- **Result<T, E>**: Rust's way of handling errors explicitly
- **Pattern Matching**: Using `match` to handle `Ok` and `Err`
- **Error Propagation**: Using `?` to pass errors up the call stack
- **Custom Error Types**: Creating your own error enums

## Background Info

In Rust, we don't use exceptions. Instead:
- `Ok(value)` means the operation succeeded
- `Err(error)` means the operation failed
- You must handle both cases explicitly

The `?` operator is shorthand for "if this is an error, return it immediately; otherwise, unwrap the success value." 
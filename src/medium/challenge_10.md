# Challenge 10: Basic Testing and Documentation

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Unit Testing, Documentation, Test-Driven Development

## Learning Objectives

By completing this challenge, you will understand:
- Writing basic unit tests with Rust's testing framework
- Creating clear documentation with examples
- Test-driven development practices
- Testing error conditions and edge cases

## Background

Testing and documentation are crucial for maintainable software:
- **Unit Tests**: Test individual functions and methods
- **Documentation**: Explain code purpose and usage with examples
- **Test-Driven Development**: Write tests before implementation
- **Error Testing**: Verify error conditions work correctly

Substrate has extensive testing practices that ensure reliability.

## Challenge

Create a well-tested and documented simple account management system.

### Data Types

```rust
/// Simple calculator for demonstrating testing and documentation
#[derive(Debug)]
pub struct Calculator;

/// Errors that can occur during calculations
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    Overflow,
}
```

### Provided Implementation

```rust
impl Calculator {
    /// Creates a new calculator instance
/// 
/// # Examples
/// 
/// ```
    /// let calc = Calculator::new();
/// ```
    pub fn new() -> Self {
        Self
    }
}
```

### Your Implementation

#### **1. Addition:**
```rust
impl Calculator {
    /// Adds two numbers together
    /// 
    /// # Arguments
    /// 
    /// * `a` - First number
    /// * `b` - Second number
    /// 
    /// # Returns
    /// 
    /// * `Ok(sum)` - The sum of a and b
    /// * `Err(CalcError::Overflow)` - If result overflows
    /// 
    /// # Examples
    /// 
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(5, 3).unwrap(), 8);
    /// ```
    // TODO: Implement this method
    pub fn add(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        // IMPLEMENT:
        // 1. Use checked_add to prevent overflow
        // 2. Return Ok(result) or Err(CalcError::Overflow)
        todo!()
    }
}
```

#### **2. Division:**
```rust
impl Calculator {
    /// Divides one number by another
    /// 
    /// # Arguments
    /// 
    /// * `a` - Dividend
    /// * `b` - Divisor
    /// 
    /// # Returns
    /// 
    /// * `Ok(quotient)` - The result of a/b
    /// * `Err(CalcError::DivisionByZero)` - If b is zero
    /// 
    /// # Examples
    /// 
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.divide(10, 2).unwrap(), 5);
    /// ```
    // TODO: Implement this method
    pub fn divide(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        // IMPLEMENT:
        // 1. Check if b is 0 (return DivisionByZero error)
        // 2. Return Ok(a / b)
        todo!()
    }
}
```

#### **3. Is Even:**
```rust
impl Calculator {
    /// Checks if a number is even
    /// 
    /// # Arguments
    /// 
    /// * `n` - Number to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If number is even
    /// * `false` - If number is odd
    /// 
    /// # Examples
    /// 
    /// ```
    /// let calc = Calculator::new();
    /// assert!(calc.is_even(4));
    /// assert!(!calc.is_even(5));
    /// ```
    // TODO: Implement this method
    pub fn is_even(&self, n: u32) -> bool {
        // IMPLEMENT:
        // Return true if n % 2 == 0
        todo!()
    }
}
```

### Tests to Implement

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_success() {
        // TODO: Implement this test
        // 1. Create calculator
        // 2. Test normal addition
        // 3. Verify result is correct
        todo!()
    }

    #[test]
    fn test_add_overflow() {
        // TODO: Implement this test
        // 1. Try to add numbers that would overflow
        // 2. Verify it returns Overflow error
        todo!()
    }

    #[test]
    fn test_divide_success() {
        // TODO: Implement this test
        // 1. Test normal division
        // 2. Verify result is correct
        todo!()
    }

    #[test]
    fn test_divide_by_zero() {
        // TODO: Implement this test
        // 1. Try to divide by zero
        // 2. Verify it returns DivisionByZero error
        todo!()
    }

    #[test]
    fn test_is_even() {
        // TODO: Implement this test
        // 1. Test with even numbers (0, 2, 4)
        // 2. Test with odd numbers (1, 3, 5)
        // 3. Verify results are correct
        todo!()
    }
}
```

## Expected Usage

```rust
fn main() {
    let calc = Calculator::new();
    
    // Test addition
    match calc.add(5, 3) {
        Ok(result) => println!("5 + 3 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Test division
    match calc.divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Test even/odd
    println!("4 is even: {}", calc.is_even(4));
    println!("5 is even: {}", calc.is_even(5));
}
```

## Key Learning Points

- **Unit testing**: Use `#[test]` for testing functions
- **Documentation**: Use `///` comments with examples
- **Error testing**: Verify error conditions work correctly
- **Assertions**: Use `assert_eq!`, `assert!` for verification

## Substrate Connection

Substrate development requires:
- **Comprehensive tests** for all pallet functions
- **Clear documentation** with working examples
- **Error handling** for all edge cases
- **Test-driven development** practices

This simplified version teaches the essential testing and documentation patterns used in Substrate!
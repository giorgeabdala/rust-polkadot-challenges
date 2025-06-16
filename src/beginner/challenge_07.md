# Challenge 7: Generics - Code Reuse

**Estimated Time:** 30 minutes  
**Difficulty:** Beginner  
**Topics:** Generic Types, Associated Types, Trait Bounds

## Learning Objectives

By completing this challenge, you will understand:
- How to write generic structs and implementations
- Using trait bounds to constrain generic types
- Associated types in traits
- The `Copy` trait and its implications

## Background

Generics allow you to write code that works with multiple types while maintaining type safety and performance. Combined with traits, they enable powerful abstractions without runtime cost.

## Challenge

Create a generic storage system that can work with different numeric types (based on existing code structure).

### Requirements

1. **Create a `Summable` trait** with:
   - `type Output` - associated type for the result
   - `fn sum_with(&self, other: &Self) -> Self::Output`

2. **Create a `Store<T>` struct** with:
   - `value: T`

3. **Implement methods:**
   - `Store<T>::new(value: T) -> Self` where `T: Clone + Add<Output = T> + Copy`
   - Implement `Summable` for `Store<T>` where `T: Clone + Add<Output = T> + Copy`

### Expected Behavior

```rust
// Works with u32
let store_u32 = Store::new(10u32);
let store2_u32 = Store::new(20u32);
let result = store_u32.sum_with(&store2_u32);
assert_eq!(result, 30u32);

// Works with i64
let store_i64 = Store::new(-5i64);
let store2_i64 = Store::new(15i64);
let result = store_i64.sum_with(&store2_i64);
assert_eq!(result, 10i64);

// Works with f64
let store_f64 = Store::new(3.5f64);
let store2_f64 = Store::new(2.1f64);
let result = store_f64.sum_with(&store2_f64);
// result is approximately 5.6
```

## Implementation Notes

This challenge is based on existing code in `challenge_06.rs`. The time estimate reflects understanding and potentially extending the existing implementation.

Your task is to:
1. Understand how the generic `Store<T>` works
2. Comprehend the trait bounds (`Clone + Add<Output = T> + Copy`)
3. See how associated types work in the `Summable` trait
4. Verify the implementation works with different numeric types

## Testing

The existing code includes tests for `u32` and `i64`. Consider adding:
- Tests for `f64` (be careful with floating-point comparisons)
- Tests that demonstrate the generic nature of the code
- Understanding why certain trait bounds are required

Example test expansion:
```rust
#[test]
fn test_sum_with_f64() {
    let a = Store::new(5.5f64);
    let b = Store::new(2.3f64);
    let result = a.sum_with(&b);
    assert!((result - 7.8).abs() < 0.001); // Floating point comparison
}
```

## Tips

- Trait bounds specify what operations are available on generic types
- Associated types make traits more flexible than using generics in the trait definition
- The `Copy` trait allows values to be copied rather than moved
- Generic code is monomorphized at compile time for zero runtime cost

## Key Learning Points

- **Generic Structs**: Writing code that works with multiple types
- **Trait Bounds**: Constraining generic types to have certain capabilities
- **Associated Types**: Flexible trait design
- **Zero-Cost Abstractions**: Generics compile to specific code for each type used 
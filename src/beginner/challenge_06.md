# Challenge 6: Traits - Shared Behavior

**Estimated Time:** 25 minutes  
**Difficulty:** Beginner  
**Topics:** Traits, Trait Implementation, Polymorphism

## Learning Objectives

By completing this challenge, you will understand:
- How to define and implement traits
- Using traits for shared behavior across types
- Generic structs with trait bounds
- Basic polymorphism in Rust

## Background

Traits in Rust define shared behavior that types can implement. They're similar to interfaces in other languages and enable polymorphism while maintaining Rust's zero-cost abstractions.

## Challenge

Create an animal behavior system using traits (based on existing code structure).

### Requirements

1. **Create an `Animal` trait** with:
   - `fn speak(&self)` - required method

2. **Create animal structs:**
   - `Dog` (empty struct)
   - `Cat` (empty struct)

3. **Create a `Handler<T>` struct** with:
   - `animal: T`

4. **Implement methods:**
   - `Handler<T>::new(animal: T) -> Self` where `T: Animal`
   - `Handler<T>::make_animal_speak(&self)` where `T: Animal`

### Expected Behavior

```rust
let dog = Dog {};
let cat = Cat {};

// Direct trait usage
dog.speak(); // Should print "Woof woof"
cat.speak(); // Should print "Meow!! I am a cat"

// Using generic handler
let dog_handler = Handler::new(dog);
dog_handler.make_animal_speak();

let cat_handler = Handler::new(cat);
cat_handler.make_animal_speak();
```

## Implementation Notes

This challenge is based on existing code in `challenge_05.rs`. The time estimate reflects adapting and understanding the existing implementation rather than building from scratch.

Your task is to:
1. Understand how the trait system works in the existing code
2. Ensure all animals implement the `Animal` trait correctly
3. Verify the generic `Handler` works with trait bounds
4. Add any missing functionality

## Testing

The existing code includes a basic test. Expand it to verify:
- Each animal type speaks correctly
- The generic handler works with different animal types
- Trait bounds are enforced properly

## Tips

- Use `&self` for methods that only read data
- Trait bounds (`T: Animal`) constrain what types can be used
- Generic code is monomorphized at compile time for zero-cost abstractions

## Key Learning Points

- **Trait Definition**: Specifying shared behavior
- **Trait Implementation**: Providing specific behavior for each type
- **Generic Constraints**: Using trait bounds to limit generic types
- **Zero-Cost Abstractions**: Generics have no runtime overhead

## Bonus Challenges

⚠️ **Optional - For Deeper Exploration Only**

1. **Advanced iterator patterns** - Practice `filter_map`, `fold`, and custom iterators
2. **Performance comparison** - Compare different collection operations 
# Challenge 1: Ownership and Move Semantics

**Estimated Time:** 40 minutes  
**Difficulty:** Beginner  
**Topics:** Ownership, Move Semantics, Stack vs Heap

## Learning Objectives

By completing this challenge, you will understand:
- How Rust's ownership system works
- The difference between stack and heap allocation
- When values are moved vs copied
- How to work with owned values

## Background

Ownership is Rust's most unique feature. Every value in Rust has a single owner, and when the owner goes out of scope, the value is dropped. This prevents memory leaks and data races at compile time.

Some types implement the `Copy` trait (like integers) and are copied when assigned. Others (like `String`) are moved, transferring ownership.

## Challenge

Create a simple player system that demonstrates ownership and move semantics.

### Requirements

1. **Create a `Player` struct** with:
   - `name: String` 
   - `score: u32`

2. **Implement methods:**
   - `Player::new(name: String) -> Self`
   - `Player::add_score(&mut self, points: u32)`
   - `Player::get_info(&self) -> String`

3. **Create a function:**
   - `transfer_player(player: Player) -> Player` - takes ownership and returns it

### Expected Behavior

```rust
let mut player1 = Player::new("Alice".to_string());
player1.add_score(100);

println!("{}", player1.get_info()); // "Alice: 100 points"

// Demonstrate ownership transfer
let player2 = transfer_player(player1);
// player1 is now moved and can't be used again

println!("{}", player2.get_info()); // Still works with player2
```

## Testing

Your implementation should demonstrate:

1. **Basic Operations**: Create player, add score, get info
2. **Ownership Transfer**: Show that moved values can't be reused
3. **Copy vs Move**: Understand why `String` moves but `u32` copies

Write tests for:
- Creating a player and adding score
- Getting player information
- Demonstrating that transferred players are moved

## Tips

- Use `String::from()` or `.to_string()` to create owned strings
- Remember that when you pass `Player` to a function, ownership moves
- Pay attention to compiler errors about moved values - they're teaching you!
- Use `format!()` macro to create formatted strings

## Key Learning Points

- **Stack vs Heap**: `u32` is copied, `String` is moved
- **Ownership Transfer**: When you move a value, the original owner can't use it
- **Borrowing**: Using `&` to access without taking ownership
- **Mutable Borrowing**: Using `&mut` to modify without taking ownership

## Bonus Challenges

1. Add a `clone_player()` method that creates a copy instead of moving
2. Implement `Display` trait for pretty printing
3. Add a `reset_score()` method

Remember: The goal is to understand ownership, not to write complex code. Focus on seeing how values move and when you need to borrow vs own. 
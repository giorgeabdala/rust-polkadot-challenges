# Challenge 2: Borrowing and References

**Estimated Time:** 20 minutes  
**Difficulty:** Beginner  
**Topics:** Borrowing, References, Mutable References, Borrow Checker

## Learning Objectives

By completing this challenge, you will understand:
- The difference between owning and borrowing
- Immutable references (`&T`)
- Mutable references (`&mut T`)
- Rust's borrowing rules

## Background

Borrowing allows you to use values without taking ownership. Rust has two types of references:
- **Immutable references** (`&T`): You can have many, but can't modify the value
- **Mutable references** (`&mut T`): You can have only one, but can modify the value

## Challenge

Create a simple library system that demonstrates borrowing and references.

### Requirements

1. **Create a `Book` struct** with:
   - `title: String`
   - `available: bool`

2. **Create a `Library` struct** with:
   - `books: Vec<Book>`

3. **Implement methods:**
   - `Library::new() -> Self`
   - `Library::add_book(&mut self, book: Book)`
   - `Library::find_book(&self, title: &str) -> Option<&Book>`
   - `Library::borrow_book(&mut self, title: &str) -> bool`
   - `Library::return_book(&mut self, title: &str) -> bool`

### Expected Behavior

```rust
let mut library = Library::new();

let book = Book {
    title: "The Rust Book".to_string(),
    available: true,
};

library.add_book(book);

// Immutable borrow - can have multiple
let book_ref = library.find_book("The Rust Book");

// Mutable borrow - only one at a time
library.borrow_book("The Rust Book");
```

## Testing

Write tests to verify:
- Finding books returns correct references
- Borrowing a book changes its availability
- Returning a book makes it available again
- Multiple immutable borrows work simultaneously

## Tips

- Use `Option<&T>` for methods that might not find what they're looking for
- Remember that `&mut self` means the method can modify the struct
- Use `&self` when you only need to read data

## Key Learning Points

- **Multiple Immutable Borrows**: Several references to the same data
- **Single Mutable Borrow**: Only one mutable reference at a time
- **Borrow Checker**: Prevents data races and use-after-free bugs

## Bonus Challenges

1. **Implement a reservation system** where books can be reserved but not yet borrowed
2. **Add book categories** and implement filtering with borrowed references
3. **Create a method that takes a closure** operating on borrowed book data
4. **Implement iterator methods** that return references to books

The goal is to become comfortable with Rust's borrowing system and understand when to use owned values vs references. 
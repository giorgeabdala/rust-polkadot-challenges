# Challenge 8: Smart Pointers Basics (Simplified)

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Box, Rc, RefCell, Basic Memory Management

## Learning Objectives

By completing this challenge, you will understand:
- Heap allocation with Box<T>
- Shared ownership with Rc<T>
- Interior mutability with RefCell<T>
- When to use each smart pointer type

## Background

Smart pointers provide additional capabilities beyond regular references. This simplified version focuses on the three most common smart pointers.

### Smart Pointer Usage

| Smart Pointer | Use Case | Example |
|---------------|----------|---------|
| `Box<T>` | Heap allocation | Recursive data structures |
| `Rc<T>` | Shared ownership | Multiple references to same data |
| `RefCell<T>` | Interior mutability | Modify data through shared reference |

## Challenge

Create simple data structures using each smart pointer type.

### Data Types

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Simple node for demonstrating Box<T>
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Box<Node>>,
}

// Shared counter for demonstrating Rc<RefCell<T>>
#[derive(Debug)]
struct SharedCounter {
    value: Rc<RefCell<i32>>,
}
```

### Provided Implementation

```rust
impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }
}

impl SharedCounter {
    pub fn new() -> Self {
        Self {
            value: Rc::new(RefCell::new(0)),
        }
    }
}
```

### Your Implementation

#### **1. Add Child (Box demonstration):**
```rust
impl Node {
    // TODO: Implement this method
    pub fn add_child(&mut self, value: i32) {
        // IMPLEMENT:
        // 1. Create new Node with given value
        // 2. Wrap it in Box
        // 3. Push to children vector
        todo!()
    }
}
```

#### **2. Increment Counter (Rc<RefCell> demonstration):**
```rust
impl SharedCounter {
    // TODO: Implement this method
    pub fn increment(&self) {
        // IMPLEMENT:
        // 1. Borrow the value mutably using borrow_mut()
        // 2. Increment the value by 1
        todo!()
    }
}
```

#### **3. Get Value (Rc<RefCell> demonstration):**
```rust
impl SharedCounter {
    // TODO: Implement this method
    pub fn get(&self) -> i32 {
        // IMPLEMENT:
        // 1. Borrow the value using borrow()
        // 2. Return the dereferenced value
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
    fn test_box_usage() {
        // TODO: Implement this test
        // 1. Create a Node
        // 2. Add children using add_child
        // 3. Verify children were added
        todo!()
    }

    #[test]
    fn test_shared_counter() {
        // TODO: Implement this test
        // 1. Create SharedCounter
        // 2. Create multiple references with clone
        // 3. Increment from different references
        // 4. Verify all see the same value
        todo!()
    }
}
```

## Expected Usage

```rust
fn main() {
    // Box example: heap allocation for recursive structure
    let mut root = Node::new(1);
    root.add_child(2);
    root.add_child(3);
    
    // Rc<RefCell> example: shared mutable state
    let counter1 = SharedCounter::new();
    let counter2 = Rc::clone(&counter1.value);
    
    counter1.increment();
    println!("Counter value: {}", counter1.get()); // Should be 1
}
```

## Key Learning Points

- **Box<T>**: Heap allocation for recursive data structures
- **Rc<T>**: Multiple ownership through reference counting
- **RefCell<T>**: Interior mutability with runtime borrow checking
- **Memory safety**: Smart pointers prevent common memory errors

## Substrate Connection

Substrate uses smart pointers extensively:
- Storage items often use `Rc` for shared data
- Runtime modules use `RefCell` for interior mutability
- Complex data structures use `Box` for heap allocation

This simplified version covers the essential smart pointer patterns used in Substrate!
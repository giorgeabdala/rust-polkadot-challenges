# Challenge 9: Concurrency Basics (Simplified)

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Threads, Arc, Mutex, Simple Channel Communication

## Learning Objectives

By completing this challenge, you will understand:
- Basic thread creation and joining
- Shared state with Arc<Mutex<T>>
- Message passing with channels
- Thread-safe data sharing

## Background

Rust provides safe concurrency primitives. This simplified version focuses on the most essential patterns.

### Key Concepts

| Primitive | Use Case | Example |
|-----------|----------|---------|
| **thread::spawn** | Create threads | CPU-bound work |
| **Arc<Mutex<T>>** | Shared mutable state | Counter, data sharing |
| **mpsc::channel** | Thread communication | Send messages between threads |

## Challenge

Create simple examples demonstrating each concurrency primitive.

### Data Types

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

// Simple counter for Arc<Mutex> demonstration
#[derive(Debug)]
struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

// Simple message for channel demonstration
#[derive(Debug, Clone)]
struct Message {
    id: u32,
    content: String,
}
```

### Provided Implementation

```rust
impl SharedCounter {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }
}

impl Message {
    pub fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
}
```

### Your Implementation

#### **1. Increment Counter (Arc/Mutex demonstration):**
```rust
impl SharedCounter {
    // TODO: Implement this method
    pub fn increment(&self) {
        // IMPLEMENT:
        // 1. Lock the mutex using lock().unwrap()
        // 2. Increment the value by 1
        todo!()
    }
}
```

#### **2. Get Counter Value:**
```rust
impl SharedCounter {
    // TODO: Implement this method  
    pub fn get(&self) -> i32 {
        // IMPLEMENT:
        // 1. Lock the mutex using lock().unwrap()
        // 2. Return the dereferenced value
        todo!()
    }
}
```

#### **3. Simple Threading Function:**
```rust
// TODO: Implement this function
fn spawn_workers() -> Vec<thread::JoinHandle<()>> {
    // IMPLEMENT:
    // 1. Create empty vector for handles
    // 2. Spawn 3 threads that print their thread number
    // 3. Each thread should print "Thread {id} working"
    // 4. Return vector of join handles
    todo!()
}
```

### Tests to Implement

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_counter() {
        // TODO: Implement this test
        // 1. Create SharedCounter
        // 2. Create multiple Arc clones
        // 3. Spawn threads that increment counter
        // 4. Join threads and verify final value
        todo!()
    }

    #[test]
    fn test_channel_communication() {
        // TODO: Implement this test
        // 1. Create mpsc channel
        // 2. Spawn thread that sends messages
        // 3. Receive messages in main thread
        // 4. Verify all received correctly
        todo!()
    }
}
```

## Expected Usage

```rust
fn main() {
    // Arc<Mutex> example: shared counter
    let counter = SharedCounter::new();
    let handles = (0..3).map(|_| {
        let counter_clone = Arc::clone(&counter.value);
        thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        })
    }).collect::<Vec<_>>();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", counter.get());
    
    // Channel example: simple message passing
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send(Message::new(1, "Hello from thread!".to_string())).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {:?}", received);
}
```

## Key Learning Points

- **thread::spawn**: Create new threads for parallel execution
- **Arc<Mutex<T>>**: Share mutable data safely between threads  
- **mpsc::channel**: Send messages between threads
- **join()**: Wait for threads to complete

## Substrate Connection

Substrate uses these patterns extensively:
- **Arc<Mutex>** for shared runtime state
- **Channels** for component communication
- **Threads** for parallel block processing

This simplified version teaches the essential concurrency patterns used in Substrate!
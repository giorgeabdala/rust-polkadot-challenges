# Challenge 9: Basic Threading and Channels

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Threads, Channels, Message Passing, Basic Concurrency

## Learning Objectives

By completing this challenge, you will understand:
- Creating and managing basic threads
- Message passing with channels
- Thread communication patterns
- Basic synchronization concepts

## Background

Concurrency enables programs to handle multiple tasks:
- **Threads**: OS-level parallelism for separate tasks
- **Channels**: Message passing for communication between threads
- **Message Passing**: Safe data sharing without shared state

### Basic Threading Patterns

| Pattern | Use Case | Why? |
|---------|----------|------|
| `thread::spawn` | Independent tasks | Parallel execution |
| `mpsc::channel` | Thread communication | Message passing |
| `join()` | Wait for completion | Synchronization |

## Challenge

Create a simple multi-threaded transaction processor using channels.

### Structures to Implement

#### **Basic Data Types:**
```rust
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Transaction {
    id: u64,
    from: String,
    to: String,
    amount: u64,
}

#[derive(Debug, Clone)]
struct ProcessedTransaction {
    transaction: Transaction,
    status: TransactionStatus,
    processing_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
enum TransactionStatus {
    Completed,
    Failed(String),
}
```

#### **Processor Structure:**
```rust
struct TransactionProcessor {
    id: u32,
    receiver: Receiver<Transaction>,
    result_sender: Sender<ProcessedTransaction>,
}

struct ProcessorManager {
    workers: Vec<thread::JoinHandle<()>>,
    tx_sender: Sender<Transaction>,
    result_receiver: Receiver<ProcessedTransaction>,
}
```

### Provided Implementations

#### **Basic Transaction Processing:**
```rust
impl Transaction {
    pub fn new(id: u64, from: String, to: String, amount: u64) -> Self {
        Self { id, from, to, amount }
    }
    
    // Simulate processing time
    pub fn process(&self) -> TransactionStatus {
        thread::sleep(Duration::from_millis(10)); // Simulate work
        
        if self.amount == 0 {
            TransactionStatus::Failed("Zero amount".to_string())
        } else if self.from == self.to {
            TransactionStatus::Failed("Same sender and receiver".to_string())
        } else {
            TransactionStatus::Completed
        }
    }
}

impl ProcessedTransaction {
    pub fn new(transaction: Transaction, status: TransactionStatus, processing_time_ms: u64) -> Self {
        Self { transaction, status, processing_time_ms }
    }
}
```

### Methods for You to Implement

#### **1. Processor Manager Creation (`new`):**
```rust
impl ProcessorManager {
    // TODO: Implement this method
    pub fn new(num_workers: usize) -> Self {
        // IMPLEMENT:
        // 1. Create channel for sending transactions to workers
        // 2. Create channel for receiving results from workers
        // 3. Initialize empty workers vector
        // 4. Return ProcessorManager with senders/receivers
        todo!()
    }
}
```

#### **2. Start Worker Threads (`start`):**
```rust
impl ProcessorManager {
    // TODO: Implement this method
    pub fn start(&mut self) {
        // IMPLEMENT:
        // 1. For each worker (0..num_workers):
        //    - Clone the necessary channels
        //    - Spawn thread that runs process_transactions
        //    - Add JoinHandle to workers vector
        // 2. Each worker should:
        //    - Receive transactions from tx channel
        //    - Process each transaction
        //    - Send result to result channel
        //    - Break when channel is closed
        todo!()
    }
}
```

#### **3. Submit Transaction (`submit_transaction`):**
```rust
impl ProcessorManager {
    // TODO: Implement this method
    pub fn submit_transaction(&self, transaction: Transaction) -> Result<(), String> {
        // IMPLEMENT:
        // 1. Send transaction through tx_sender
        // 2. Return Ok(()) if successful
        // 3. Return Err with error message if channel is closed
        todo!()
    }
}
```

#### **4. Get Processed Transaction (`get_result`):**
```rust
impl ProcessorManager {
    // TODO: Implement this method
    pub fn get_result(&self) -> Option<ProcessedTransaction> {
        // IMPLEMENT:
        // 1. Try to receive from result_receiver (non-blocking)
        // 2. Return Some(result) if available
        // 3. Return None if no result available
        // Use try_recv() for non-blocking receive
        todo!()
    }
}
```

#### **5. Shutdown (`shutdown`):**
```rust
impl ProcessorManager {
    // TODO: Implement this method  
    pub fn shutdown(self) {
        // IMPLEMENT:
        // 1. Drop tx_sender to close channel (signals workers to stop)
        // 2. Wait for all worker threads to finish using join()
        // 3. Handle any join errors appropriately
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
    fn test_transaction_processing() {
        // TODO: Implement this test
        // 1. Create valid and invalid transactions
        // 2. Process them and verify status
        todo!()
    }

    #[test]
    fn test_processor_manager() {
        // TODO: Implement this test
        // 1. Create ProcessorManager with 2 workers
        // 2. Start the workers
        // 3. Submit several transactions
        // 4. Collect results
        // 5. Verify all transactions were processed
        // 6. Shutdown manager
        todo!()
    }

    #[test]
    fn test_channel_communication() {
        // TODO: Implement this test
        // 1. Test basic channel send/receive
        // 2. Test channel closure behavior
        todo!()
    }

    #[test]
    fn test_multiple_workers() {
        // TODO: Implement this test
        // 1. Submit many transactions
        // 2. Verify they're processed by different workers
        // 3. Check all results are received
        todo!()
    }
}
```

### Threading Patterns

#### **1. Basic Thread Creation:**
```rust
let handle = thread::spawn(|| {
    println!("Hello from thread!");
    42 // Return value
});

let result = handle.join().unwrap(); // Wait and get result
```

#### **2. Channel Communication:**
```rust
let (sender, receiver) = mpsc::channel();

thread::spawn(move || {
    sender.send("Hello").unwrap();
});

let message = receiver.recv().unwrap();
```

#### **3. Multiple Producers:**
```rust
let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        tx_clone.send(format!("Message {}", i)).unwrap();
    });
}
```

### Example Usage

```rust
fn main() {
    // Create processor manager with 3 workers
    let mut manager = ProcessorManager::new(3);
    
    // Start the worker threads
    manager.start();
    
    // Submit transactions
    for i in 0..10 {
        let tx = Transaction::new(
            i,
            format!("account_{}", i),
            format!("account_{}", i + 1),
            100 + i,
        );
        
        if let Err(e) = manager.submit_transaction(tx) {
            println!("Failed to submit transaction: {}", e);
        }
    }
    
    // Collect results
    let mut results = Vec::new();
    for _ in 0..10 {
        while let Some(result) = manager.get_result() {
            results.push(result);
        }
        thread::sleep(Duration::from_millis(50));
    }
    
    println!("Processed {} transactions", results.len());
    
    // Shutdown
    manager.shutdown();
}
```

### Expected Output

A basic threading system that:
- Creates worker threads for parallel processing
- Uses channels for safe communication between threads
- Processes transactions concurrently
- Handles thread lifecycle (start, work, shutdown)
- Demonstrates message passing patterns

### Theoretical Context

**Threading Fundamentals:**
- **Thread Creation**: Spawning independent execution units
- **Message Passing**: Communication without shared state
- **Channel Types**: Single producer, multiple consumer (mpsc)
- **Synchronization**: Waiting for thread completion

**Key Patterns:**
1. **Producer-Consumer**: One thread produces, others consume
2. **Worker Pool**: Multiple threads processing from shared queue
3. **Message Passing**: Safe data sharing through channels
4. **Graceful Shutdown**: Properly closing channels and joining threads

**Substrate Connection:**
- Block import pipeline uses worker threads
- Network handling with concurrent connections  
- Transaction pool with parallel processing
- Consensus algorithms with threaded execution

This challenge teaches essential threading patterns needed for understanding Substrate's concurrent architecture.

--- 
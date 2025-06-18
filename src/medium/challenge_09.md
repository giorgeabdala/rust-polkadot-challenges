# Challenge 9: Concurrency and Threading

**Estimated Time:** 45 minutes  
**Difficulty:** Medium  
**Topics:** Threads, Channels, Mutex, RwLock, Atomic Types, Thread Safety

## Learning Objectives

By completing this challenge, you will understand:
- Creating and managing threads
- Message passing with channels
- Shared state with Mutex and RwLock
- Atomic operations for lock-free programming
- Thread safety patterns and best practices

## Background

Concurrency enables programs to handle multiple tasks simultaneously:
- **Threads**: OS-level parallelism for CPU-bound tasks
- **Channels**: Message passing for communication between threads
- **Mutexes**: Mutual exclusion for shared mutable state
- **Atomics**: Lock-free operations for simple data types
- **Thread Safety**: Ensuring data races don't occur

### 🎯 **Concurrency vs Parallelism**

#### **Concurrency** - Dealing with multiple things at once (logical):
```rust
// One chef managing multiple orders (time-slicing)
let mut chef = Chef::new();
chef.start_cooking_pasta();    // Switch between tasks
chef.check_pizza_oven();       // when waiting for others
chef.plate_salad();
```

#### **Parallelism** - Doing multiple things at once (physical):
```rust
// Multiple chefs working simultaneously (true parallel)
let chefs = vec![Chef::new(); 4];
chefs.par_iter().enumerate().for_each(|(i, chef)| {
    chef.cook_order(orders[i]);  // Each chef works independently
});
```

### ⚠️ **Data Races vs Race Conditions**

#### **Data Race** - Undefined behavior from unsynchronized access:
```rust
// ❌ DATA RACE: Two threads accessing same memory without synchronization
static mut COUNTER: usize = 0;

thread::spawn(|| unsafe { COUNTER += 1 });  // Thread 1
thread::spawn(|| unsafe { COUNTER += 1 });  // Thread 2
// Undefined behavior! Could corrupt memory
```

#### **Race Condition** - Logic error from timing dependencies:
```rust
// ✅ No data race (using Arc<Mutex>) but still has race condition
let balance = Arc::new(Mutex::new(100));
let b1 = balance.clone();
let b2 = balance.clone();

// Both threads check balance before withdrawing
thread::spawn(move || {
    let mut bal = b1.lock().unwrap();
    if *bal >= 60 { *bal -= 60; }  // Check-then-act race
});

thread::spawn(move || {
    let mut bal = b2.lock().unwrap();
    if *bal >= 60 { *bal -= 60; }  // Both might withdraw!
});
```

### 🆚 **Threads vs Async: When to Use What?**

| Use Threads When | Use Async When |
|------------------|----------------|
| ✅ CPU-intensive work | ✅ I/O-bound operations |
| ✅ Blocking operations | ✅ Network requests |
| ✅ Parallel computation | ✅ File system operations |
| ✅ Independent tasks | ✅ Thousands of connections |

```rust
// CPU-bound: Use threads
let handles: Vec<_> = (0..num_cpus::get()).map(|_| {
    thread::spawn(|| {
        expensive_cpu_work();  // Utilize multiple cores
    })
}).collect();

// I/O-bound: Use async
async fn handle_requests() {
    let futures: Vec<_> = requests.into_iter().map(|req| {
        async move { process_request(req).await }  // Concurrent I/O
    }).collect();
    
    futures::future::join_all(futures).await;
}
```

Substrate uses threading for block processing, networking, and consensus algorithms.

## Challenge

Create a multi-threaded blockchain transaction processor.

### Requirements

1. **Create basic data structures:**
   ```rust
   #[derive(Debug, Clone)]
   struct Transaction {
       id: u64,
       from: String,
       to: String,
       amount: u64,
       timestamp: u64,
   }

   #[derive(Debug, Clone)]
   struct ProcessedTransaction {
       transaction: Transaction,
       status: TransactionStatus,
       processing_time_ms: u64,
   }

   #[derive(Debug, Clone, PartialEq)]
   enum TransactionStatus {
       Pending,
       Processing,
       Completed,
       Failed(String),
   }
   ```

2. **Create a `TransactionPool` with thread-safe operations:**
   ```rust
   use std::sync::{Arc, Mutex, RwLock};
   use std::sync::mpsc::{Sender, Receiver};
   use std::sync::atomic::{AtomicU64, Ordering};

   struct TransactionPool {
       pending: Arc<Mutex<VecDeque<Transaction>>>,
       processed: Arc<RwLock<HashMap<u64, ProcessedTransaction>>>,
       total_processed: AtomicU64,
       stats: Arc<Mutex<PoolStats>>,
   }

   #[derive(Debug, Default)]
   struct PoolStats {
       total_received: u64,
       total_completed: u64,
       total_failed: u64,
       average_processing_time: f64,
   }
   ```

3. **Create a `TransactionProcessor` worker:**
   ```rust
   struct TransactionProcessor {
       id: u32,
       pool: Arc<TransactionPool>,
       receiver: Receiver<Transaction>,
       shutdown: Arc<AtomicBool>,
   }
   ```

4. **Create a `ProcessorManager` to coordinate workers:**
   ```rust
   struct ProcessorManager {
       pool: Arc<TransactionPool>,
       workers: Vec<JoinHandle<()>>,
       sender: Sender<Transaction>,
       shutdown: Arc<AtomicBool>,
   }
   ```

5. **Implement methods:**
   - `TransactionPool::new() -> Arc<Self>`
   - `TransactionPool::submit_transaction(&self, tx: Transaction)`
   - `TransactionPool::get_transaction_status(&self, id: u64) -> Option<TransactionStatus>`
   - `TransactionPool::get_stats(&self) -> PoolStats`
   - `ProcessorManager::new(pool: Arc<TransactionPool>, num_workers: usize) -> Self`
   - `ProcessorManager::start(&mut self)`
   - `ProcessorManager::shutdown(&self)`

### Expected Behavior

```rust
use std::time::Duration;

// Create transaction pool and processor manager
let pool = TransactionPool::new();
let mut manager = ProcessorManager::new(pool.clone(), 4);

// Start processing threads
manager.start();

// Submit transactions
for i in 0..100 {
    let tx = Transaction {
        id: i,
        from: format!("account_{}", i % 10),
        to: format!("account_{}", (i + 1) % 10),
        amount: 100 + i,
        timestamp: current_timestamp(),
    };
    pool.submit_transaction(tx);
}

// Wait for processing
std::thread::sleep(Duration::from_secs(2));

// Check results
let stats = pool.get_stats();
println!("Processed: {}, Failed: {}", stats.total_completed, stats.total_failed);

// Shutdown
manager.shutdown();
```

## Advanced Requirements

1. **Implement a `WorkStealingQueue` for load balancing:**
   ```rust
   struct WorkStealingQueue<T> {
       queues: Vec<Arc<Mutex<VecDeque<T>>>>,
       next_queue: AtomicUsize,
   }
   
   impl<T> WorkStealingQueue<T> {
       fn new(num_queues: usize) -> Self;
       fn push(&self, item: T);
       fn pop(&self, worker_id: usize) -> Option<T>;
       fn steal(&self, worker_id: usize) -> Option<T>;
   }
   ```

2. **Create a `RateLimiter` using atomics:**
   ```rust
   struct RateLimiter {
       tokens: AtomicU64,
       last_refill: AtomicU64,
       max_tokens: u64,
       refill_rate: u64, // tokens per second
   }
   
   impl RateLimiter {
       fn new(max_tokens: u64, refill_rate: u64) -> Self;
       fn try_acquire(&self, tokens: u64) -> bool;
       fn refill(&self);
   }
   ```

3. **Implement a `ThreadSafeCounter` with different synchronization methods:**
   ```rust
   // Using Mutex
   struct MutexCounter(Arc<Mutex<u64>>);
   
   // Using Atomic
   struct AtomicCounter(AtomicU64);
   
   // Using RwLock
   struct RwLockCounter(Arc<RwLock<u64>>);
   
   trait Counter: Send + Sync {
       fn increment(&self);
       fn get(&self) -> u64;
       fn add(&self, value: u64);
   }
   ```

## Testing

Write tests that demonstrate:
- Concurrent access to shared data
- Message passing between threads
- Thread safety with different synchronization primitives
- Performance comparison of different approaches
- Proper cleanup and shutdown

```rust
#[test]
fn test_concurrent_transaction_processing() {
    let pool = TransactionPool::new();
    let mut manager = ProcessorManager::new(pool.clone(), 2);
    
    manager.start();
    
    // Submit transactions from multiple threads
    let handles: Vec<_> = (0..4).map(|thread_id| {
        let pool = pool.clone();
        std::thread::spawn(move || {
            for i in 0..25 {
                let tx = Transaction {
                    id: thread_id * 25 + i,
                    from: format!("thread_{}", thread_id),
                    to: "destination".to_string(),
                    amount: 100,
                    timestamp: current_timestamp(),
                };
                pool.submit_transaction(tx);
            }
        })
    }).collect();
    
    // Wait for all submissions
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Wait for processing
    std::thread::sleep(Duration::from_millis(500));
    
    let stats = pool.get_stats();
    assert_eq!(stats.total_received, 100);
    
    manager.shutdown();
}

#[test]
fn test_atomic_vs_mutex_performance() {
    const ITERATIONS: u64 = 1_000_000;
    const THREADS: usize = 4;
    
    // Test AtomicCounter
    let atomic_counter = Arc::new(AtomicCounter::new());
    let start = Instant::now();
    
    let handles: Vec<_> = (0..THREADS).map(|_| {
        let counter = atomic_counter.clone();
        std::thread::spawn(move || {
            for _ in 0..ITERATIONS / THREADS as u64 {
                counter.increment();
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_time = start.elapsed();
    assert_eq!(atomic_counter.get(), ITERATIONS);
    
    // Compare with MutexCounter...
}
```

## Concurrency Patterns

1. **Producer-Consumer with Channels:**
   ```rust
   let (tx, rx) = mpsc::channel();
   
   // Producer thread
   std::thread::spawn(move || {
       for i in 0..10 {
           tx.send(i).unwrap();
       }
   });
   
   // Consumer thread
   std::thread::spawn(move || {
       while let Ok(value) = rx.recv() {
           println!("Received: {}", value);
       }
   });
   ```

2. **Shared State with Mutex:**
   ```rust
   let data = Arc::new(Mutex::new(Vec::new()));
   let mut handles = vec![];
   
   for i in 0..10 {
       let data = data.clone();
       let handle = std::thread::spawn(move || {
           let mut data = data.lock().unwrap();
           data.push(i);
       });
       handles.push(handle);
   }
   
   for handle in handles {
       handle.join().unwrap();
   }
   ```

3. **Lock-Free with Atomics:**
   ```rust
   let counter = Arc::new(AtomicU64::new(0));
   let mut handles = vec![];
   
   for _ in 0..10 {
       let counter = counter.clone();
       let handle = std::thread::spawn(move || {
           for _ in 0..1000 {
               counter.fetch_add(1, Ordering::Relaxed);
           }
       });
       handles.push(handle);
   }
   ```

## Tips

- Use channels for communication, mutexes for shared state
- Prefer `RwLock` when reads are more frequent than writes
- Use atomics for simple counters and flags
- Always handle `PoisonError` from mutex operations
- Use `Arc` to share ownership across threads

## Key Learning Points

- **Thread Creation**: Spawning and joining threads
- **Message Passing**: Using channels for thread communication
- **Shared State**: Synchronizing access with mutexes and locks
- **Atomic Operations**: Lock-free programming for performance
- **Thread Safety**: Ensuring data races don't occur

## Substrate Connection

Substrate's concurrency patterns:
- Block import pipeline with multiple threads
- Network handling with async/await and threads
- Consensus algorithms with shared state
- Transaction pool with concurrent access
- RPC server handling multiple requests

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Lock-free programming** - Understand atomic operations in blockchain contexts
2. **Performance profiling** - Measure and optimize concurrent blockchain operations 
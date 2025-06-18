# Challenge 5: Async Programming and Futures

**Estimated Time:** 45 minutes  
**Difficulty:** Medium  
**Topics:** Async/Await, Futures, Tokio Runtime, Concurrent Operations

## Learning Objectives

By completing this challenge, you will understand:
- Async/await syntax and execution model
- Creating and composing futures
- Concurrent vs parallel execution
- Error handling in async contexts
- Common async patterns and pitfalls

## Dependencies Setup

Before starting this challenge, you will need to configure the necessary dependencies for async programming:

#### **Option 1 - Quick Setup:**
```bash
# Add async dependencies to your Cargo.toml
cargo add tokio --features full
cargo add futures
```

#### **Option 2 - Manual Cargo.toml:**
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

After adding dependencies:
```bash
cargo check  # Verify dependencies are correctly installed
```

## Background

Asynchronous programming is a paradigm shift that enables handling thousands of concurrent operations efficiently. Understanding async deeply is crucial for Substrate development where network I/O, database operations, and peer communication must scale without blocking.

### 🎯 **Async vs Threads: The Fundamental Difference**

#### **Traditional Threading Model:**
```rust
use std::thread;

// Each connection gets its own OS thread
for connection in incoming_connections {
    thread::spawn(move || {
        handle_connection(connection);  // Blocks thread until complete
    });
}
// Problem: 10,000 connections = 10,000 threads = memory explosion!
```

#### **Async Model:**
```rust
use tokio;

// All connections handled by a few threads
for connection in incoming_connections {
    tokio::spawn(async move {
        handle_connection_async(connection).await;  // Yields when waiting
    });
}
// Solution: 10,000 connections handled by ~4-8 threads!
```

### 🔮 **Understanding Futures**

A `Future` is a value that represents a computation that will complete in the future:

```rust
// A Future is like a recipe - it describes what to do, but doesn't cook yet
let future = async {
    let data = fetch_data().await;
    process_data(data).await
};

// Nothing happens until you await or poll the future
let result = future.await;  // Now it actually runs
```

#### **Future States:**
```rust
enum Poll<T> {
    Ready(T),      // Future completed with result
    Pending,       // Future not ready yet, try again later
}

// Futures are state machines that progress through states
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

#### **Futures are Lazy:**
```rust
async fn expensive_computation() -> u32 {
    println!("Starting computation...");  // This won't print!
    42
}

let future = expensive_computation();  // No work done yet
// ... later ...
let result = future.await;  // Now it actually runs and prints
```

### ⚡ **The Async Runtime (Tokio)**

The runtime is the engine that executes async code:

#### **What the Runtime Does:**
1. **Task Scheduling**: Decides which async tasks to run when
2. **I/O Event Loop**: Monitors network/file operations for completion
3. **Waker System**: Notifies tasks when they can make progress
4. **Thread Pool**: Manages worker threads for CPU-bound tasks

```rust
// Runtime architecture visualization
┌─────────────────────────────────────────┐
│             Tokio Runtime               │
├─────────────────────────────────────────┤
│  Event Loop  │  Scheduler  │ Thread Pool │
│     I/O      │    Tasks    │ CPU Work    │
├─────────────────────────────────────────┤
│         Your Async Tasks                │
│    await    │   await    │    await     │
└─────────────────────────────────────────┘
```

#### **Runtime Types:**
```rust
// Single-threaded runtime (for testing/simple cases)
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // All tasks run on one thread
}

// Multi-threaded runtime (default, production-ready)
#[tokio::main]  // Equivalent to multi_thread
async fn main() {
    // Tasks distributed across multiple threads
}

// Custom runtime configuration
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()
    .unwrap();

rt.block_on(async {
    // Your async code here
});
```

### 🔄 **Concurrency vs Parallelism in Async**

#### **Concurrency** - Making progress on multiple tasks:
```rust
async fn concurrent_example() {
    // These run concurrently - when one waits, others can progress
    let future1 = fetch_user_data();      // Starts HTTP request
    let future2 = fetch_user_posts();     // Starts another HTTP request
    let future3 = fetch_user_friends();   // Starts third HTTP request
    
    // All three requests happen "at the same time"
    let (user, posts, friends) = tokio::join!(future1, future2, future3);
}
```

#### **True Parallelism** - CPU-bound work on multiple cores:
```rust
async fn parallel_example() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    // Spawn blocking tasks for CPU-intensive work
    let handles: Vec<_> = data.chunks(2).map(|chunk| {
        let chunk = chunk.to_vec();
        tokio::task::spawn_blocking(move || {
            // This runs on a separate thread pool
            expensive_cpu_computation(chunk)
        })
    }).collect();
    
    // Await all parallel computations
    let results: Vec<_> = futures::future::join_all(handles).await;
}
```

### 🎛️ **Async Control Flow Patterns**

#### **Sequential Execution:**
```rust
async fn sequential() {
    let step1 = do_step_1().await;          // Wait for step 1
    let step2 = do_step_2(step1).await;     // Then step 2
    let step3 = do_step_3(step2).await;     // Then step 3
    // Total time: sum of all steps
}
```

#### **Concurrent Execution:**
```rust
async fn concurrent() {
    // All start immediately
    let (result1, result2, result3) = tokio::join!(
        independent_task_1(),
        independent_task_2(),
        independent_task_3()
    );
    // Total time: max of all tasks
}
```

#### **Racing/Selection:**
```rust
async fn racing() {
    tokio::select! {
        result = fetch_from_primary() => {
            println!("Primary responded first: {:?}", result);
        }
        result = fetch_from_backup() => {
            println!("Backup responded first: {:?}", result);
        }
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            println!("Both services timed out!");
        }
    }
}
```

### 🌊 **Streams: Async Iterators**

Streams are like async iterators - they yield values over time:

```rust
use futures::stream::{Stream, StreamExt};

async fn process_stream() {
    let mut stream = get_data_stream();
    
    while let Some(item) = stream.next().await {
        process_item(item).await;
    }
}

// Creating streams
use futures::stream;

let stream = stream::iter(vec![1, 2, 3, 4, 5])
    .map(|x| async move { x * 2 })
    .buffer_unordered(3);  // Process up to 3 items concurrently
```

### 🚨 **Common Async Pitfalls and Solutions**

#### **Pitfall 1: Blocking in Async Context**
```rust
// ❌ BAD: This blocks the entire async runtime
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1));  // Blocks everything!
    let data = std::fs::read_to_string("file.txt").unwrap();  // Blocks!
}

// ✅ GOOD: Use async versions
async fn good_example() {
    tokio::time::sleep(Duration::from_secs(1)).await;  // Yields to other tasks
    let data = tokio::fs::read_to_string("file.txt").await.unwrap();  // Async I/O
}
```

#### **Pitfall 2: Not Understanding Send + Sync**
```rust
// ❌ BAD: Rc is not Send, can't cross thread boundaries
async fn bad_shared_state() {
    let data = Rc::new(RefCell::new(42));
    tokio::spawn(async move {
        // Error: Rc<RefCell<i32>> is not Send
        *data.borrow_mut() += 1;
    });
}

// ✅ GOOD: Use Arc + Mutex for shared state
async fn good_shared_state() {
    let data = Arc::new(Mutex::new(42));
    let data_clone = data.clone();
    
    tokio::spawn(async move {
        let mut guard = data_clone.lock().await;
        *guard += 1;
    });
}
```

#### **Pitfall 3: Creating Too Many Tasks**
```rust
// ❌ BAD: Creates millions of tasks
async fn spawn_heavy() {
    for i in 0..1_000_000 {
        tokio::spawn(async move {
            tiny_work(i).await;
        });
    }
}

// ✅ GOOD: Batch work or use worker pools
async fn batch_work() {
    let work: Vec<_> = (0..1_000_000).collect();
    
    // Process in chunks
    for chunk in work.chunks(1000) {
        let tasks: Vec<_> = chunk.iter().map(|&i| tiny_work(i)).collect();
        futures::future::join_all(tasks).await;
    }
}
```

### 🎨 **Substrate/Polkadot Async Patterns**

#### **Network Layer Pattern:**
```rust
// Substrate uses async extensively for networking
trait NetworkService {
    async fn send_request(&self, peer: PeerId, request: Request) -> Result<Response, Error>;
    async fn handle_incoming(&mut self) -> Option<(PeerId, Request)>;
}

// Background task pattern
async fn network_worker(mut service: NetworkService) {
    loop {
        tokio::select! {
            Some((peer, request)) = service.handle_incoming() => {
                // Handle incoming request
                tokio::spawn(async move {
                    handle_request(peer, request).await;
                });
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                // Periodic maintenance
                service.cleanup_stale_connections().await;
            }
        }
    }
}
```

#### **Block Import Pipeline:**
```rust
// Async pipeline for processing blocks
async fn block_import_pipeline() {
    let (block_sender, mut block_receiver) = mpsc::channel(100);
    
    // Background task for importing blocks
    tokio::spawn(async move {
        while let Some(block) = block_receiver.recv().await {
            // Async block validation and import
            match validate_block(&block).await {
                Ok(_) => import_block(block).await,
                Err(e) => handle_invalid_block(block, e).await,
            }
        }
    });
    
    // Main task receives blocks from network
    while let Some(block) = network.next_block().await {
        block_sender.send(block).await.unwrap();
    }
}
```

#### **RPC Server Pattern:**
```rust
// Async RPC methods
#[rpc]
pub trait ChainApi {
    #[method(name = "chain_getBlock")]
    async fn get_block(&self, hash: Option<BlockHash>) -> Result<Block, Error>;
    
    #[method(name = "chain_subscribeNewHeads")]
    async fn subscribe_new_heads(&self) -> SubscriptionSink;
}

// Implementation with async database access
impl ChainApi for ChainService {
    async fn get_block(&self, hash: Option<BlockHash>) -> Result<Block, Error> {
        let hash = hash.unwrap_or_else(|| self.best_block_hash());
        
        // Async database lookup
        self.database
            .get_block(hash)
            .await?
            .ok_or(Error::BlockNotFound)
    }
}
```

### 🔧 **Error Handling in Async Context**

```rust
// Propagating errors through async chains
async fn error_handling_example() -> Result<String, Box<dyn std::error::Error>> {
    let data = fetch_data().await?;           // ? works with async
    let processed = process_data(data).await?;
    let result = finalize(processed).await?;
    Ok(result)
}

// Collecting errors from concurrent operations
async fn handle_multiple_results() {
    let futures = vec![
        fetch_from_source_1(),
        fetch_from_source_2(),
        fetch_from_source_3(),
    ];
    
    let results = futures::future::join_all(futures).await;
    
    let (successes, failures): (Vec<_>, Vec<_>) = results
        .into_iter()
        .partition(Result::is_ok);
    
    println!("Successes: {}, Failures: {}", successes.len(), failures.len());
}
```

### 💡 **Performance Considerations**

1. **Async overhead**: Small for I/O-bound tasks, significant for CPU-bound
2. **Task granularity**: Don't spawn tasks for trivial work
3. **Backpressure**: Handle scenarios where producers outpace consumers
4. **Memory usage**: Async tasks hold state between yields

Understanding async programming unlocks efficient, scalable applications in the Substrate ecosystem!

## Challenge

Create an async blockchain data fetcher that demonstrates async patterns.

### Requirements

1. **Create async data structures:**
   ```rust
   #[derive(Debug, Clone)]
   struct Block {
       number: u64,
       hash: String,
       timestamp: u64,
       transactions: Vec<String>,
   }

   #[derive(Debug)]
   struct NetworkError {
       message: String,
       retry_after: Option<u64>,
   }
   ```

2. **Create a `BlockchainClient` struct:**
   ```rust
   struct BlockchainClient {
       base_url: String,
       timeout_ms: u64,
   }
   ```

3. **Implement async methods:**
   - `async fn fetch_block(&self, number: u64) -> Result<Block, NetworkError>`
   - `async fn fetch_latest_block(&self) -> Result<Block, NetworkError>`
   - `async fn fetch_block_range(&self, start: u64, end: u64) -> Result<Vec<Block>, NetworkError>`
   - `async fn fetch_blocks_concurrent(&self, numbers: Vec<u64>) -> Vec<Result<Block, NetworkError>>`

4. **Create a `BlockCache` struct:**
   ```rust
   struct BlockCache {
       cache: Arc<Mutex<HashMap<u64, Block>>>,
       client: BlockchainClient,
   }
   ```

5. **Implement async caching methods:**
   - `async fn get_block(&self, number: u64) -> Result<Block, NetworkError>`
   - `async fn get_blocks(&self, numbers: Vec<u64>) -> Vec<Result<Block, NetworkError>>`
   - `async fn refresh_cache(&self, numbers: Vec<u64>) -> Result<(), NetworkError>`

### Expected Behavior

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BlockchainClient::new("https://api.blockchain.com");
    
    // Sequential fetching
    let block = client.fetch_block(12345).await?;
    println!("Block: {:?}", block);
    
    // Concurrent fetching
    let block_numbers = vec![1, 2, 3, 4, 5];
    let blocks = client.fetch_blocks_concurrent(block_numbers).await;
    
    // With caching
    let cache = BlockCache::new(client);
    let cached_block = cache.get_block(12345).await?;
    
    Ok(())
}
```

## Advanced Requirements

1. **Implement timeout handling:**
   ```rust
   use tokio::time::{timeout, Duration};
   
   async fn fetch_with_timeout(&self, number: u64) -> Result<Block, NetworkError> {
       timeout(Duration::from_millis(self.timeout_ms), self.fetch_block(number))
           .await
           .map_err(|_| NetworkError::timeout())?
   }
   ```

2. **Create a stream processor:**
   ```rust
   use futures::stream::{Stream, StreamExt};
   
   async fn process_block_stream<S>(&self, mut stream: S) -> Result<Vec<Block>, NetworkError>
   where
       S: Stream<Item = u64> + Unpin,
   {
       let mut blocks = Vec::new();
       while let Some(block_number) = stream.next().await {
           let block = self.fetch_block(block_number).await?;
           blocks.push(block);
       }
       Ok(blocks)
   }
   ```

3. **Implement retry logic with exponential backoff:**
   ```rust
   async fn fetch_with_retry(&self, number: u64, max_retries: u32) -> Result<Block, NetworkError> {
       let mut retries = 0;
       loop {
           match self.fetch_block(number).await {
               Ok(block) => return Ok(block),
               Err(e) if retries < max_retries => {
                   let delay = Duration::from_millis(100 * 2_u64.pow(retries));
                   tokio::time::sleep(delay).await;
                   retries += 1;
               }
               Err(e) => return Err(e),
           }
       }
   }
   ```

## Testing

Write async tests that demonstrate:
- Basic async/await functionality
- Concurrent operations vs sequential
- Error handling in async contexts
- Timeout behavior
- Stream processing

```rust
#[tokio::test]
async fn test_concurrent_fetching() {
    let client = BlockchainClient::new("test");
    let numbers = vec![1, 2, 3];
    
    let start = std::time::Instant::now();
    let results = client.fetch_blocks_concurrent(numbers).await;
    let duration = start.elapsed();
    
    // Should be faster than sequential fetching
    assert!(duration < Duration::from_millis(300));
    assert_eq!(results.len(), 3);
}
```

## Async Patterns

1. **Join Multiple Futures:**
   ```rust
   let (block1, block2, block3) = tokio::join!(
       client.fetch_block(1),
       client.fetch_block(2),
       client.fetch_block(3)
   );
   ```

2. **Select First Completed:**
   ```rust
   let result = tokio::select! {
       block = client.fetch_block(1) => block,
       _ = tokio::time::sleep(Duration::from_secs(5)) => {
           Err(NetworkError::timeout())
       }
   };
   ```

3. **Spawn Background Tasks:**
   ```rust
   let handle = tokio::spawn(async move {
       client.fetch_block(12345).await
   });
   let block = handle.await??;
   ```

## Tips

- Use `Arc<Mutex<T>>` for shared state in async contexts
- Prefer `tokio::spawn` for CPU-bound tasks
- Use `futures::join!` for concurrent operations
- Handle cancellation with `tokio::select!`
- Be careful with `Mutex` in async code (prefer `tokio::sync::Mutex`)

## Key Learning Points

- **Async Execution Model**: How futures are polled and executed
- **Concurrency Patterns**: join, select, spawn for different scenarios
- **Error Propagation**: Using `?` in async functions
- **Resource Management**: Sharing data between async tasks
- **Performance**: When to use concurrent vs sequential execution

## Substrate Connection

Substrate's async usage:
- Network layer (libp2p) for peer communication
- RPC server handling concurrent requests
- Block import pipeline with async processing
- Off-chain workers with async HTTP requests
- Database operations with async traits

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Concurrent async operations** - Practice patterns used in Substrate networking
2. **Async error handling strategies** - Advanced error propagation in async contexts 
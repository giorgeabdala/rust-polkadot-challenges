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

## Background

Asynchronous programming enables efficient I/O operations without blocking threads. Key concepts:
- **Futures**: Represent values that will be available later
- **Async/Await**: Syntax for writing asynchronous code
- **Runtime**: Executes async tasks (like Tokio)
- **Concurrency**: Multiple tasks making progress simultaneously

Substrate uses async extensively for network operations, RPC calls, and runtime interactions.

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

1. Create a `BlockSubscription` that streams new blocks
2. Implement a connection pool for multiple blockchain clients
3. Add metrics collection for async operations
4. Create a generic `AsyncCache<K, V>` trait
5. Implement graceful shutdown for long-running async tasks 
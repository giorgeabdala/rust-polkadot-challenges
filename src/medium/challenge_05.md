# Challenge 5: Async Programming - Polkadot SDK Essentials

**Estimated Time:** 45 minutes  
**Difficulty:** Medium  
**Topics:** Async/Await, Futures, Concurrent Operations, Shared State

## Learning Objectives

By completing this challenge, you will understand:
- Essential async/await syntax for Substrate
- Concurrent vs sequential operations
- Shared state with Arc<Mutex<T>>
- Error handling in async contexts
- Async patterns used in Polkadot SDK

## Dependencies Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

```bash
cargo check  # Verify dependencies
```

## Essential Concepts for Polkadot SDK

### 🎯 **Why Async is Crucial in Polkadot SDK**

In Substrate/Polkadot, async is used for:
- **Networking**: P2P communication between nodes
- **RPC Server**: Multiple simultaneous requests
- **Block Import**: Block processing pipeline
- **Off-chain Workers**: External HTTP operations

### ⚡ **Concurrency - The Fundamental Pattern**

```rust
// ❌ Sequential - Slow
async fn sequential_blocks() {
    let block1 = fetch_block(1).await;  // 100ms
    let block2 = fetch_block(2).await;  // 100ms
    let block3 = fetch_block(3).await;  // 100ms
    // Total: 300ms
}

// ✅ Concurrent - Fast (Substrate pattern)
async fn concurrent_blocks() {
    let (block1, block2, block3) = tokio::join!(
        fetch_block(1),
        fetch_block(2),
        fetch_block(3)
    );
    // Total: ~100ms
}
```

### 🔄 **Shared State - Arc<Mutex<T>>**

```rust
use std::sync::Arc;
use tokio::sync::Mutex;  // Async-friendly mutex

// Pattern used in Substrate for cache/state
let shared_cache = Arc::new(Mutex::new(HashMap::new()));

// Share between tasks
let cache_clone = shared_cache.clone();
tokio::spawn(async move {
    let mut cache = cache_clone.lock().await;
    cache.insert(key, value);
});
```

### 🚨 **Async Error Handling**

```rust
// Error propagation (Substrate pattern)
async fn substrate_pattern() -> Result<Block, NetworkError> {
    let data = fetch_data().await?;
    let block = process_block(data).await?;
    Ok(block)
}
```

## Challenge - Blockchain Data Fetcher

Implement a blockchain data fetcher that demonstrates Polkadot SDK patterns.

### Base Structures

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
}

impl NetworkError {
    fn new(msg: &str) -> Self {
        Self { message: msg.to_string() }
    }
}
```

### Requirements (Core - 30 minutes)

**1. BlockchainClient** (15 min):
```rust
struct BlockchainClient {
    base_url: String,
    timeout_ms: u64,
}

impl BlockchainClient {
    fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout_ms: 1000,
        }
    }

    // Simulate fetching a block (with delay)
    async fn fetch_block(&self, number: u64) -> Result<Block, NetworkError> {
        // TODO: Implement
        // Simulate network delay with tokio::time::sleep()
        // Return mock block or occasional error
    }

    // Concurrent fetch (ESSENTIAL for Substrate)
    async fn fetch_blocks_concurrent(&self, numbers: Vec<u64>) -> Vec<Result<Block, NetworkError>> {
        // TODO: Use tokio::join! or futures::join_all
    }

    // Fetch with timeout (Substrate networking pattern)
    async fn fetch_with_timeout(&self, number: u64) -> Result<Block, NetworkError> {
        // TODO: Use tokio::time::timeout
    }
}
```

**2. BlockCache with Shared State** (15 min):
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

struct BlockCache {
    cache: Arc<Mutex<HashMap<u64, Block>>>,
    client: BlockchainClient,
}

impl BlockCache {
    fn new(client: BlockchainClient) -> Self {
        // TODO: Implement
    }

    // Get with cache (Substrate pattern)
    async fn get_block(&self, number: u64) -> Result<Block, NetworkError> {
        // TODO: 
        // 1. Check cache first
        // 2. If not found, fetch from network
        // 3. Save to cache
        // 4. Return result
    }

    // Background refresh (Substrate pattern)
    async fn refresh_cache_background(&self, numbers: Vec<u64>) {
        // TODO: Use tokio::spawn for background task
    }
}
```

### Requirements (Advanced - 15 minutes)

**3. Essential Tests**:
```rust
#[tokio::test]
async fn test_concurrent_faster_than_sequential() {
    // TODO: Demonstrate that concurrent is faster
}

#[tokio::test]
async fn test_cache_works() {
    // TODO: Verify cache prevents re-fetch
}

#[tokio::test]
async fn test_timeout_handling() {
    // TODO: Verify timeout functionality
}
```

### Suggested Implementation

**Fetch Block (Mock)**:
```rust
async fn fetch_block(&self, number: u64) -> Result<Block, NetworkError> {
    // Simulate network delay
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Simulate occasional error
    if number % 10 == 0 {
        return Err(NetworkError::new("Network error"));
    }
    
    Ok(Block {
        number,
        hash: format!("hash_{}", number),
        timestamp: number * 1000,
        transactions: vec![format!("tx_{}", number)],
    })
}
```

**Concurrent Fetch**:
```rust
async fn fetch_blocks_concurrent(&self, numbers: Vec<u64>) -> Vec<Result<Block, NetworkError>> {
    let futures: Vec<_> = numbers.into_iter()
        .map(|n| self.fetch_block(n))
        .collect();
    
    futures::future::join_all(futures).await
}
```

**Timeout**:
```rust
async fn fetch_with_timeout(&self, number: u64) -> Result<Block, NetworkError> {
    tokio::time::timeout(
        Duration::from_millis(self.timeout_ms),
        self.fetch_block(number)
    )
    .await
    .map_err(|_| NetworkError::new("Timeout"))?
}
```

### Expected Usage

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BlockchainClient::new("https://api.blockchain.com");
    
    // Concurrent fetching (Substrate pattern)
    let block_numbers = vec![1, 2, 3, 4, 5];
    let blocks = client.fetch_blocks_concurrent(block_numbers).await;
    
    // With caching
    let cache = BlockCache::new(client);
    let cached_block = cache.get_block(12345).await?;
    
    // Background refresh
    cache.refresh_cache_background(vec![1, 2, 3]).await;
    
    Ok(())
}
```

## Substrate/Polkadot Patterns Demonstrated

1. **Concurrent Operations**: `tokio::join!` for multiple operations
2. **Shared State**: `Arc<Mutex<T>>` for shared cache
3. **Background Tasks**: `tokio::spawn` for asynchronous work
4. **Timeout Handling**: Essential for P2P networking
5. **Error Propagation**: `?` operator in async context

## Tips for 45 minutes

1. **Focus on implementation**, not theory
2. **Use mocks** instead of real HTTP
3. **Test manually** before writing automated tests
4. **Prioritize**: Concurrent fetching > Cache > Timeout > Tests

## Key Learning Points

- **Tokio Runtime**: How to execute async code
- **Concurrency**: `join!` vs sequential await
- **Shared State**: Arc<Mutex<T>> for shared data
- **Error Handling**: Result<T, E> in async context
- **Background Tasks**: spawn for non-blocking work

These patterns are the foundation of networking, RPC, and block processing in the Polkadot SDK! 
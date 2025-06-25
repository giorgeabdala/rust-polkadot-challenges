use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{timeout, Instant};

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

#[derive(Clone)]
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


    async fn fetch_blocks_concurrent(&self, numbers: Vec<u64>) -> Vec<Result<Block, NetworkError>> {
        let futures: Vec<_> = numbers.into_iter()
            .map(|n| self.fetch_block(n))
            .collect();
        futures::future::join_all(futures).await
    }
    
    async fn fetch_with_timeout(&self, number: u64) -> Result<Block, NetworkError> {
       timeout(Duration::from_millis(self.timeout_ms),
       self.fetch_block(number)
       ).await
           .map_err(|_| NetworkError::new("Timeout"))?
    }
}

struct BlockCache {
    cache: Arc<Mutex<HashMap<u64, Block>>>,
    client: BlockchainClient,
}

impl BlockCache {
    fn new(client: BlockchainClient) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            client
        }
    }

    async fn get_block(&self, number: u64) -> Result<Block, NetworkError> {
        {
            let cache_guard = self.cache.lock().await; // Acquire a lock on the cache asynchronously
            if let Some(cached_block) = cache_guard.get(&number) { // Try to get the block from cache
                return Ok(cached_block.clone()); // If found, clone and return it immediately
            }
        } // The cache_guard goes out of scope here

        // 2. Fetch from the network if not in cache
        let block = self.client.fetch_block(number).await?;

        // 3. Save to cache
        {
            let mut cache_guard = self.cache.lock().await;
            cache_guard.insert(number, block.clone());
        }
        Ok(block)
    }

    async fn refresh_cache_background(&self, numbers: Vec<u64>) {
        let cache = self.cache.clone();
        let client = self.client.clone();
        let numbers_len = numbers.len();
        tokio::spawn(async move {
            println!("🔄 Starting background refresh for {} blocks", numbers.len());

            for number in numbers {
                match client.fetch_block(number).await {
                    Ok(block) => {
                        let mut cache_guard = cache.lock().await;
                        cache_guard.insert(number, block);
                        println!("Block {} cached in background", number);
                    }
                    Err(e) => {
                        println!("Error fetching block {} in background: {}", number, e.message);
                    }
                }
            }

            println!(" Background refresh complete!");
        });

        println!(" Background refresh initiated for {} blocks", numbers_len);
    }
}

mod tests {
    use tokio::time::Instant;
    use crate::medium::challenge_05::{Block, BlockchainClient, NetworkError};

    async fn fetch_blocks_sequencial(client: &BlockchainClient, numbers: &[u64])
        -> Vec<Result<Block, NetworkError>> {
        let mut results = Vec::new();
        for &number in numbers {
            results.push(client.fetch_block(number).await);
        }
        results
    }

    #[tokio::test]
    async fn test_concurrent_faster_than_sequential() {
       let client = BlockchainClient::new("mock://test");
        let block_numbers = vec![1,2,3,4,5];

        let start_sequential = Instant::now();
        let sequential_results = fetch_blocks_sequencial(&client, &block_numbers).await;
        let sequential_time = start_sequential.elapsed();

        let start = Instant::now();
        let concurrent_results = client.fetch_blocks_concurrent(block_numbers.clone()).await;
        let concurrent_time = start.elapsed();

        assert_eq!(sequential_results.len(), concurrent_results.len());
        assert!(concurrent_time < sequential_time);
        //With 5 blocks of 100ms each, we expect:
        // Sequential: ~500ms, Concurrent: ~100ms
        // So concurrent should be at least 3x faster
        assert!(concurrent_time.as_millis() * 3 < sequential_time.as_millis());

    }


    }

    #[tokio::test]
    async fn test_cache_works() {
        let client = BlockchainClient::new("mock://test");
        let mut cache = BlockCache::new(client);
        let block_number = 42;

        let start_first = Instant::now();
        let first_result = cache.get_block(block_number).await.expect("Block not found");
        let first_duration = start_first.elapsed();

        let start_second = Instant::now();
        let second_result = cache.get_block(block_number).await.expect("Block not found");
        let second_duration = start_second.elapsed();

        assert_eq!(first_result.number, second_result.number);
        //cache should be at least 10x faster
        assert!(second_duration.as_micros() * 10 < first_duration.as_micros());
    }

    #[tokio::test]
    async fn test_timeout_handling() {
        let mut client = BlockchainClient::new("mock://test");
        client.timeout_ms = 50; // Timeout menor que o delay de 100ms

        let start_fast = Instant::now();
        let fast_result = client.fetch_with_timeout(1).await;
        let fast_duration = start_fast.elapsed();

        assert!(fast_result.is_err()); // Agora deve dar timeout
        assert!(fast_duration.as_millis() >= 50); // Should wait for the timeout
        assert!(fast_duration.as_millis() < 150); // But not much more than that

        if let Err(error) = fast_result {
            assert!(error.message.contains("Timeout") || error.message.contains("timeout"));
        }
}

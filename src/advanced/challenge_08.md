## Challenge 8: Off-Chain Worker Simulator

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement a simplified off-chain worker system that demonstrates the core concepts of Substrate off-chain workers. This system will simulate data collection from external sources and processing without requiring complex blockchain dependencies.

**Main Concepts Covered:**
1. **Off-Chain Workers:** Background processing separate from consensus
2. **Data Collection:** Fetching external data sources
3. **Data Storage:** Simple caching mechanism
4. **Error Handling:** Robust failure management
5. **Worker Lifecycle:** Execution cycles and state management

### Detailed Structures to Implement:

#### **Data Structure:**
```rust
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// External data point collected by worker
#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub id: String,
    pub value: f64,
    pub timestamp: u64,
    pub source: String,
}

impl DataPoint {
    pub fn new(id: String, value: f64, source: String) -> Self {
        Self {
            id,
            value,
            timestamp: Self::current_timestamp(),
            source,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && 
        !self.source.is_empty() && 
        self.value.is_finite() &&
        self.timestamp > 0
    }
    
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
```

#### **Data Source Simulation:**
```rust
/// Trait for external data sources
pub trait DataSource {
    fn name(&self) -> &str;
    fn fetch_data(&mut self) -> Result<Vec<DataPoint>, DataSourceError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSourceError {
    NetworkError,
    InvalidData,
    Timeout,
}

/// Mock external API for testing
pub struct MockDataSource {
    name: String,
    data_counter: usize,
    should_fail: bool,
}

impl MockDataSource {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data_counter: 0,
            should_fail: false,
        }
    }
    
    pub fn with_failure(mut self, should_fail: bool) -> Self {
        self.should_fail = should_fail;
        self
    }
}

impl DataSource for MockDataSource {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn fetch_data(&mut self) -> Result<Vec<DataPoint>, DataSourceError> {
        if self.should_fail {
            return Err(DataSourceError::NetworkError);
        }
        
        self.data_counter += 1;
        
        let mut data_points = Vec::new();
        for i in 0..3 {
            data_points.push(DataPoint::new(
                format!("metric_{}", i),
                (self.data_counter as f64) * 10.0 + (i as f64),
                self.name.clone(),
            ));
        }
        
        Ok(data_points)
    }
}
```

#### **Simple Cache System:**
```rust
/// Simple cache for collected data
pub struct DataCache {
    data: HashMap<String, DataPoint>,
    max_entries: usize,
}

impl DataCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_entries,
        }
    }
    
    pub fn insert(&mut self, data_point: DataPoint) {
        // Simple eviction: remove oldest if at capacity
        if self.data.len() >= self.max_entries {
            if let Some(oldest_key) = self.data
                .iter()
                .min_by_key(|(_, point)| point.timestamp)
                .map(|(key, _)| key.clone())
            {
                self.data.remove(&oldest_key);
            }
        }
        
        self.data.insert(data_point.id.clone(), data_point);
    }
    
    pub fn get(&self, id: &str) -> Option<&DataPoint> {
        self.data.get(id)
    }
    
    pub fn get_all(&self) -> Vec<&DataPoint> {
        self.data.values().collect()
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
```

#### **Off-Chain Worker:**
```rust
/// Main off-chain worker implementation
pub struct OffChainWorker {
    sources: Vec<Box<dyn DataSource>>,
    cache: DataCache,
    stats: WorkerStats,
}

#[derive(Debug, Clone, Default)]
pub struct WorkerStats {
    pub total_executions: usize,
    pub successful_fetches: usize,
    pub failed_fetches: usize,
    pub cached_items: usize,
}

impl OffChainWorker {
    pub fn new(cache_size: usize) -> Self {
        Self {
            sources: Vec::new(),
            cache: DataCache::new(cache_size),
            stats: WorkerStats::default(),
        }
    }
    
    /// Add a data source to the worker
    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        self.sources.push(source);
    }
    
    /// Execute one worker cycle
    pub fn execute(&mut self) -> Result<(), String> {
        self.stats.total_executions += 1;
        
        for source in &mut self.sources {
            match source.fetch_data() {
                Ok(data_points) => {
                    self.stats.successful_fetches += 1;
                    self.process_data(data_points);
                },
                Err(error) => {
                    self.stats.failed_fetches += 1;
                    eprintln!("Failed to fetch from {}: {:?}", source.name(), error);
                }
            }
        }
        
        self.stats.cached_items = self.cache.size();
        Ok(())
    }
    
    fn process_data(&mut self, data_points: Vec<DataPoint>) {
        for data_point in data_points {
            if data_point.is_valid() {
                self.cache.insert(data_point);
            }
        }
    }
    
    /// Get cached data by ID
    pub fn get_data(&self, id: &str) -> Option<&DataPoint> {
        self.cache.get(id)
    }
    
    /// Get all cached data
    pub fn get_all_data(&self) -> Vec<&DataPoint> {
        self.cache.get_all()
    }
    
    /// Get worker statistics
    pub fn get_stats(&self) -> &WorkerStats {
        &self.stats
    }
    
    /// Clear all cached data
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Check if worker has data from specific source
    pub fn has_data_from_source(&self, source_name: &str) -> bool {
        self.cache.get_all()
            .iter()
            .any(|data_point| data_point.source == source_name)
    }
}
```

#### **Worker Manager:**
```rust
/// Manages multiple worker executions
pub struct WorkerManager {
    worker: OffChainWorker,
    execution_count: usize,
}

impl WorkerManager {
    pub fn new(worker: OffChainWorker) -> Self {
        Self {
            worker,
            execution_count: 0,
        }
    }
    
    /// Run worker for specified number of cycles
    pub fn run_cycles(&mut self, cycles: usize) -> Result<WorkerStats, String> {
        for _ in 0..cycles {
            self.worker.execute()?;
            self.execution_count += 1;
        }
        
        Ok(self.worker.get_stats().clone())
    }
    
    /// Get total execution count
    pub fn execution_count(&self) -> usize {
        self.execution_count
    }
    
    /// Get access to the worker
    pub fn worker(&self) -> &OffChainWorker {
        &self.worker
    }
    
    /// Get mutable access to the worker
    pub fn worker_mut(&mut self) -> &mut OffChainWorker {
        &mut self.worker
    }
}
```

#### **Simple Oracle Data Source:**
```rust
/// Oracle-like data source for blockchain data
pub struct OracleDataSource {
    name: String,
    price_base: f64,
    price_counter: usize,
}

impl OracleDataSource {
    pub fn new(name: String, base_price: f64) -> Self {
        Self {
            name,
            price_base: base_price,
            price_counter: 0,
        }
    }
}

impl DataSource for OracleDataSource {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn fetch_data(&mut self) -> Result<Vec<DataPoint>, DataSourceError> {
        self.price_counter += 1;
        
        // Simulate price fluctuation
        let price_variation = (self.price_counter as f64 * 0.1).sin() * 50.0;
        let current_price = self.price_base + price_variation;
        
        let data_point = DataPoint::new(
            "price".to_string(),
            current_price,
            self.name.clone(),
        );
        
        Ok(vec![data_point])
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Data Collection:**
   - Test successful data fetching from sources
   - Test error handling for failed sources
   - Test data validation

2. **Cache Management:**
   - Test data insertion and retrieval
   - Test cache size limits
   - Test data eviction

3. **Worker Execution:**
   - Test single execution cycle
   - Test multiple execution cycles
   - Test statistics collection

4. **Integration:**
   - Test worker with multiple sources
   - Test worker manager functionality
   - Test oracle data source

### Expected Output

A complete off-chain worker system that:
- Collects data from simulated external sources
- Caches processed data efficiently
- Handles errors gracefully
- Provides execution statistics
- Demonstrates core off-chain worker concepts

### Theoretical Context

**Off-Chain Workers in Substrate:**
- **Purpose:** Execute logic outside of consensus without blocking block production
- **Use Cases:** Oracle data, heavy computations, external API calls
- **Architecture:** Separate execution environment from runtime
- **Data Flow:** Fetch → Process → Store/Submit
- **Benefits:** Scalability, external integration, resource efficiency

**Key Concepts:**
- **Asynchronous Execution:** Workers run independently of block production
- **External Data Access:** Ability to call external APIs and services
- **State Management:** Local storage and caching mechanisms
- **Error Resilience:** Handling network failures and data inconsistencies
- **Resource Isolation:** Separate from consensus-critical operations

**Best Practices:**
- Keep worker logic simple and focused
- Implement proper error handling and retries
- Use efficient caching strategies
- Monitor worker performance and health
- Ensure data validation before processing

This system demonstrates the fundamental patterns used in Substrate off-chain workers while remaining simple enough to implement and understand quickly.
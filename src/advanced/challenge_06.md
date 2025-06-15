## Challenge 6: Asynchronous Worker Simulator for Data Collection

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement an asynchronous worker simulator in pure Rust that demonstrates the fundamental concepts of Substrate off-chain workers. This system will collect data from simulated sources and process it asynchronously, without requiring external Substrate dependencies.

The simulator should allow:
- Periodic execution of data collection tasks
- Asynchronous processing of collected data
- Cache system for processed data
- Data validation and filtering
- Event system for notifications

**Main Concepts Covered:**
1. **Asynchronous Workers:** Background processing
2. **Data Collection:** Simulation of external APIs
3. **Data Caching:** Efficient temporary storage
4. **Validation:** Data integrity verification
5. **Periodic Processing:** Execution at regular intervals

### Detailed Structures to Implement:

#### **Data Structure:**
```rust
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
    
    pub fn is_recent(&self, max_age_seconds: u64) -> bool {
        let current = Self::current_timestamp();
        current.saturating_sub(self.timestamp) <= max_age_seconds
    }
    
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
```

#### **Simulated Data Source:**
```rust
pub trait DataSource {
    fn name(&self) -> &str;
    fn fetch_data(&mut self) -> Result<Vec<DataPoint>, DataSourceError>;
    fn is_available(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSourceError {
    Unavailable,
    InvalidData,
    Timeout,
    NetworkError,
}

/// External data source simulator
pub struct MockDataSource {
    name: String,
    data_points: Vec<DataPoint>,
    failure_rate: f64, // 0.0 = never fails, 1.0 = always fails
    call_count: usize,
}

impl MockDataSource {
    pub fn new(name: String) -> Self {
        let mut data_points = Vec::new();
        
        // Simulate some data
        for i in 0..10 {
            data_points.push(DataPoint::new(
                format!("metric_{}", i),
                (i as f64) * 10.5 + 100.0,
                name.clone(),
            ));
        }
        
        Self {
            name,
            data_points,
            failure_rate: 0.1, // 10% chance of failure
            call_count: 0,
        }
    }
    
    pub fn with_failure_rate(mut self, rate: f64) -> Self {
        self.failure_rate = rate.clamp(0.0, 1.0);
        self
    }
    
    pub fn add_data_point(&mut self, data_point: DataPoint) {
        self.data_points.push(data_point);
    }
    
    pub fn call_count(&self) -> usize {
        self.call_count
    }
}

impl DataSource for MockDataSource {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn fetch_data(&mut self) -> Result<Vec<DataPoint>, DataSourceError> {
        self.call_count += 1;
        
        // Simulate occasional failures
        if self.failure_rate > 0.0 {
            let random_value = (self.call_count as f64 * 0.1) % 1.0;
            if random_value < self.failure_rate {
                return Err(DataSourceError::NetworkError);
            }
        }
        
        // Simulate data variation
        let mut result = Vec::new();
        for (i, base_point) in self.data_points.iter().enumerate() {
            let variation = ((self.call_count + i) as f64 * 0.1).sin() * 5.0;
            let mut point = base_point.clone();
            point.value += variation;
            point.timestamp = DataPoint::current_timestamp();
            result.push(point);
        }
        
        Ok(result)
    }
    
    fn is_available(&self) -> bool {
        true
    }
}
```

#### **Cache System:**
```rust
pub struct DataCache {
    cache: HashMap<String, DataPoint>,
    max_age_seconds: u64,
    max_entries: usize,
}

impl DataCache {
    pub fn new(max_age_seconds: u64, max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_age_seconds,
            max_entries,
        }
    }
    
    pub fn insert(&mut self, data_point: DataPoint) {
        // Clean cache if necessary
        self.cleanup_expired();
        
        // Limit number of entries
        if self.cache.len() >= self.max_entries {
            self.remove_oldest();
        }
        
        self.cache.insert(data_point.id.clone(), data_point);
    }
    
    pub fn get(&self, id: &str) -> Option<&DataPoint> {
        self.cache.get(id).filter(|point| point.is_recent(self.max_age_seconds))
    }
    
    pub fn get_all_valid(&self) -> Vec<&DataPoint> {
        self.cache
            .values()
            .filter(|point| point.is_recent(self.max_age_seconds))
            .collect()
    }
    
    pub fn cleanup_expired(&mut self) {
        let current_time = DataPoint::current_timestamp();
        self.cache.retain(|_, point| {
            current_time.saturating_sub(point.timestamp) <= self.max_age_seconds
        });
    }
    
    fn remove_oldest(&mut self) {
        if let Some(oldest_key) = self.cache
            .iter()
            .min_by_key(|(_, point)| point.timestamp)
            .map(|(key, _)| key.clone())
        {
            self.cache.remove(&oldest_key);
        }
    }
    
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
```

#### **Asynchronous Worker:**
```rust
pub struct AsyncWorker {
    sources: Vec<Box<dyn DataSource>>,
    cache: DataCache,
    events: Vec<WorkerEvent>,
    config: WorkerConfig,
    stats: WorkerStats,
}

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub max_data_age_seconds: u64,
    pub cache_max_entries: usize,
    pub batch_size: usize,
    pub retry_attempts: usize,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            max_data_age_seconds: 300, // 5 minutes
            cache_max_entries: 1000,
            batch_size: 10,
            retry_attempts: 3,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkerStats {
    pub total_fetches: usize,
    pub successful_fetches: usize,
    pub failed_fetches: usize,
    pub cached_items: usize,
    pub processed_items: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorkerEvent {
    DataFetched { source: String, count: usize },
    DataCached { id: String, source: String },
    FetchFailed { source: String, error: DataSourceError },
    CacheCleanup { removed_count: usize },
    WorkerExecuted { duration_ms: u64 },
}

impl AsyncWorker {
    pub fn new(config: WorkerConfig) -> Self {
        Self {
            sources: Vec::new(),
            cache: DataCache::new(config.max_data_age_seconds, config.cache_max_entries),
            events: Vec::new(),
            config,
            stats: WorkerStats::default(),
        }
    }
    
    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        self.sources.push(source);
    }
    
    /// Execute complete worker cycle
    pub fn execute_cycle(&mut self) -> Result<WorkerStats, String> {
        let start_time = std::time::Instant::now();
        
        // Clean expired cache
        let initial_cache_size = self.cache.size();
        self.cache.cleanup_expired();
        let cleaned_count = initial_cache_size.saturating_sub(self.cache.size());
        
        if cleaned_count > 0 {
            self.events.push(WorkerEvent::CacheCleanup { 
                removed_count: cleaned_count 
            });
        }
        
        // Collect data from all sources
        for source in &mut self.sources {
            self.fetch_from_source(source.as_mut());
        }
        
        // Update statistics
        self.stats.cached_items = self.cache.size();
        
        let duration = start_time.elapsed();
        self.events.push(WorkerEvent::WorkerExecuted { 
            duration_ms: duration.as_millis() as u64 
        });
        
        Ok(self.stats.clone())
    }
    
    fn fetch_from_source(&mut self, source: &mut dyn DataSource) {
        self.stats.total_fetches += 1;
        
        let mut attempts = 0;
        while attempts < self.config.retry_attempts {
            match source.fetch_data() {
                Ok(data_points) => {
                    self.stats.successful_fetches += 1;
                    self.process_data_points(data_points, source.name());
                    
                    self.events.push(WorkerEvent::DataFetched { 
                        source: source.name().to_string(),
                        count: data_points.len(),
                    });
                    return;
                },
                Err(error) => {
                    attempts += 1;
                    if attempts >= self.config.retry_attempts {
                        self.stats.failed_fetches += 1;
                        self.events.push(WorkerEvent::FetchFailed { 
                            source: source.name().to_string(),
                            error,
                        });
                    }
                }
            }
        }
    }
    
    fn process_data_points(&mut self, data_points: Vec<DataPoint>, source_name: &str) {
        for data_point in data_points {
            if data_point.is_valid() {
                self.events.push(WorkerEvent::DataCached { 
                    id: data_point.id.clone(),
                    source: source_name.to_string(),
                });
                
                self.cache.insert(data_point);
                self.stats.processed_items += 1;
            }
        }
    }
    
    /// Get data from cache
    pub fn get_cached_data(&self, id: &str) -> Option<&DataPoint> {
        self.cache.get(id)
    }
    
    /// Get all valid data from cache
    pub fn get_all_cached_data(&self) -> Vec<&DataPoint> {
        self.cache.get_all_valid()
    }
    
    /// Get emitted events
    pub fn get_events(&self) -> &[WorkerEvent] {
        &self.events
    }
    
    /// Clear events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> &WorkerStats {
        &self.stats
    }
    
    /// Check if there is recent data from a source
    pub fn has_recent_data_from_source(&self, source: &str) -> bool {
        self.cache.get_all_valid()
            .iter()
            .any(|point| point.source == source)
    }
}
```

### Implementation Requirements:

1. **Collection System:**
   - Implement multiple simulated data sources
   - Manage failures and retry logic
   - Process data in batches

2. **Smart Cache:**
   - Automatic expiration of old data
   - Memory limits
   - Efficient cleanup

3. **Asynchronous Processing:**
   - Simulated periodic execution
   - Robust error handling
   - Performance statistics

### Test Configuration:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_worker() -> AsyncWorker {
        let config = WorkerConfig {
            max_data_age_seconds: 60,
            cache_max_entries: 100,
            batch_size: 5,
            retry_attempts: 2,
        };
        
        let mut worker = AsyncWorker::new(config);
        
        // Add test sources
        let source1 = Box::new(MockDataSource::new("test_source_1".to_string()));
        let source2 = Box::new(MockDataSource::new("test_source_2".to_string())
            .with_failure_rate(0.3));
        
        worker.add_source(source1);
        worker.add_source(source2);
        
        worker
    }

    #[test]
    fn worker_executes_successfully() {
        let mut worker = create_test_worker();
        
        let stats = worker.execute_cycle().unwrap();
        
        assert!(stats.total_fetches > 0);
        assert!(stats.processed_items > 0);
        assert!(worker.get_all_cached_data().len() > 0);
    }

    #[test]
    fn cache_expires_old_data() {
        // Cache expiration test
        let mut cache = DataCache::new(1, 100); // 1 second TTL
        
        let data_point = DataPoint::new(
            "test".to_string(),
            42.0,
            "test_source".to_string(),
        );
        
        cache.insert(data_point);
        assert!(cache.get("test").is_some());
        
        // Simulate time passage
        std::thread::sleep(std::time::Duration::from_secs(2));
        cache.cleanup_expired();
        
        assert!(cache.get("test").is_none());
    }

    // Add more tests here...
}
```

### Tests

Create a test module with the following scenarios:
- **Successful execution:** Verify worker collects and processes data
- **Failure handling:** Verify retry logic and error handling
- **Functional cache:** Verify expiration and data cleanup
- **Multiple sources:** Verify processing of various sources
- **Statistics:** Verify metrics are collected correctly
- **Events:** Verify events are emitted appropriately

### Expected Output

A complete asynchronous worker simulator implementation that:
- Compiles without errors
- Passes all unit tests
- Demonstrates off-chain worker concepts without external dependencies
- Manages data efficiently with cache
- Provides detailed statistics and events

### Theoretical Context

This challenge simulates fundamental concepts of Substrate off-chain workers:

- **Asynchronous Processing:** Workers execute in background without blocking the blockchain
- **External Data Collection:** Integration with APIs and external data sources
- **Cache and Performance:** Optimization of access to frequently used data
- **Failure Handling:** Robustness against network failures and unavailable sources
- **Monitoring:** Collection of metrics and events for observability

This foundation prepares for understanding how real off-chain workers function in Substrate, but without the complexity of blockchain environment configuration.

**Advantages of This Approach:**
- Focus on fundamental concepts without blockchain overhead
- Pure Rust with only standard libraries
- Testable and iterable quickly
- Demonstrates asynchronous design patterns
- Prepares for real off-chain worker implementations
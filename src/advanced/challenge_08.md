## Challenge 8: Simplified Off-Chain Worker

**Difficulty Level:** Advanced
**Estimated Time:** 1 hour

### Objective Description

You will implement a simplified off-chain worker system that demonstrates the core concepts of Substrate off-chain workers. This system will simulate data collection from external sources and basic processing without complex blockchain dependencies.

### Main Concepts Covered

1. **Off-Chain Workers**: Background processing separate from consensus
2. **Data Collection**: Fetching external data sources
3. **Data Storage**: Simple caching mechanism
4. **Error Handling**: Basic failure management
5. **Worker Patterns**: Core execution patterns used in Substrate

### Structures to Implement

#### **External Data Point:**
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
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
            
        Self {
            id,
            value,
            timestamp,
            source,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && self.value.is_finite()
    }
}
```

#### **Data Source Abstraction:**
```rust
/// Trait for external data sources
pub trait DataSource {
    fn name(&self) -> &str;
    fn fetch_data(&mut self) -> Result<DataPoint, String>;
}

/// Mock external API for testing and simulation
pub struct MockDataSource {
    name: String,
    counter: usize,
    should_fail: bool,
}

impl MockDataSource {
    pub fn new(name: String) -> Self {
        Self {
            name,
            counter: 0,
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
    
    fn fetch_data(&mut self) -> Result<DataPoint, String> {
        if self.should_fail {
            return Err("Mock failure".to_string());
        }
        
        self.counter += 1;
        let data_point = DataPoint::new(
            format!("{}_{}", self.name, self.counter), // ID único por fonte
            (self.counter as f64) * 10.0,
            self.name.clone(),
        );
        
        Ok(data_point)
    }
}
```

#### **Simple Cache System:**
```rust
/// Simple cache for collected data
pub struct DataCache {
    data: HashMap<String, DataPoint>,
}

impl DataCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, data_point: DataPoint) {
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
}
```

#### **Off-Chain Worker:**
```rust
/// Main off-chain worker implementation
pub struct OffChainWorker {
    sources: Vec<Box<dyn DataSource>>,
    cache: DataCache,
    execution_count: usize,
}

impl OffChainWorker {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            cache: DataCache::new(),
            execution_count: 0,
        }
    }
    
    /// Add a data source to the worker
    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        self.sources.push(source);
    }
    
    /// Get cached data by ID
    pub fn get_data(&self, id: &str) -> Option<&DataPoint> {
        self.cache.get(id)
    }
    
    /// Get execution count
    pub fn executions(&self) -> usize {
        self.execution_count
    }
    
    /// Get total cached items
    pub fn cached_items(&self) -> usize {
        self.cache.size()
    }
}
```

### **Design Note: Unique Data IDs**

**Important:** Notice that the `fetch_data()` implementation generates unique IDs using the pattern `format!("{}_{}", self.name, self.counter)`. This is crucial because:

1. **Cache Collision Prevention**: Without unique IDs, data from different sources would overwrite each other in the HashMap
2. **Data Accumulation**: Multiple executions should accumulate data, not replace it
3. **Source Identification**: Each data point can be traced back to its specific source
4. **Real-World Pattern**: This mirrors how actual off-chain workers handle data from multiple APIs

**Example ID Generation:**
- Source "CoinGecko": generates `CoinGecko_1`, `CoinGecko_2`, `CoinGecko_3`...
- Source "Binance": generates `Binance_1`, `Binance_2`, `Binance_3`...
- Result: All IDs are unique, no data loss occurs

### Methods for You to Implement

#### **1. Execute Worker Cycle (`execute`):**
```rust
impl OffChainWorker {
    /// Execute one worker cycle
    // TODO: Implement this method
    pub fn execute(&mut self) -> Result<usize, String> {
        // IMPLEMENT:
        // 1. Increment execution_count
        // 2. Initialize successful_fetches counter to 0
        // 3. Iterate through all sources
        // 4. For each source, try to fetch_data()
        // 5. On success: validate data with is_valid() and cache if valid
        // 6. On error: continue to next source (don't fail entire execution)
        // 7. Increment successful_fetches for each successful and valid fetch
        // 8. Return Ok(successful_fetches)
        todo!()
    }
}
```

### Tests to Implement

Create tests that cover:

#### **Test Scenarios:**

1. **Basic Functionality:**
   - Worker creation and source addition
   - Successful data fetching and caching
   - Execution count tracking

2. **Error Handling:**
   - Worker continues despite source failures
   - Invalid data is not cached
   - Successful sources work despite failed ones

3. **Data Management:**
   - Cache stores and retrieves data correctly
   - Multiple executions accumulate data
   - Data validation works properly

### Example Usage

```rust
fn main() {
    let mut worker = OffChainWorker::new();
    
    // Add data sources
    let source1 = Box::new(MockDataSource::new("API-1".to_string()));
    let source2 = Box::new(MockDataSource::new("API-2".to_string()).with_failure(true));
    
    worker.add_source(source1);
    worker.add_source(source2);
    
    // Execute worker
    match worker.execute() {
        Ok(successful_fetches) => {
            println!("Successfully fetched {} data points", successful_fetches);
            println!("Total executions: {}", worker.executions());
            println!("Cached items: {}", worker.cached_items());
        }
        Err(e) => println!("Worker failed: {}", e),
    }
    
    // Retrieve cached data
    if let Some(data) = worker.get_data("API-1_1") {
        println!("Found data: {:?}", data);
    }
}
```

### Expected Output

A simplified off-chain worker system that:
- Demonstrates core off-chain worker concepts
- Handles data collection from multiple sources
- Implements basic error resilience
- Shows essential patterns used in Substrate off-chain workers

### Theoretical Context

**Off-Chain Workers in Substrate:**
- **Purpose**: Execute tasks that can't or shouldn't be done on-chain
- **Separation**: Run separately from consensus to avoid blocking
- **Data Flow**: Fetch → Validate → Process → Submit (when needed)
- **Error Isolation**: Failures don't crash the blockchain

**Core Problem They Solve:**
Blockchains are deterministic and consensus-driven, but real applications need:
- **External Data**: Price feeds, weather data, API responses
- **Heavy Computation**: Processing without blocking block production
- **Asynchronous Tasks**: Background work independent of transactions

**Key Design Patterns:**
1. **Fetch → Validate → Cache**: Standard data collection flow
2. **Error Resilience**: Continue working despite individual failures
3. **State Separation**: Off-chain state vs on-chain state
4. **Periodic Execution**: Regular cycles independent of user actions

**Real-World Applications:**
- **DeFi Oracles**: Price feed aggregation for trading protocols
- **IoT Integration**: Collecting and processing sensor data
- **Cross-Chain Bridges**: Monitoring other blockchains for events

**Substrate Integration:**
In production Substrate applications, off-chain workers:
- Access local storage for data persistence
- Submit signed transactions back to the chain
- Coordinate with on-chain pallets through extrinsics
- Run in dedicated threads separate from block production

This challenge provides the conceptual foundation for working with real Substrate off-chain workers in production environments.
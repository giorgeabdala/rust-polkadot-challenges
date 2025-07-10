use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub trait DataSource {
    fn name(&self) -> &str;
    fn fetch_data(&mut self) -> Result<DataPoint, String>;
}

#[derive(PartialEq, Eq, PartialOrd)]
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
            format!("{}_{}", self.name, self.counter), // ✅ ID único por fonte
            (self.counter as f64) * 10.0,
            self.name.clone(),
        );

        Ok(data_point)
    }
}

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

    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        self.sources.push(source);
    }

    pub fn get_data(&self, id: &str) -> Option<&DataPoint> {
        self.cache.get(id)
    }

    pub fn executions(&self) -> usize {
        self.execution_count
    }

    pub fn cached_items(&self) -> usize {
        self.cache.size()
    }

    pub fn execute(&mut self) -> Result<usize, String> {
        self.execution_count += 1;
        let mut successful_fetches = 0;
        for source in self.sources.iter_mut()  {
            let result = source.fetch_data();
            
            if let Ok(datapoint) = result {
                if datapoint.is_valid() {
                    self.cache.insert(datapoint);
                    successful_fetches+=1;
                }
            }
            
  }
        Ok(successful_fetches)
    }
}


#[cfg(test)]

mod tests {
    use crate::advanced::challenge_08::{DataPoint, DataSource, MockDataSource, OffChainWorker};

    #[test]
    fn test_successful_execution_fetches_and_caches_data() {
        let mut worker = OffChainWorker::new();

        let source1  = Box::new(MockDataSource::new("API-1".to_string()));
        let source2  = Box::new(MockDataSource::new("API-2".to_string()));

        worker.add_source(source1);
        assert_eq!(worker.sources.len(), 1);
        assert_eq!(worker.sources.get(0).unwrap().name(), "API-1");
        worker.add_source(source2);
        assert_eq!(worker.sources.len(), 2);
        assert_eq!(worker.sources.get(1).unwrap().name(), "API-2");

        let result = worker.execute();
        assert_eq!(result, Ok(2));
        assert_eq!(worker.executions(), 1);
        assert_eq!(worker.cached_items(), 2); // ✅ Agora 2 itens únicos
        let data_point_opt = worker.get_data("API-1_1");
        assert!(data_point_opt.is_some());
        assert_eq!(data_point_opt.unwrap().value, 10f64);
        let data_point_opt = worker.get_data("API-2_1");
        assert!(data_point_opt.is_some());
        assert_eq!(data_point_opt.unwrap().value, 10f64); // ✅ Ambas começam com counter=1

        let result = worker.execute();
        assert_eq!(result, Ok(2));
        assert_eq!(worker.executions(), 2);
        assert_eq!(worker.cached_items(), 4); // ✅ Agora 4 itens únicos
    }

    #[test]
    fn test_worker_handles_mixed_success_and_failure_sources() {
        let mut worker = OffChainWorker::new();
        let source1  = Box::new(MockDataSource::new("API-1".to_string()));
        let source2  = Box::new(MockDataSource::new("API-2".to_string()));
        let source_fail = Box::new(MockDataSource::new("API-Fail".to_string()).with_failure(true));

        worker.add_source(source1);
        worker.add_source(source2);
        worker.add_source(source_fail);

        let result = worker.execute();
        assert_eq!(result, Ok(2));
        assert_eq!(worker.executions(), 1);
        assert_eq!(worker.cached_items(), 2);
        
        assert!(worker.get_data("API-1_1").is_some());
        assert!(worker.get_data("API-2_1").is_some());
        
   }
    
    #[test]
    fn test_worker_ignores_invalid_data() {
        
        struct InvalidDataSource;
        
        impl DataSource for InvalidDataSource {
            fn name(&self) -> &str { "Invalid-Source" }
            fn fetch_data(&mut self) -> Result<DataPoint, String> {
                Ok(DataPoint::new("".to_string(), 123.0, self.name().to_string()))
            }
        }
        
        let mut worker = OffChainWorker::new();
        worker.add_source(Box::new(InvalidDataSource));
        
        let result = worker.execute();
        assert_eq!(result, Ok(0));
        assert_eq!(worker.cached_items(), 0);
    }
    

}

// Exemplo de uso demonstrando o sistema funcionando
#[allow(dead_code)]
fn exemplo_de_uso() {
    println!("=== Off-Chain Worker Demonstration ===");
    
    let mut worker = OffChainWorker::new();
    
    // Add data sources
    let source1 = Box::new(MockDataSource::new("CoinGecko".to_string()));
    let source2 = Box::new(MockDataSource::new("Binance".to_string()));
    let source3 = Box::new(MockDataSource::new("Kraken".to_string()).with_failure(true));
    
    worker.add_source(source1);
    worker.add_source(source2);
    worker.add_source(source3);
    
    println!("Sources added: 3 (1 with simulated failure)");
    
    // First execution
    match worker.execute() {
        Ok(successful_fetches) => {
            println!("✅ First execution:");
            println!("  - Successful fetches: {}", successful_fetches);
            println!("  - Total executions: {}", worker.executions());
            println!("  - Cached items: {}", worker.cached_items());
        }
        Err(e) => println!("❌ First execution failure: {}", e),
    }
    
    // Second execution
    match worker.execute() {
        Ok(successful_fetches) => {
            println!("✅ Second execution:");
            println!("  - Successful fetches: {}", successful_fetches);
            println!("  - Total executions: {}", worker.executions());
            println!("  - Cached items: {}", worker.cached_items());
        }
        Err(e) => println!("❌ Second execution failure: {}", e),
    }
    
    // Display some collected data
    println!("\n📊 Collected data:");
    if let Some(data) = worker.get_data("CoinGecko_1") {
        println!("  - {}: {} (timestamp: {})", data.id, data.value, data.timestamp);
    }
    if let Some(data) = worker.get_data("Binance_2") {
        println!("  - {}: {} (timestamp: {})", data.id, data.value, data.timestamp);
    }
    
    println!("\n🎯 System demonstrates:");
    println!("  ✓ Data collection from multiple sources");
    println!("  ✓ Fault resistance (continues even with source failing)");
    println!("  ✓ Cache with unique IDs (no overlap)");
    println!("  ✓ Rastreamento de execuções");
}

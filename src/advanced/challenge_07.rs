use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct InherentData {
    data: HashMap<String, Vec<u8>>,
}

impl InherentData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn put_data(&mut self, identifier: &str, data: Vec<u8>) {
        self.data.insert(identifier.to_string(), data);
    }
    pub fn get_data(&self, identifier: &str) -> Option<&Vec<u8>> {
        self.data.get(identifier)
    }
    
    pub fn has_data(&self, identifier: &str) -> bool {
        self.data.contains_key(identifier)
    }
    pub fn identifiers(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

pub trait InherentDataProvider {
    fn get_identifier(&self) -> &'static str;
    
    fn provide_inherent_data(&self) -> Result<InherentData, &'static str>;
    
    fn is_required(&self) -> bool;
    
    fn error_message(&self) -> &'static str;
}

// Timestamp data structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Timestamp {
    /// Milliseconds since Unix epoch
    pub millis: u64,
}

impl Timestamp {
    pub fn now() -> Self {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self { millis }
    }
    pub fn from_millis(millis: u64) -> Self {
        Self { millis }
    }
    pub fn to_string(&self) -> String {
        self.millis.to_string()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.millis.to_le_bytes().to_vec()
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 8 {
            return Err("Invalid timestamp bytes length");
        }
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        let millis = u64::from_le_bytes(array);
        Ok(Self::from_millis(millis))
    }
}
pub struct TimestampProvider {
    custom_timestamp: Option<Timestamp>,
}

impl TimestampProvider {
    pub const INHERENT_IDENTIFIER: &'static str = "timestamp";
    pub fn new() -> Self {
        Self {
            custom_timestamp: None,
        }
    }
    pub fn with_custom_timestamp(mut self, timestamp: Timestamp) -> Self {
        self.custom_timestamp = Some(timestamp);
        self
    }
    
    fn get_timestamp(&self) -> Timestamp {
        self.custom_timestamp.unwrap_or_else(Timestamp::now)
    }
}

impl InherentDataProvider for TimestampProvider {
    fn get_identifier(&self) -> &'static str {
        "timestamp"
    }

    fn provide_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut inherent_data = InherentData::new();
        let timestamp = self.get_timestamp();
        inherent_data.put_data(Self::INHERENT_IDENTIFIER, timestamp.to_bytes());
        Ok(inherent_data)
    }

    fn is_required(&self) -> bool {
        true
    }

    fn error_message(&self) -> &'static str {
        "Required timestamp not found."
    }
}


pub struct BlockConstructor {
    providers: Vec<Box<dyn InherentDataProvider>>,
    block_number: u64,
}



impl BlockConstructor {
    pub fn new(block_number: u64) -> Self {
        Self {
            providers: Vec::new(),
            block_number
        }
    }

    pub fn collect_inherent_data(&self) -> Result<InherentData, &'static str> {
        let mut combined_data = InherentData::new();
        
        for provider in self.providers.iter() {
            let provider_data = provider.provide_inherent_data()?;
            for (identifier, data_value) in provider_data.data {
                if combined_data.has_data(&identifier) {
                    return Err("Duplicate inherent identifier found.");
                }
                combined_data.put_data(&identifier, data_value);
            }
        }
        Ok(combined_data)
    }


    // Versão Aprimorada
    pub fn validate_inherents(&self, inherent_data: &InherentData) -> Result<(), &'static str> {
        for provider in self.providers.iter() {
            if provider.is_required() && !inherent_data.has_data(provider.get_identifier()) {
                return Err(provider.error_message());
            }
        }

        if let Some(timestamp_bytes) = inherent_data.get_data(TimestampProvider::INHERENT_IDENTIFIER) {
            let timestamp = Timestamp::from_bytes(timestamp_bytes)?;
            if timestamp.millis == 0 {
                return Err("Invalid Timestamp: cannot be zero.");
            }
            
            let now = Timestamp::now().millis;
            const MAX_DRIFT_MS: u64 = 5000;

            if timestamp.millis > now + MAX_DRIFT_MS {
                return Err("Timestamp is too far in the future.");
            }
        }
        Ok(())
    }


    // Versão Refatorada
    pub fn build_block(&self) -> Result<Block, &'static str> {
        let inherent_data = self.collect_inherent_data()?;
        self.validate_inherents(&inherent_data)?;

        let timestamp_bytes = inherent_data
            .get_data(TimestampProvider::INHERENT_IDENTIFIER)
            .unwrap();
        let timestamp = Timestamp::from_bytes(timestamp_bytes)?.millis;

        let block = Block {
            block_number: self.block_number,
            inherent_data,
            timestamp,
        };

        block.validate()?;
        Ok(block)
    }


    pub fn register_provider(&mut self, provider: Box<dyn InherentDataProvider>) {
        self.providers.push(provider);
    }

    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

}

#[derive(Debug, Clone)]
pub struct Block {
    pub block_number: u64,
    pub inherent_data: InherentData,
    pub timestamp: u64
}

impl Block {
    pub fn get_inherent_timestamp(&self) -> Result<Option<Timestamp>, &'static str> {
        if let Some(timestamp_byte) = self.inherent_data.get_data("timestamp") {
            let timestamp = Timestamp::from_bytes(timestamp_byte)?;
            Ok(Some(timestamp))
        } else {
            Ok(None)
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.block_number == 0 {
            return Err("Block number cannot be zero");
        }
        if self.timestamp == 0 {
            return Err("Block timestamp cannot be zero");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_inherent_data() -> (InherentData, &'static str, Vec<u8>){
        let key  = "key";
        let data = vec![1,2,3,4];
        let mut inherent_data = InherentData::new();
        inherent_data.put_data(key, data.clone() );
        (inherent_data, key, data)
    }

    #[test]
    fn test_basic_functionality() {
        let (inherent_data, key, data) = create_inherent_data();

        let has_opt = inherent_data.has_data(key);
        assert!(has_opt);
        
        let data_opt = inherent_data.get_data(key);
        assert!(data_opt.is_some());
        assert_eq!(*data_opt.unwrap(), data);
        
        let identifiers = inherent_data.identifiers();
        assert_eq!(identifiers.len(), 1);
        assert!(identifiers.contains(&key.to_string()));
    }

    #[test]
    fn test_timestamp_provider() {
        let timestamp = Timestamp::now();
        let provider = TimestampProvider::new().with_custom_timestamp(timestamp);
        let inherent_result = provider.provide_inherent_data();
        assert!(inherent_result.is_ok());
        let inherent = inherent_result.unwrap();
        assert!(inherent.get_data("timestamp").is_some());
        let data = inherent.get_data("timestamp").unwrap();
        let new_timestamp = Timestamp::from_bytes(data);
        assert!(new_timestamp.is_ok());
        assert_eq!(timestamp, new_timestamp.unwrap());
    }

    #[test]
    fn test_block_construction_success() {
        let provider = Box::new(TimestampProvider::new());
        let mut constructor = BlockConstructor::new(1);
        constructor.register_provider(provider);
        let block_resut = constructor.build_block();
        assert!(block_resut.is_ok());
        let block = block_resut.unwrap();
        assert!(block.get_inherent_timestamp().is_ok());
        assert!(block.get_inherent_timestamp().unwrap().is_some());
        
        let validate_result = block.validate();
        assert!(validate_result.is_ok());
    }

    #[test]
    #[test]
    fn test_missing_required_inherent() {
        let mut constructor = BlockConstructor::new(1);
        constructor.register_provider(Box::new(TimestampProvider::new()));
        
        let empty_data = InherentData::new();
        let validation_result = constructor.validate_inherents(&empty_data);

        assert!(validation_result.is_err());
        assert_eq!(validation_result.err(), Some("Required timestamp not found."));
    }

}








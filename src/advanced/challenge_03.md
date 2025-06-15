## Challenge 3: Simple Custom RPC for Counter Pallet

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a simple custom RPC (Remote Procedure Call) system for a counter pallet. This challenge focuses on understanding how external applications can query blockchain state and call custom functions that are not part of the standard runtime API.

**Main Concepts Covered:**
1. **RPC System:** External interface for blockchain interaction
2. **Runtime API:** Bridge between RPC and runtime logic
3. **State Queries:** Reading blockchain state without transactions
4. **Custom Endpoints:** Creating specialized query functions
5. **JSON-RPC Protocol:** Standard protocol for remote calls

### Detailed Structures to Implement:

#### **Counter Pallet (Simplified):**
```rust
use std::collections::HashMap;

pub struct CounterPallet {
    counters: HashMap<String, u32>,
}

impl CounterPallet {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }
    
    pub fn increment_counter(&mut self, name: String) -> u32 {
        let counter = self.counters.entry(name).or_insert(0);
        *counter = counter.saturating_add(1);
        *counter
    }
    
    pub fn get_counter(&self, name: &str) -> Option<u32> {
        self.counters.get(name).copied()
    }
    
    pub fn get_all_counters(&self) -> Vec<(String, u32)> {
        self.counters.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}
```

#### **RPC Trait Definition:**
```rust
/// RPC interface for counter operations
pub trait CounterRpc {
    /// Get the value of a specific counter
    fn get_counter(&self, name: String) -> Result<Option<u32>, String>;
    
    /// Get all counters and their values
    fn get_all_counters(&self) -> Result<Vec<(String, u32)>, String>;
    
    /// Get the total sum of all counters
    fn get_total_sum(&self) -> Result<u32, String>;
    
    /// Get counters above a certain threshold
    fn get_counters_above(&self, threshold: u32) -> Result<Vec<(String, u32)>, String>;
}
```

#### **Runtime API Bridge:**
```rust
/// Bridge between RPC and runtime
pub trait CounterRuntimeApi {
    fn get_counter_value(&self, name: String) -> Option<u32>;
    fn get_all_counter_values(&self) -> Vec<(String, u32)>;
}

/// Mock runtime API implementation
pub struct MockRuntimeApi {
    pallet: CounterPallet,
}

impl MockRuntimeApi {
    pub fn new(pallet: CounterPallet) -> Self {
        Self { pallet }
    }
}

impl CounterRuntimeApi for MockRuntimeApi {
    fn get_counter_value(&self, name: String) -> Option<u32> {
        self.pallet.get_counter(&name)
    }
    
    fn get_all_counter_values(&self) -> Vec<(String, u32)> {
        self.pallet.get_all_counters()
    }
}
```

#### **RPC Implementation:**
```rust
pub struct CounterRpcImpl<Api> {
    runtime_api: Api,
}

impl<Api> CounterRpcImpl<Api> {
    pub fn new(runtime_api: Api) -> Self {
        Self { runtime_api }
    }
}

impl<Api: CounterRuntimeApi> CounterRpc for CounterRpcImpl<Api> {
    fn get_counter(&self, name: String) -> Result<Option<u32>, String> {
        Ok(self.runtime_api.get_counter_value(name))
    }
    
    fn get_all_counters(&self) -> Result<Vec<(String, u32)>, String> {
        Ok(self.runtime_api.get_all_counter_values())
    }
    
    fn get_total_sum(&self) -> Result<u32, String> {
        let counters = self.runtime_api.get_all_counter_values();
        let sum = counters.iter().map(|(_, value)| value).sum();
        Ok(sum)
    }
    
    fn get_counters_above(&self, threshold: u32) -> Result<Vec<(String, u32)>, String> {
        let counters = self.runtime_api.get_all_counter_values();
        let filtered: Vec<(String, u32)> = counters
            .into_iter()
            .filter(|(_, value)| *value > threshold)
            .collect();
        Ok(filtered)
    }
}
```

### JSON-RPC Protocol Simulation:

### Project Setup

Before starting, you will need to configure the necessary dependencies:

#### **Cargo.toml:**
```toml
[package]
name = "counter-rpc-challenge"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### **How to configure (choose one option):**

**Option 1 - Using cargo add (recommended):**
```bash
cargo add serde --features derive
cargo add serde_json
```

**Option 2 - Editing Cargo.toml manually:**
```bash
# Edit the Cargo.toml file above and then run:
cargo build
```

#### **RPC Request/Response Types:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<RpcError>,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}
```

#### **RPC Handler:**
```rust
pub struct RpcHandler<Rpc> {
    counter_rpc: Rpc,
}

impl<Rpc: CounterRpc> RpcHandler<Rpc> {
    pub fn new(counter_rpc: Rpc) -> Self {
        Self { counter_rpc }
    }
    
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        let result = match request.method.as_str() {
            "counter_getCounter" => {
                let name = match request.params.get("name") {
                    Some(serde_json::Value::String(s)) => s.clone(),
                    _ => return self.error_response(request.id, -32602, "Invalid params"),
                };
                
                match self.counter_rpc.get_counter(name) {
                    Ok(value) => serde_json::to_value(value).unwrap(),
                    Err(e) => return self.error_response(request.id, -32603, &e),
                }
            },
            "counter_getAllCounters" => {
                match self.counter_rpc.get_all_counters() {
                    Ok(counters) => serde_json::to_value(counters).unwrap(),
                    Err(e) => return self.error_response(request.id, -32603, &e),
                }
            },
            "counter_getTotalSum" => {
                match self.counter_rpc.get_total_sum() {
                    Ok(sum) => serde_json::to_value(sum).unwrap(),
                    Err(e) => return self.error_response(request.id, -32603, &e),
                }
            },
            "counter_getCountersAbove" => {
                let threshold = match request.params.get("threshold") {
                    Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0) as u32,
                    _ => return self.error_response(request.id, -32602, "Invalid params"),
                };
                
                match self.counter_rpc.get_counters_above(threshold) {
                    Ok(counters) => serde_json::to_value(counters).unwrap(),
                    Err(e) => return self.error_response(request.id, -32603, &e),
                }
            },
            _ => return self.error_response(request.id, -32601, "Method not found"),
        };
        
        RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id: request.id,
        }
    }
    
    fn error_response(&self, id: u64, code: i32, message: &str) -> RpcResponse {
        RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError {
                code,
                message: message.to_string(),
            }),
            id,
        }
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Counter Pallet Operations:**
   - Test counter increment functionality
   - Test counter retrieval
   - Test getting all counters

2. **Runtime API:**
   - Test runtime API bridge functionality
   - Verify correct data flow from pallet to API

3. **RPC Implementation:**
   - Test all RPC methods
   - Verify correct return values
   - Test error handling

4. **JSON-RPC Protocol:**
   - Test request/response serialization
   - Test method routing
   - Test error responses
   - Test parameter validation

### Expected Output

A complete RPC system that:
- Provides external access to counter pallet state
- Implements proper JSON-RPC protocol
- Handles errors gracefully
- Demonstrates separation between runtime and RPC layers
- Shows understanding of blockchain external interfaces

### Theoretical Context

**RPC in Substrate:**
- **Purpose:** Allows external applications to interact with the blockchain
- **Architecture:** RPC layer → Runtime API → Pallet logic
- **Protocol:** Uses JSON-RPC 2.0 standard
- **State Queries:** Read blockchain state without submitting transactions
- **Custom Methods:** Pallets can expose specialized query functions

**Runtime API:**
- Bridge between external RPC calls and runtime logic
- Defined using `sp_api::decl_runtime_apis!` macro
- Implemented in runtime using `impl_runtime_apis!` macro
- Provides versioned interface for external queries

This system enables rich client applications and external integrations with the blockchain.
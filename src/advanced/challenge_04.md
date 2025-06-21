## Challenge 4: Custom RPC Methods

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement a custom RPC (Remote Procedure Call) system that allows external applications to query blockchain state through custom endpoints. This challenge focuses on understanding how Substrate exposes blockchain data to external clients and how to create efficient query interfaces.

**Main Concepts Covered:**
1. **RPC Interface Design:** Creating custom query endpoints
2. **Runtime API Bridge:** Connecting RPC to runtime logic
3. **State Queries:** Efficient blockchain state access
4. **Error Handling:** Proper RPC error management
5. **Separation of Concerns:** RPC layer vs Runtime layer

### Detailed Structures to Implement:

#### **RPC Trait Definition:**
```rust
/// Custom RPC interface for blockchain queries
pub trait CustomRpc {
    /// Get item by ID
    fn get_item(&self, id: u32) -> Result<Option<String>, RpcError>;
    
    /// Get all items
    fn get_all_items(&self) -> Result<Vec<(u32, String)>, RpcError>;
    
    /// Get items count
    fn get_items_count(&self) -> Result<u32, RpcError>;
    
    /// Check if item exists
    fn item_exists(&self, id: u32) -> Result<bool, RpcError>;
}
```

#### **RPC Error Types:**
```rust
#[derive(Debug, Clone)]
pub enum RpcError {
    /// Item not found
    ItemNotFound,
    /// Internal server error
    InternalError(String),
    /// Invalid parameters
    InvalidParams(String),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::ItemNotFound => write!(f, "Item not found"),
            RpcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            RpcError::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
        }
    }
}
```

#### **Runtime API Interface:**
```rust
/// Runtime API for RPC queries
pub trait RuntimeApi {
    /// Get item from runtime storage
    fn runtime_get_item(&self, id: u32) -> Option<String>;
    
    /// Get all items from runtime storage
    fn runtime_get_all_items(&self) -> Vec<(u32, String)>;
    
    /// Get total items count
    fn runtime_get_count(&self) -> u32;
}
```

#### **Mock Runtime Implementation:**
```rust
/// Mock runtime that simulates blockchain state
pub struct MockRuntime {
    items: std::collections::HashMap<u32, String>,
}

impl MockRuntime {
    pub fn new() -> Self {
        let mut items = std::collections::HashMap::new();
        items.insert(1, "First item".to_string());
        items.insert(2, "Second item".to_string());
        items.insert(3, "Third item".to_string());
        
        Self { items }
    }
}

impl RuntimeApi for MockRuntime {
    fn runtime_get_item(&self, id: u32) -> Option<String> {
        self.items.get(&id).cloned()
    }
    
    fn runtime_get_all_items(&self) -> Vec<(u32, String)> {
        self.items.iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
    
    fn runtime_get_count(&self) -> u32 {
        self.items.len() as u32
    }
}
```

#### **RPC Implementation:**
```rust
/// RPC implementation that bridges to runtime
pub struct CustomRpcImpl<R: RuntimeApi> {
    runtime: R,
}

impl<R: RuntimeApi> CustomRpcImpl<R> {
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }
}

impl<R: RuntimeApi> CustomRpc for CustomRpcImpl<R> {
    fn get_item(&self, id: u32) -> Result<Option<String>, RpcError> {
        if id == 0 {
            return Err(RpcError::InvalidParams("ID cannot be zero".to_string()));
        }
        
        Ok(self.runtime.runtime_get_item(id))
    }
    
    fn get_all_items(&self) -> Result<Vec<(u32, String)>, RpcError> {
        let items = self.runtime.runtime_get_all_items();
        if items.len() > 1000 {
            return Err(RpcError::InternalError("Too many items".to_string()));
        }
        Ok(items)
    }
    
    fn get_items_count(&self) -> Result<u32, RpcError> {
        Ok(self.runtime.runtime_get_count())
    }
    
    fn item_exists(&self, id: u32) -> Result<bool, RpcError> {
        if id == 0 {
            return Err(RpcError::InvalidParams("ID cannot be zero".to_string()));
        }
        
        Ok(self.runtime.runtime_get_item(id).is_some())
    }
}
```

#### **RPC Server Simulation:**
```rust
/// Simulates an RPC server handling requests
pub struct RpcServer<T: CustomRpc> {
    rpc_impl: T,
}

impl<T: CustomRpc> RpcServer<T> {
    pub fn new(rpc_impl: T) -> Self {
        Self { rpc_impl }
    }
    
    /// Handle RPC request
    pub fn handle_request(&self, method: &str, params: RpcParams) -> RpcResponse {
        match method {
            "get_item" => {
                match params.id {
                    Some(id) => match self.rpc_impl.get_item(id) {
                        Ok(Some(item)) => RpcResponse::success(ResponseData::Item(item)),
                        Ok(None) => RpcResponse::error(RpcError::ItemNotFound),
                        Err(e) => RpcResponse::error(e),
                    },
                    None => RpcResponse::error(RpcError::InvalidParams("Missing ID".to_string())),
                }
            },
            "get_all_items" => {
                match self.rpc_impl.get_all_items() {
                    Ok(items) => RpcResponse::success(ResponseData::Items(items)),
                    Err(e) => RpcResponse::error(e),
                }
            },
            "get_items_count" => {
                match self.rpc_impl.get_items_count() {
                    Ok(count) => RpcResponse::success(ResponseData::Count(count)),
                    Err(e) => RpcResponse::error(e),
                }
            },
            "item_exists" => {
                match params.id {
                    Some(id) => match self.rpc_impl.item_exists(id) {
                        Ok(exists) => RpcResponse::success(ResponseData::Exists(exists)),
                        Err(e) => RpcResponse::error(e),
                    },
                    None => RpcResponse::error(RpcError::InvalidParams("Missing ID".to_string())),
                }
            },
            _ => RpcResponse::error(RpcError::InvalidParams("Unknown method".to_string())),
        }
    }
}
```

#### **RPC Protocol Types:**
```rust
/// RPC request parameters
#[derive(Debug)]
pub struct RpcParams {
    pub id: Option<u32>,
}

/// RPC response data
#[derive(Debug)]
pub enum ResponseData {
    Item(String),
    Items(Vec<(u32, String)>),
    Count(u32),
    Exists(bool),
}

/// RPC response
#[derive(Debug)]
pub struct RpcResponse {
    pub success: bool,
    pub data: Option<ResponseData>,
    pub error: Option<RpcError>,
}

impl RpcResponse {
    pub fn success(data: ResponseData) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(error: RpcError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
```

### Tests

Create comprehensive tests covering:

1. **RPC Interface:**
   - Test all RPC methods with valid parameters
   - Test error handling for invalid parameters
   - Verify proper error propagation

2. **Runtime Bridge:**
   - Test runtime API integration
   - Verify data consistency between RPC and runtime
   - Test edge cases and error conditions

3. **Server Simulation:**
   - Test request handling for all methods
   - Verify response formatting
   - Test unknown method handling

4. **Error Handling:**
   - Test all error types
   - Verify error message formatting
   - Test error propagation through layers

### Expected Output

A complete custom RPC system that:
- Defines clear RPC interfaces
- Bridges RPC calls to runtime logic
- Handles errors appropriately
- Demonstrates separation of concerns
- Shows understanding of Substrate's RPC architecture

### Theoretical Context

**RPC in Substrate:**
- **Purpose:** Allows external applications to query blockchain state
- **Architecture:** RPC layer → Runtime API → Runtime logic
- **Separation:** RPC handles protocol, Runtime handles business logic
- **Efficiency:** Direct state access without transaction overhead
- **Flexibility:** Custom endpoints for specific application needs

**Best Practices:**
- Keep RPC methods simple and focused
- Validate parameters at RPC layer
- Handle errors gracefully
- Use appropriate data types for responses
- Document RPC interfaces clearly

This system demonstrates how Substrate exposes blockchain functionality to external clients through well-designed RPC interfaces.
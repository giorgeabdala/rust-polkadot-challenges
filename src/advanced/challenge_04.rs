#[derive(Debug, Clone,PartialEq)]
pub enum RpcError {
    ItemNotFound,
    InternalError(String),
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
pub trait CustomRpc {
    fn get_item(&self, id: u32) -> Result<Option<String>, RpcError>;
    fn get_all_items(&self) -> Result<Vec<(u32, String)>, RpcError>;
    fn get_items_count(&self) -> Result<u32, RpcError>;
    fn item_exists(&self, id: u32) -> Result<bool, RpcError>;
}

pub trait RuntimeApi {
    fn runtime_get_item(&self, id: u32) -> Option<String>;
    fn runtime_get_all_items(&self) -> Vec<(u32, String)>;
    fn runtime_get_count(&self) -> u32;
}


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

    pub fn new_with_many_items(count: u32) -> Self {
        let mut items = std::collections::HashMap::new();
        for i in 1..=count {
            items.insert(i, format!("Item {}", i));
        }
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


pub struct CustomRpcImpl<R: RuntimeApi> {
    runtime: R
}

impl <R: RuntimeApi> CustomRpcImpl<R>{
    pub fn new(runtime: R) -> Self {
        Self{runtime}
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

/// RPC request parameters
#[derive(Debug, Clone)]
pub struct RpcParams {
    pub id: Option<u32>,
}

/// RPC response data
#[derive(Debug, PartialEq)]
pub enum ResponseData {
    Item(String),
    Items(Vec<(u32, String)>),
    Count(u32),
    Exists(bool),
}

/// RPC response
#[derive(Debug, PartialEq)]
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


mod tests {
    use crate::advanced::challenge_04::{CustomRpc, CustomRpcImpl, MockRuntime, ResponseData, RpcError, RpcParams, RpcResponse, RpcServer};
    use crate::advanced::challenge_04::RpcError::InvalidParams;

    fn create_rpc() -> CustomRpcImpl<MockRuntime> {
        let runtime = MockRuntime::new();
        CustomRpcImpl::new(runtime)
    }

    fn create_server() -> RpcServer<CustomRpcImpl<MockRuntime>> {
        RpcServer::new(create_rpc())
    }


    #[test]
    fn server_handle_request_test() {
        let server = create_server();
        let first_id = 1;
        let first_item = "First item".to_string();
        let second_item = "Second item".to_string();
        let third_item = "Third item".to_string();

        let rpc_params = RpcParams { id: Some(first_id) };
        let rpc_response = server.handle_request("get_item", rpc_params.clone());
        assert_eq!(rpc_response.success, true);
        assert!(rpc_response.data.is_some());
        assert_eq!(rpc_response.data.unwrap(), ResponseData::Item(first_item.clone()));

        let rpc_response = server.handle_request("get_all_items", rpc_params.clone());
        assert_eq!(rpc_response.success, true);
        assert!(rpc_response.data.is_some());

        if let ResponseData::Items(mut returned_items) = rpc_response.data.unwrap() {
            returned_items.sort_by_key(|item| item.0);

            let mut expected_items = vec![
                (1, first_item.clone()),
                (2, second_item.clone()),
                (3, third_item.clone())
            ];
            expected_items.sort_by_key(|item| item.0);

            assert_eq!(returned_items, expected_items);
        } else {
            panic!("Expected ResponseData::Items");
        }

        let rpc_response = server.handle_request("get_items_count", rpc_params.clone());
        assert_eq!(rpc_response.success, true);
        assert!(rpc_response.data.is_some());
        assert_eq!(rpc_response.data.unwrap(), ResponseData::Count(3));

        let rpc_response = server.handle_request("item_exists", rpc_params);
        assert_eq!(rpc_response.success, true);
        assert!(rpc_response.data.is_some());
        assert_eq!(rpc_response.data.unwrap(), ResponseData::Exists(true));

        let rpc_params_not_found = RpcParams { id: Some(999) };
        let rpc_response = server.handle_request("item_exists", rpc_params_not_found);
        assert_eq!(rpc_response.success, true);
        assert!(rpc_response.data.is_some());
        assert_eq!(rpc_response.data.unwrap(), ResponseData::Exists(false));
    }

    #[test]
    fn server_handle_request_method_nonexistent() {
        let server = create_server();
        let first_id = 1;

        let rpc_params = RpcParams { id: Some(first_id) };
        let rpc_response = server.handle_request("non_existent", rpc_params.clone());
        assert_eq!(rpc_response.success, false);
        assert!(rpc_response.data.is_none());
        assert_eq!(rpc_response.error, Some(InvalidParams("Unknown method".to_string())));
    }

    #[test]
    fn rpc_get_item_test() {
        let rpc = create_rpc();
        let result = rpc.get_item(1);
        assert!(result.is_ok());
        let opt = result.unwrap();
        assert!(opt.is_some());
        assert_eq!(opt, Some("First item".to_string()));
    }

    #[test]
    fn rpc_get_all_items_test() {
        let rpc = create_rpc();
        let result = rpc.get_all_items();

        assert!(result.is_ok());
        let mut items = result.unwrap();
        assert_eq!(items.len(), 3);

        items.sort_by_key(|item| item.0);
        assert_eq!(items[0], (1, "First item".to_string()));
        assert_eq!(items[1], (2, "Second item".to_string()));
        assert_eq!(items[2], (3, "Third item".to_string()));
    }

    #[test]
    fn get_items_count_test() {
        let rpc = create_rpc();
        let result = rpc.get_items_count();
        assert!(result.is_ok());
        let count = result.unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn rpc_item_exists_true() {
        let rpc = create_rpc();
        let result = rpc.item_exists(1);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn rpc_item_exists_false() {
        let rpc = create_rpc();
        let result = rpc.item_exists(5);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn custom_rpc_impl_get_item_id_zero() {
        let rpc = create_rpc(); // Sua função auxiliar
        let result = rpc.get_item(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RpcError::InvalidParams("ID cannot be zero".to_string())
        );
    }

    #[test]
    fn custom_rpc_impl_get_all_items_exceeds_limit() {
        let runtime_with_many_items = MockRuntime::new_with_many_items(1001);
        let rpc = CustomRpcImpl::new(runtime_with_many_items);
        let result = rpc.get_all_items();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RpcError::InternalError("Too many items".to_string())
        );
    }
}


## Challenge 12: Simple Runtime Integration

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement a simplified runtime that integrates two basic pallets (System and Balances) to demonstrate how pallets work together in a blockchain runtime. This challenge focuses on understanding runtime construction and pallet integration without complex external dependencies.

### Main Concepts Covered

1. **Runtime Construction**: How to build a basic runtime
2. **Pallet Integration**: Integration of multiple pallets
3. **Cross-Pallet Communication**: How pallets interact with each other
4. **Event System**: Unified event handling across pallets
5. **Error Handling**: Runtime-level error management

### Structures to Implement

#### **Basic Runtime Types:**
```rust
// Fundamental runtime types
pub type AccountId = String; // Simplified
pub type BlockNumber = u64;
pub type Hash = [u8; 32];
pub type Balance = u128;
```

#### **System Pallet:**
```rust
pub mod system {
    use super::*;
    use std::collections::HashMap;

    pub trait Config {
        type AccountId: Clone + PartialEq + core::fmt::Debug;
        type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug;
        type Hash: Clone + PartialEq + core::fmt::Debug;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        NewAccount { account: T::AccountId },
        BlockFinalized { number: T::BlockNumber },
        ExtrinsicSuccess { account: T::AccountId },
    }

    pub struct Pallet<T: Config> {
        account_nonces: HashMap<T::AccountId, u32>,
        current_block_number: T::BlockNumber,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                account_nonces: HashMap::new(),
                current_block_number: T::BlockNumber::default(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn inc_account_nonce(&mut self, account: &T::AccountId) {
            let nonce = self.account_nonces.entry(account.clone()).or_insert(0);
            *nonce += 1;
            
            if *nonce == 1 {
                self.events.push(Event::NewAccount { account: account.clone() });
            }
        }

        pub fn account_nonce(&self, account: &T::AccountId) -> u32 {
            self.account_nonces.get(account).copied().unwrap_or_default()
        }

        pub fn finalize_block(&mut self, number: T::BlockNumber) {
            self.current_block_number = number;
            self.events.push(Event::BlockFinalized { number });
        }

        pub fn block_number(&self) -> T::BlockNumber {
            self.current_block_number
        }

        pub fn record_extrinsic_success(&mut self, account: T::AccountId) {
            self.events.push(Event::ExtrinsicSuccess { account });
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Balances Pallet:**
```rust
pub mod balances {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config {
        type Balance: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug +
                     core::ops::Add<Output = Self::Balance> + 
                     core::ops::Sub<Output = Self::Balance>;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        Transfer { 
            from: T::AccountId, 
            to: T::AccountId, 
            amount: T::Balance 
        },
        BalanceSet { 
            account: T::AccountId, 
            balance: T::Balance 
        },
        Endowed {
            account: T::AccountId,
            balance: T::Balance,
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        InsufficientBalance,
        AccountNotFound,
        ZeroAmount,
    }

    pub struct Pallet<T: Config> {
        balances: HashMap<T::AccountId, T::Balance>,
        total_issuance: T::Balance,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                balances: HashMap::new(),
                total_issuance: T::Balance::default(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn set_balance(&mut self, account: T::AccountId, balance: T::Balance) {
            let old_balance = self.balances.get(&account).copied().unwrap_or_default();
            
            if old_balance == T::Balance::default() && balance > T::Balance::default() {
                self.events.push(Event::Endowed { 
                    account: account.clone(), 
                    balance 
                });
            }
            
            self.balances.insert(account.clone(), balance);
            
            // Adjust total issuance
            if balance > old_balance {
                self.total_issuance = self.total_issuance + (balance - old_balance);
            } else if old_balance > balance {
                self.total_issuance = self.total_issuance - (old_balance - balance);
            }

            self.events.push(Event::BalanceSet { account, balance });
        }

        pub fn transfer(
            &mut self,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> Result<(), Error> {
            if amount == T::Balance::default() {
                return Err(Error::ZeroAmount);
            }

            let from_balance = self.balances.get(&from).copied().unwrap_or_default();
            
            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let to_balance = self.balances.get(&to).copied().unwrap_or_default();

            // Update balances
            self.balances.insert(from.clone(), from_balance - amount);
            self.balances.insert(to.clone(), to_balance + amount);

            self.events.push(Event::Transfer { from, to, amount });
            Ok(())
        }

        pub fn balance(&self, account: &T::AccountId) -> T::Balance {
            self.balances.get(account).copied().unwrap_or_default()
        }

        pub fn total_issuance(&self) -> T::Balance {
            self.total_issuance
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Runtime Events (Unified):**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeEvent<T: RuntimeConfig> {
    System(system::Event<T>),
    Balances(balances::Event<T>),
}

impl<T: RuntimeConfig> From<system::Event<T>> for RuntimeEvent<T> {
    fn from(event: system::Event<T>) -> Self {
        RuntimeEvent::System(event)
    }
}

impl<T: RuntimeConfig> From<balances::Event<T>> for RuntimeEvent<T> {
    fn from(event: balances::Event<T>) -> Self {
        RuntimeEvent::Balances(event)
    }
}
```

#### **Runtime Configuration:**
```rust
pub trait RuntimeConfig: system::Config + balances::Config {
    // Runtime-specific configuration can go here
}
```

#### **Main Runtime:**
```rust
pub struct Runtime<T: RuntimeConfig> {
    pub system: system::Pallet<T>,
    pub balances: balances::Pallet<T>,
    events: Vec<RuntimeEvent<T>>,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: RuntimeConfig> Runtime<T> {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            events: Vec::new(),
            _phantom: core::marker::PhantomData,
        }
    }

    // Genesis configuration - set initial state
    pub fn genesis_config(
        &mut self,
        initial_balances: Vec<(T::AccountId, T::Balance)>,
    ) {
        for (account, balance) in initial_balances {
            self.balances.set_balance(account, balance);
        }
        
        // Collect genesis events
        self.collect_events();
    }

    // Execute a transfer extrinsic
    pub fn execute_transfer(
        &mut self,
        origin: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), balances::Error> {
        // Increment nonce (simulating extrinsic execution)
        self.system.inc_account_nonce(&origin);
        
        // Execute transfer
        let result = self.balances.transfer(origin.clone(), to, amount);
        
        // Record success if transfer succeeded
        if result.is_ok() {
            self.system.record_extrinsic_success(origin);
        }
        
        // Collect events from all pallets
        self.collect_events();
        
        result
    }

    // Finalize a block
    pub fn finalize_block(&mut self, block_number: T::BlockNumber) {
        self.system.finalize_block(block_number);
        self.collect_events();
    }

    // Collect events from all pallets
    fn collect_events(&mut self) {
        // Collect system events
        for event in self.system.take_events() {
            self.events.push(RuntimeEvent::System(event));
        }
        
        // Collect balances events
        for event in self.balances.take_events() {
            self.events.push(RuntimeEvent::Balances(event));
        }
    }

    // Get all runtime events
    pub fn take_events(&mut self) -> Vec<RuntimeEvent<T>> {
        std::mem::take(&mut self.events)
    }

    // Query methods
    pub fn account_balance(&self, account: &T::AccountId) -> T::Balance {
        self.balances.balance(account)
    }

    pub fn account_nonce(&self, account: &T::AccountId) -> u32 {
        self.system.account_nonce(account)
    }

    pub fn current_block(&self) -> T::BlockNumber {
        self.system.block_number()
    }

    pub fn total_issuance(&self) -> T::Balance {
        self.balances.total_issuance()
    }
}
```

### Test Configuration

#### **Test Runtime Config:**
```rust
struct TestRuntimeConfig;

impl system::Config for TestRuntimeConfig {
    type AccountId = String;
    type BlockNumber = u64;
    type Hash = [u8; 32];
}

impl balances::Config for TestRuntimeConfig {
    type Balance = u128;
}

impl RuntimeConfig for TestRuntimeConfig {}

type TestRuntime = Runtime<TestRuntimeConfig>;
```

### Tests

Create comprehensive tests covering:

#### **Test Scenarios:**

1. **Runtime Construction:**
   - Test runtime initialization
   - Test pallet integration
   - Test genesis configuration

2. **Cross-Pallet Communication:**
   - Test system pallet tracking account nonces
   - Test balances pallet operations
   - Test event propagation between pallets

3. **Extrinsic Execution:**
   - Test successful transfer execution
   - Test failed transfer handling
   - Test nonce incrementation

4. **Block Lifecycle:**
   - Test block finalization
   - Test event collection across blocks
   - Test state persistence

5. **Event System:**
   - Test unified event handling
   - Test event ordering
   - Test event data integrity

6. **Integration Tests:**
   - Test complete runtime workflow
   - Test multiple accounts and transfers
   - Test runtime state consistency

### Example Usage

```rust
fn main() {
    let mut runtime = TestRuntime::new();
    
    // Set up genesis state
    runtime.genesis_config(vec![
        ("alice".to_string(), 1000),
        ("bob".to_string(), 500),
    ]);
    
    println!("Genesis events: {:?}", runtime.take_events());
    
    // Execute a transfer
    let result = runtime.execute_transfer(
        "alice".to_string(),
        "bob".to_string(),
        100,
    );
    
    println!("Transfer result: {:?}", result);
    println!("Transfer events: {:?}", runtime.take_events());
    
    // Check balances
    println!("Alice balance: {}", runtime.account_balance(&"alice".to_string()));
    println!("Bob balance: {}", runtime.account_balance(&"bob".to_string()));
    println!("Alice nonce: {}", runtime.account_nonce(&"alice".to_string()));
    
    // Finalize block
    runtime.finalize_block(1);
    println!("Block events: {:?}", runtime.take_events());
}
```

### Expected Output

A complete runtime integration system that:
- Demonstrates pallet integration within a runtime
- Shows cross-pallet communication patterns
- Implements unified event handling
- Provides runtime-level state management
- Shows understanding of blockchain runtime architecture

### Theoretical Context

**Runtime Architecture:**
- **Pallets**: Modular components providing specific functionality
- **Integration**: How pallets work together to form a complete runtime
- **Events**: Unified notification system across all pallets
- **State**: Coordinated state management across multiple pallets

**Key Concepts:**
- **Modularity**: Each pallet handles specific concerns
- **Composition**: Runtime is composed of multiple pallets
- **Communication**: Pallets can interact through the runtime
- **Consistency**: Runtime ensures consistent state across pallets

**Simplifications:**
- **No External Dependencies**: Pure Rust implementation without complex codecs
- **Simple Types**: Basic types instead of complex Substrate types
- **Direct Integration**: Straightforward pallet communication patterns
- **Minimal Overhead**: Focus on concepts without implementation complexity

This challenge teaches essential runtime integration concepts while maintaining focus on the core architectural patterns that make modular blockchain runtimes possible. 
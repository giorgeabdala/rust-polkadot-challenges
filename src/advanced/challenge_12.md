## Challenge 12: Simple Runtime Integration (Simplified)

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a simplified runtime that integrates two basic pallets (System and Balances) to demonstrate how pallets work together in a Substrate runtime. This challenge focuses on understanding runtime construction and pallet integration with essential external dependencies.

**Main Concepts Covered:**
1. **Runtime Construction**: How to build a basic runtime
2. **Pallet Integration**: Integration of multiple pallets
3. **Runtime Configuration**: Runtime parameter configuration
4. **Genesis Configuration**: Initial blockchain state

### Project Setup

Before starting, you will need to configure the necessary dependencies:

#### **Cargo.toml:**
```toml
[package]
name = "runtime-integration-challenge"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
```

#### **How to configure (choose one option):**

**Option 1 - Using cargo add (recommended):**
```bash
cargo add codec --package parity-scale-codec --features derive
cargo add scale-info --features derive
```

**Option 2 - Editing Cargo.toml manually:**
```bash
# Edit the Cargo.toml file above and then run:
cargo build
```

### Structures to Implement

#### **Basic Runtime Types:**
```rust
use codec::{Encode, Decode};
use scale_info::TypeInfo;

// Fundamental runtime types
pub type AccountId = String; // Simplified
pub type BlockNumber = u64;
pub type Hash = [u8; 32];
pub type Balance = u128;
```

#### **System Pallet (Simplified):**
```rust
pub mod system {
    use super::*;
    use std::collections::HashMap;

    pub trait Config {
        type AccountId: Clone + PartialEq + core::fmt::Debug + Encode + Decode + TypeInfo;
        type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug + Encode + Decode + TypeInfo;
        type Hash: Clone + PartialEq + core::fmt::Debug + Encode + Decode + TypeInfo;
    }

    #[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
    pub enum Event<T: Config> {
        NewAccount { account: T::AccountId },
        BlockFinalized { number: T::BlockNumber },
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

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Balances Pallet (Simplified):**
```rust
pub mod balances {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config {
        type Balance: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug +
                     core::ops::Add<Output = Self::Balance> + 
                     core::ops::Sub<Output = Self::Balance> +
                     Encode + Decode + TypeInfo;
    }

    #[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
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
    }

    #[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
        AccountNotFound,
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
            let from_balance = self.balances.get(&from).copied().unwrap_or_default();
            
            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let to_balance = self.balances.get(&to).copied().unwrap_or_default();

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

### Runtime Configuration and Integration

#### **Runtime Configuration:**
```rust
// Runtime configuration that ties all pallets together
pub struct RuntimeConfig;

impl system::Config for RuntimeConfig {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
}

impl balances::Config for RuntimeConfig {
    type Balance = Balance;
}
```

#### **Complete Runtime:**
```rust
pub struct Runtime {
    pub system: system::Pallet<RuntimeConfig>,
    pub balances: balances::Pallet<RuntimeConfig>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    /// Initialize genesis state
    pub fn initialize_genesis(&mut self, accounts: Vec<(AccountId, Balance)>) {
        for (account, balance) in accounts {
            self.balances.set_balance(account, balance);
        }
    }

    /// Process a transfer transaction
    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), balances::Error> {
        // Increment nonce for the sender
        self.system.inc_account_nonce(&from);
        
        // Perform the transfer
        self.balances.transfer(from, to, amount)
    }

    /// Finalize a block
    pub fn finalize_block(&mut self, block_number: BlockNumber) {
        self.system.finalize_block(block_number);
    }

    /// Get all events from all pallets
    pub fn take_all_events(&mut self) -> Vec<String> {
        let mut all_events = Vec::new();
        
        // Collect system events
        for event in self.system.take_events() {
            match event {
                system::Event::NewAccount { account } => {
                    all_events.push(format!("System: New account created: {}", account));
                },
                system::Event::BlockFinalized { number } => {
                    all_events.push(format!("System: Block {} finalized", number));
                },
            }
        }
        
        // Collect balance events
        for event in self.balances.take_events() {
            match event {
                balances::Event::Transfer { from, to, amount } => {
                    all_events.push(format!("Balances: Transfer {} from {} to {}", amount, from, to));
                },
                balances::Event::BalanceSet { account, balance } => {
                    all_events.push(format!("Balances: Balance set for {}: {}", account, balance));
                },
            }
        }
        
        all_events
    }

    /// Get runtime statistics
    pub fn get_stats(&self) -> RuntimeStats {
        RuntimeStats {
            total_accounts: self.system.account_nonces.len(),
            total_balance: self.balances.total_issuance(),
            current_block: self.system.block_number(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo)]
pub struct RuntimeStats {
    pub total_accounts: usize,
    pub total_balance: Balance,
    pub current_block: BlockNumber,
}
```

### Implementation Requirements:

1. **System Pallet**: Implement account management and block finalization with SCALE codec
2. **Balances Pallet**: Implement balance tracking and transfers with SCALE codec
3. **Runtime Configuration**: Connect pallets with proper types
4. **Runtime Integration**: Coordinate pallet interactions

### Test Configuration:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_initialization_works() {
        let mut runtime = Runtime::new();
        
        // Initialize with some accounts
        let genesis_accounts = vec![
            ("alice".to_string(), 1000),
            ("bob".to_string(), 500),
        ];
        
        runtime.initialize_genesis(genesis_accounts);
        
        let stats = runtime.get_stats();
        assert_eq!(stats.total_balance, 1500);
    }

    #[test]
    fn transfer_works() {
        let mut runtime = Runtime::new();
        
        // Initialize accounts
        runtime.initialize_genesis(vec![
            ("alice".to_string(), 1000),
            ("bob".to_string(), 500),
        ]);
        
        // Perform transfer
        let result = runtime.transfer("alice".to_string(), "bob".to_string(), 200);
        assert!(result.is_ok());
        
        // Check balances
        assert_eq!(runtime.balances.balance(&"alice".to_string()), 800);
        assert_eq!(runtime.balances.balance(&"bob".to_string()), 700);
        
        // Check events
        let events = runtime.take_all_events();
        assert!(events.iter().any(|e| e.contains("Transfer 200 from alice to bob")));
    }

    #[test]
    fn insufficient_balance_fails() {
        let mut runtime = Runtime::new();
        
        runtime.initialize_genesis(vec![
            ("alice".to_string(), 100),
        ]);
        
        let result = runtime.transfer("alice".to_string(), "bob".to_string(), 200);
        assert_eq!(result, Err(balances::Error::InsufficientBalance));
    }

    #[test]
    fn block_finalization_works() {
        let mut runtime = Runtime::new();
        
        runtime.finalize_block(1);
        runtime.finalize_block(2);
        
        let stats = runtime.get_stats();
        assert_eq!(stats.current_block, 2);
        
        let events = runtime.take_all_events();
        assert!(events.iter().any(|e| e.contains("Block 1 finalized")));
        assert!(events.iter().any(|e| e.contains("Block 2 finalized")));
    }

    #[test]
    fn account_nonce_tracking_works() {
        let mut runtime = Runtime::new();
        
        runtime.initialize_genesis(vec![
            ("alice".to_string(), 1000),
        ]);
        
        // First transfer creates account event
        runtime.transfer("alice".to_string(), "bob".to_string(), 100).unwrap();
        let events = runtime.take_all_events();
        assert!(events.iter().any(|e| e.contains("New account created: alice")));
        
        // Second transfer doesn't create account event
        runtime.transfer("alice".to_string(), "bob".to_string(), 100).unwrap();
        let events = runtime.take_all_events();
        assert!(!events.iter().any(|e| e.contains("New account created: alice")));
    }

    #[test]
    fn scale_encoding_works() {
        let stats = RuntimeStats {
            total_accounts: 5,
            total_balance: 10000,
            current_block: 42,
        };
        
        let encoded = stats.encode();
        let decoded = RuntimeStats::decode(&mut &encoded[..]).unwrap();
        assert_eq!(stats, decoded);
    }
}
```

### Expected Output

A functional runtime that:
- Compiles without errors
- Passes all unit tests
- Demonstrates pallet integration with SCALE codec
- Shows runtime configuration
- Implements basic blockchain functionality

### Theoretical Context

**Runtime in Substrate:** The runtime is the core logic of a blockchain, containing all the pallets and their interactions. It's compiled to WebAssembly and executed by the blockchain nodes.

**SCALE Codec in Substrate:** Substrate uses the SCALE (Simple Concatenated Aggregate Little-Endian) codec for efficient binary serialization. The `codec` and `scale-info` dependencies are essential for understanding how data is encoded and decoded in the blockchain.

**Pallet Integration:** Pallets work together through shared types and configurations. The runtime acts as the coordinator, ensuring proper interaction between different modules.

**Genesis Configuration:** The initial state of the blockchain is set during genesis, establishing the starting balances and accounts.

This simplified version focuses on the essential concepts of runtime construction and pallet integration, while maintaining the essential external dependencies for proper SCALE serialization. 
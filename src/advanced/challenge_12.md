## Challenge 12: Substrate Node Template and Runtime Configuration

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

In this final challenge, you will implement a complete simulation of a Substrate Runtime, integrating multiple pallets into a cohesive system. The focus is on understanding how pallets are configured, connected, and how the runtime is built to form a functional blockchain.

### Main Concepts Covered

1. **Runtime Construction**: How to build a complete runtime
2. **Pallet Integration**: Integration of multiple pallets
3. **Runtime Configuration**: Runtime parameter configuration
4. **Genesis Configuration**: Initial blockchain state
5. **Runtime APIs**: Interfaces for external queries

### Structures to Implement

#### **Basic Runtime Types:**
```rust
// Fundamental runtime types
pub type AccountId = String; // Simplified
pub type BlockNumber = u64;
pub type Hash = [u8; 32];
pub type Balance = u128;
pub type Nonce = u32;

// Block header
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub number: BlockNumber,
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub extrinsics_root: Hash,
}

// Complete block
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub header: Header,
    pub extrinsics: Vec<Vec<u8>>, // Serialized extrinsics
}
```

#### **System Pallet (Simplified):**
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
    }

    pub struct Pallet<T: Config> {
        account_nonces: HashMap<T::AccountId, u32>,
        current_block_number: T::BlockNumber,
        block_hash: HashMap<T::BlockNumber, T::Hash>,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                account_nonces: HashMap::new(),
                current_block_number: T::BlockNumber::default(),
                block_hash: HashMap::new(),
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
            self.account_nonces.get(account).copied().unwrap_or(0)
        }

        pub fn finalize_block(&mut self, number: T::BlockNumber, hash: T::Hash) {
            self.current_block_number = number;
            self.block_hash.insert(number, hash);
            self.events.push(Event::BlockFinalized { number });
        }

        pub fn block_number(&self) -> T::BlockNumber {
            self.current_block_number
        }

        pub fn block_hash(&self, number: T::BlockNumber) -> Option<T::Hash> {
            self.block_hash.get(&number).copied()
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
    }

    #[derive(Clone, Debug, PartialEq)]
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

#### **Simple Governance Pallet:**
```rust
pub mod governance {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config + balances::Config {
        type ProposalId: Clone + Copy + Default + PartialEq + core::fmt::Debug;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Proposal<T: Config> {
        pub id: T::ProposalId,
        pub proposer: T::AccountId,
        pub description: String,
        pub votes_for: T::Balance,
        pub votes_against: T::Balance,
        pub status: ProposalStatus,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        ProposalCreated { 
            id: T::ProposalId, 
            proposer: T::AccountId 
        },
        Voted { 
            proposal_id: T::ProposalId, 
            voter: T::AccountId, 
            vote: bool, 
            weight: T::Balance 
        },
        ProposalExecuted { 
            id: T::ProposalId, 
            result: ProposalStatus 
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        ProposalNotFound,
        ProposalNotActive,
        InsufficientBalance,
    }

    pub struct Pallet<T: Config> {
        proposals: HashMap<T::ProposalId, Proposal<T>>,
        next_proposal_id: u32,
        votes: HashMap<(T::ProposalId, T::AccountId), (bool, T::Balance)>,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                proposals: HashMap::new(),
                next_proposal_id: 1,
                votes: HashMap::new(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn create_proposal(
            &mut self,
            proposer: T::AccountId,
            description: String,
        ) -> T::ProposalId {
            let id = T::ProposalId::default(); // Simplified ID generation
            let proposal = Proposal {
                id,
                proposer: proposer.clone(),
                description,
                votes_for: T::Balance::default(),
                votes_against: T::Balance::default(),
                status: ProposalStatus::Active,
            };

            self.proposals.insert(id, proposal);
            self.events.push(Event::ProposalCreated { id, proposer });
            self.next_proposal_id += 1;
            id
        }

        pub fn vote(
            &mut self,
            proposal_id: T::ProposalId,
            voter: T::AccountId,
            vote: bool,
            weight: T::Balance,
        ) -> Result<(), Error> {
            let proposal = self.proposals.get_mut(&proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            if !matches!(proposal.status, ProposalStatus::Active) {
                return Err(Error::ProposalNotActive);
            }

            // Remove previous vote if exists
            if let Some((old_vote, old_weight)) = self.votes.get(&(proposal_id, voter.clone())) {
                if *old_vote {
                    proposal.votes_for = proposal.votes_for - *old_weight;
                } else {
                    proposal.votes_against = proposal.votes_against - *old_weight;
                }
            }

            // Add new vote
            if vote {
                proposal.votes_for = proposal.votes_for + weight;
            } else {
                proposal.votes_against = proposal.votes_against + weight;
            }

            self.votes.insert((proposal_id, voter.clone()), (vote, weight));
            self.events.push(Event::Voted { proposal_id, voter, vote, weight });

            Ok(())
        }

        pub fn execute_proposal(&mut self, proposal_id: T::ProposalId) -> Result<(), Error> {
            let proposal = self.proposals.get_mut(&proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            if !matches!(proposal.status, ProposalStatus::Active) {
                return Err(Error::ProposalNotActive);
            }

            // Simple majority rule
            proposal.status = if proposal.votes_for > proposal.votes_against {
                ProposalStatus::Passed
            } else {
                ProposalStatus::Rejected
            };

            self.events.push(Event::ProposalExecuted { 
                id: proposal_id, 
                result: proposal.status.clone() 
            });

            Ok(())
        }

        pub fn get_proposal(&self, id: &T::ProposalId) -> Option<&Proposal<T>> {
            self.proposals.get(id)
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

impl governance::Config for RuntimeConfig {
    type ProposalId = u32;
}
```

#### **Complete Runtime:**
```rust
pub struct Runtime {
    pub system: system::Pallet<RuntimeConfig>,
    pub balances: balances::Pallet<RuntimeConfig>,
    pub governance: governance::Pallet<RuntimeConfig>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            governance: governance::Pallet::new(),
        }
    }

    // Genesis configuration
    pub fn initialize_genesis(&mut self, genesis_config: GenesisConfig) {
        // Set initial balances
        for (account, balance) in genesis_config.balances {
            self.balances.set_balance(account, balance);
        }

        // Initialize system
        self.system.finalize_block(0, [0u8; 32]);
    }

    // Execute a block
    pub fn execute_block(&mut self, block: Block) -> Result<(), RuntimeError> {
        // Validate block header
        let expected_number = self.system.block_number() + 1;
        if block.header.number != expected_number {
            return Err(RuntimeError::InvalidBlockNumber);
        }

        // Process extrinsics (simplified)
        for extrinsic in block.extrinsics {
            self.execute_extrinsic(extrinsic)?;
        }

        // Finalize block
        self.system.finalize_block(block.header.number, block.header.state_root);

        Ok(())
    }

    fn execute_extrinsic(&mut self, extrinsic: Vec<u8>) -> Result<(), RuntimeError> {
        // Simplified extrinsic execution
        // In real Substrate, this would involve decoding and dispatching calls
        Ok(())
    }

    // Runtime APIs for external queries
    pub fn account_balance(&self, account: &AccountId) -> Balance {
        self.balances.balance(account)
    }

    pub fn account_nonce(&self, account: &AccountId) -> Nonce {
        self.system.account_nonce(account)
    }

    pub fn current_block_number(&self) -> BlockNumber {
        self.system.block_number()
    }

    pub fn total_issuance(&self) -> Balance {
        self.balances.total_issuance()
    }

    pub fn get_proposal(&self, id: &u32) -> Option<&governance::Proposal<RuntimeConfig>> {
        self.governance.get_proposal(id)
    }

    // Collect all events from all pallets
    pub fn take_all_events(&mut self) -> RuntimeEvents {
        RuntimeEvents {
            system: self.system.take_events(),
            balances: self.balances.take_events(),
            governance: self.governance.take_events(),
        }
    }
}
```

#### **Supporting Types:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct GenesisConfig {
    pub balances: Vec<(AccountId, Balance)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    InvalidBlockNumber,
    ExtrinsicExecutionFailed,
    SystemError,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeEvents {
    pub system: Vec<system::Event<RuntimeConfig>>,
    pub balances: Vec<balances::Event<RuntimeConfig>>,
    pub governance: Vec<governance::Event<RuntimeConfig>>,
}

// Simplified extrinsic types
#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    Balances(BalancesCall),
    Governance(GovernanceCall),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BalancesCall {
    Transfer { to: AccountId, amount: Balance },
}

#[derive(Clone, Debug, PartialEq)]
pub enum GovernanceCall {
    CreateProposal { description: String },
    Vote { proposal_id: u32, vote: bool },
    ExecuteProposal { proposal_id: u32 },
}
```

### Tests

Create comprehensive tests covering:

#### **Test Scenarios:**

1. **Runtime Initialization:**
   - Test genesis configuration
   - Test initial balances setup
   - Test system initialization

2. **Pallet Integration:**
   - Test cross-pallet functionality
   - Test event collection from all pallets
   - Test runtime APIs

3. **Block Execution:**
   - Test block validation
   - Test extrinsic processing
   - Test block finalization

4. **Complete Workflows:**
   - Test balance transfers
   - Test governance proposals and voting
   - Test multi-step operations

5. **Error Handling:**
   - Test invalid block numbers
   - Test insufficient balances
   - Test invalid proposals

### Expected Output

A complete runtime system that:
- Integrates multiple pallets seamlessly
- Provides proper configuration and initialization
- Handles block execution and validation
- Offers comprehensive runtime APIs
- Demonstrates understanding of Substrate architecture
- Shows proper error handling and event management

### Theoretical Context

**Substrate Runtime Architecture:**
- **Modular Design**: Runtime composed of multiple pallets
- **Configuration**: Each pallet configured through traits
- **Integration**: Pallets interact through shared types and APIs
- **Execution**: Runtime processes blocks and extrinsics

**Key Components:**
- **System Pallet**: Core blockchain functionality
- **Executive**: Orchestrates block execution
- **Runtime APIs**: External interfaces for queries
- **Genesis**: Initial blockchain state configuration

**Best Practices:**
- **Type Safety**: Strong typing prevents runtime errors
- **Modularity**: Pallets can be easily added/removed
- **Upgradability**: Runtime can be upgraded without hard forks
- **Efficiency**: Optimized for blockchain performance

This final challenge demonstrates the culmination of Substrate development knowledge, showing how individual components come together to form a complete blockchain runtime.

## Challenge 11: Simple Asset Transfer via Mocked XCM

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

In this challenge, you will simulate a simple asset transfer between two mocked "chains" (Chain A and Chain B). Each chain will have a basic asset management system to handle user balances. The focus is on understanding cross-chain communication concepts through simplified XCM-like message structures.

### Main Concepts Covered

1. **Cross-Chain Messaging**: Basic message structures for chain communication
2. **Asset Management**: Simple balance tracking across chains
3. **Message Processing**: Handling incoming transfer messages
4. **Validation**: Basic checks for transfers (balance, destination)
5. **Event Emission**: Tracking transfer activities

### Structures to Implement

#### **Basic Types:**
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChainId(pub u32);

pub type Balance = u128;
pub type AccountId = String; // Simplified for this challenge

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetId {
    MainToken, // Our single simulated asset
}
```

#### **Transfer Message:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct TransferMessage {
    pub from_chain: ChainId,
    pub to_chain: ChainId,
    pub from_account: AccountId,
    pub to_account: AccountId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

impl TransferMessage {
    pub fn new(
        from_chain: ChainId,
        to_chain: ChainId,
        from_account: AccountId,
        to_account: AccountId,
        asset_id: AssetId,
        amount: Balance,
    ) -> Self {
        Self {
            from_chain,
            to_chain,
            from_account,
            to_account,
            asset_id,
            amount,
        }
    }
}
```

#### **Chain Configuration:**
```rust
pub trait ChainConfig {
    fn chain_id() -> ChainId;
    fn chain_name() -> &'static str;
}
```

#### **Events:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    TransferInitiated {
        from_account: AccountId,
        to_chain: ChainId,
        to_account: AccountId,
        asset_id: AssetId,
        amount: Balance,
    },
    TransferReceived {
        from_chain: ChainId,
        from_account: AccountId,
        to_account: AccountId,
        asset_id: AssetId,
        amount: Balance,
    },
}
```

#### **Errors:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InsufficientBalance,
    InvalidDestinationChain,
    UnsupportedAsset,
    ZeroAmountTransfer,
    AccountNotFound,
    MessageProcessingFailed(String),
}
```

#### **Asset Pallet:**
```rust
use std::collections::HashMap;

pub struct AssetPallet<C: ChainConfig> {
    // Maps (AccountId, AssetId) -> Balance
    balances: HashMap<(AccountId, AssetId), Balance>,
    emitted_events: Vec<Event>,
    _phantom: std::marker::PhantomData<C>,
}
```

### Required Methods of `AssetPallet<C: ChainConfig>`

#### **Constructor and Utilities:**
```rust
impl<C: ChainConfig> AssetPallet<C> {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            emitted_events: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
    
    fn deposit_event(&mut self, event: Event) {
        self.emitted_events.push(event);
    }
    
    pub fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.emitted_events)
    }
    
    pub fn balance_of(&self, account: &AccountId, asset_id: &AssetId) -> Balance {
        self.balances.get(&(account.clone(), *asset_id)).copied().unwrap_or(0)
    }
    
    pub fn set_balance(&mut self, account: AccountId, asset_id: AssetId, amount: Balance) {
        if amount == 0 {
            self.balances.remove(&(account, asset_id));
        } else {
            self.balances.insert((account, asset_id), amount);
        }
    }
    
    fn increase_balance(&mut self, account: &AccountId, asset_id: AssetId, amount: Balance) {
        let current = self.balance_of(account, &asset_id);
        self.set_balance(account.clone(), asset_id, current + amount);
    }
    
    fn decrease_balance(&mut self, account: &AccountId, asset_id: AssetId, amount: Balance) -> Result<(), Error> {
        let current = self.balance_of(account, &asset_id);
        if current < amount {
            return Err(Error::InsufficientBalance);
        }
        self.set_balance(account.clone(), asset_id, current - amount);
        Ok(())
    }
}
```

#### **Transfer Operations:**
```rust
impl<C: ChainConfig> AssetPallet<C> {
    pub fn initiate_transfer(
        &mut self,
        sender: AccountId,
        destination_chain: ChainId,
        beneficiary: AccountId,
        asset_id: AssetId,
        amount: Balance,
    ) -> Result<TransferMessage, Error> {
        // Validation checks
        if destination_chain == C::chain_id() {
            return Err(Error::InvalidDestinationChain);
        }
        
        if amount == 0 {
            return Err(Error::ZeroAmountTransfer);
        }
        
        if asset_id != AssetId::MainToken {
            return Err(Error::UnsupportedAsset);
        }
        
        // Check sender balance
        if self.balance_of(&sender, &asset_id) < amount {
            return Err(Error::InsufficientBalance);
        }
        
        // Debit from sender
        self.decrease_balance(&sender, asset_id, amount)?;
        
        // Create transfer message
        let message = TransferMessage::new(
            C::chain_id(),
            destination_chain,
            sender.clone(),
            beneficiary.clone(),
            asset_id,
            amount,
        );
        
        // Emit event
        self.deposit_event(Event::TransferInitiated {
            from_account: sender,
            to_chain: destination_chain,
            to_account: beneficiary,
            asset_id,
            amount,
        });
        
        Ok(message)
    }
    
    pub fn process_incoming_transfer(
        &mut self,
        message: TransferMessage,
    ) -> Result<(), Error> {
        // Validate message is for this chain
        if message.to_chain != C::chain_id() {
            return Err(Error::MessageProcessingFailed(
                "Message not intended for this chain".to_string()
            ));
        }
        
        // Validate asset
        if message.asset_id != AssetId::MainToken {
            return Err(Error::UnsupportedAsset);
        }
        
        // Credit to beneficiary
        self.increase_balance(&message.to_account, message.asset_id, message.amount);
        
        // Emit event
        self.deposit_event(Event::TransferReceived {
            from_chain: message.from_chain,
            from_account: message.from_account,
            to_account: message.to_account,
            asset_id: message.asset_id,
            amount: message.amount,
        });
        
        Ok(())
    }
}
```

#### **Query Methods:**
```rust
impl<C: ChainConfig> AssetPallet<C> {
    pub fn get_total_balance(&self, account: &AccountId) -> Balance {
        self.balance_of(account, &AssetId::MainToken)
    }
    
    pub fn get_all_balances(&self) -> Vec<(AccountId, AssetId, Balance)> {
        self.balances
            .iter()
            .map(|((account, asset_id), balance)| (account.clone(), *asset_id, *balance))
            .collect()
    }
    
    pub fn get_chain_info() -> (ChainId, &'static str) {
        (C::chain_id(), C::chain_name())
    }
}
```

### Test Configuration

#### **Chain Configurations:**
```rust
struct ChainAConfig;
impl ChainConfig for ChainAConfig {
    fn chain_id() -> ChainId {
        ChainId(1)
    }
    
    fn chain_name() -> &'static str {
        "Chain A"
    }
}

struct ChainBConfig;
impl ChainConfig for ChainBConfig {
    fn chain_id() -> ChainId {
        ChainId(2)
    }
    
    fn chain_name() -> &'static str {
        "Chain B"
    }
}
```

### Tests

Create comprehensive tests covering:

#### **Test Scenarios:**

1. **Basic Transfer:**
   - Test successful transfer initiation
   - Test message processing on destination chain
   - Test balance updates on both chains

2. **Validation Tests:**
   - Test insufficient balance rejection
   - Test zero amount rejection
   - Test same chain transfer rejection
   - Test unsupported asset rejection

3. **Event Emission:**
   - Test TransferInitiated event
   - Test TransferReceived event
   - Test event data accuracy

4. **Integration Tests:**
   - Test complete transfer flow between two chains
   - Test multiple transfers
   - Test balance consistency

5. **Edge Cases:**
   - Test transfer to non-existent account
   - Test processing invalid messages
   - Test boundary conditions

### Example Usage

```rust
fn main() {
    // Create two chains
    let mut chain_a = AssetPallet::<ChainAConfig>::new();
    let mut chain_b = AssetPallet::<ChainBConfig>::new();
    
    // Set initial balances
    chain_a.set_balance("alice".to_string(), AssetId::MainToken, 1000);
    
    println!("Alice balance on Chain A: {}", 
             chain_a.get_total_balance(&"alice".to_string()));
    
    // Initiate transfer from Chain A to Chain B
    let transfer_msg = chain_a.initiate_transfer(
        "alice".to_string(),
        ChainId(2), // Chain B
        "bob".to_string(),
        AssetId::MainToken,
        100,
    ).unwrap();
    
    // Process transfer on Chain B
    chain_b.process_incoming_transfer(transfer_msg).unwrap();
    
    println!("Alice balance on Chain A: {}", 
             chain_a.get_total_balance(&"alice".to_string())); // 900
    println!("Bob balance on Chain B: {}", 
             chain_b.get_total_balance(&"bob".to_string()));   // 100
}
```

### Expected Output

A complete cross-chain asset transfer system that:
- Demonstrates basic cross-chain communication concepts
- Implements asset transfer with proper validation
- Handles balance management across multiple chains
- Provides clear event tracking
- Shows understanding of message-based chain interaction

### Theoretical Context

**Cross-Chain Communication Fundamentals:**
- **Message Passing**: How chains communicate through structured messages
- **Asset Teleportation**: Moving assets between chains by burning and minting
- **Validation**: Ensuring transfers are legitimate and properly formatted
- **State Consistency**: Maintaining accurate balances across chains

**Key Simplifications:**
- **Direct Messaging**: Simple message structure instead of complex XCM format
- **Single Asset**: Focus on one asset type to reduce complexity
- **Immediate Processing**: Direct message processing without consensus delays
- **No Fees**: Simplified economics without transfer fees

This challenge teaches essential cross-chain concepts while maintaining focus on the core mechanisms that enable asset transfers between blockchain networks. 
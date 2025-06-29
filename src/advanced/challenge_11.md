## Challenge 11: Simple Asset Transfer Between Chains

**Difficulty Level:** Advanced
**Estimated Time:** 1 hour

### Objective Description

In this challenge, you will simulate a simple asset transfer between two simulated "chains" (Chain A and Chain B). Each chain will have a basic asset management system to handle user balances. The focus is on understanding cross-chain communication concepts through simplified message structures.

### Main Concepts Covered

1. **Cross-Chain Messaging**: Basic message structures for chain communication
2. **Asset Management**: Simple balance tracking across chains
3. **Message Processing**: Handling incoming transfer messages
4. **Validation**: Basic checks for transfers (balance, destination)

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

#### **Errors:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InsufficientBalance,
    InvalidDestinationChain,
    ZeroAmountTransfer,
}
```

#### **Asset Pallet:**
```rust
use std::collections::HashMap;

pub struct AssetPallet {
    // Maps (AccountId, AssetId) -> Balance
    balances: HashMap<(AccountId, AssetId), Balance>,
    chain_id: ChainId,
}
```

### Provided Methods of `AssetPallet`

#### **Constructor and Utilities:**
```rust
impl AssetPallet {
    pub fn new(chain_id: ChainId) -> Self {
        Self {
            balances: HashMap::new(),
            chain_id,
        }
    }
    
    pub fn get_chain_id(&self) -> ChainId {
        self.chain_id
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

### Methods for You to Implement

#### **1. Transfer Initiation (`initiate_transfer`):**
```rust
impl AssetPallet {
    // TODO: Implement this method
    pub fn initiate_transfer(
        &mut self,
        sender: AccountId,
        destination_chain: ChainId,
        beneficiary: AccountId,
        asset_id: AssetId,
        amount: Balance,
    ) -> Result<TransferMessage, Error> {
        // IMPLEMENT:
        // 1. Validate that destination_chain is different from current chain
        // 2. Validate that amount > 0
        // 3. Check if sender has sufficient balance
        // 4. Decrease sender's balance
        // 5. Create and return TransferMessage
        todo!()
    }
}
```

#### **2. Incoming Transfer Processing (`process_incoming_transfer`):**
```rust
impl AssetPallet {
    // TODO: Implement this method
    pub fn process_incoming_transfer(
        &mut self,
        message: TransferMessage,
    ) -> Result<(), Error> {
        // IMPLEMENT:
        // 1. Validate that the message is intended for this chain
        // 2. Increase beneficiary's balance
        // 3. Return Ok(())
        todo!()
    }
}
```

### Tests to Implement

Create tests that cover:

#### **Test Scenarios:**

1. **Basic Transfer:**
   - Test successful transfer initiation
   - Test message processing on destination chain
   - Test balance updates on both chains

2. **Validation Tests:**
   - Rejection due to insufficient balance
   - Rejection due to zero amount
   - Rejection due to same chain transfer

3. **Integration Test:**
   - Complete transfer flow between two chains
   - Balance consistency verification

### Test Configurations

```rust
// Constants for tests
const CHAIN_A_ID: ChainId = ChainId(1);
const CHAIN_B_ID: ChainId = ChainId(2);
```

### Example Usage

```rust
fn main() {
    // Create two chains
    let mut chain_a = AssetPallet::new(CHAIN_A_ID);
    let mut chain_b = AssetPallet::new(CHAIN_B_ID);
    
    // Set initial balances
    chain_a.set_balance("alice".to_string(), AssetId::MainToken, 1000);
    
    println!("Alice balance on Chain A: {}", 
             chain_a.balance_of(&"alice".to_string(), &AssetId::MainToken));
    
    // Initiate transfer from Chain A to Chain B
    let transfer_msg = chain_a.initiate_transfer(
        "alice".to_string(),
        CHAIN_B_ID,
        "bob".to_string(),
        AssetId::MainToken,
        100,
    ).unwrap();
    
    // Process transfer on Chain B
    chain_b.process_incoming_transfer(transfer_msg).unwrap();
    
    println!("Alice balance on Chain A: {}", 
             chain_a.balance_of(&"alice".to_string(), &AssetId::MainToken)); // 900
    println!("Bob balance on Chain B: {}", 
             chain_b.balance_of(&"bob".to_string(), &AssetId::MainToken));   // 100
}
```

### Expected Output

A simple cross-chain asset transfer system that:
- Demonstrates basic cross-chain communication concepts
- Implements asset transfer with proper validation
- Manages balances across multiple chains
- Shows understanding of message-based chain interaction

### Theoretical Context

**Cross-Chain Communication Fundamentals:**
- **Message Passing**: How chains communicate through structured messages
- **Asset Teleportation**: Moving assets between chains by burning and minting
- **Validation**: Ensuring transfers are legitimate and properly formatted
- **State Consistency**: Maintaining accurate balances across chains

**Simplifications in This Challenge:**
- **Direct Messaging**: Simple structure instead of complex XCM format
- **Single Asset**: Focus on one asset type to reduce complexity
- **Immediate Processing**: Direct processing without consensus delays
- **No Fees**: Simplified economics without transfer fees

This challenge teaches essential cross-chain concepts while maintaining focus on the fundamental mechanisms that enable asset transfers between blockchain networks. 
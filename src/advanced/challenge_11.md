## Challenge 11: Simple Asset Teleportation via Mocked XCM

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

In this challenge, you will simulate a fungible asset transfer between two mocked "chains", Chain A and Chain B. Each chain will have a simple "asset pallet" to manage user balances for a specific asset type (let's call it `SimulatedAsset`).

Chain A will construct a simplified XCM message to instruct Chain B to credit assets to a beneficiary. The focus will be on defining XCM message structures, the logic for debiting assets at the origin and crediting at the destination after "receiving" and processing the message.

**Main Concepts Covered (Simulated):**

1. **Simplified XCM Message Structures:**
   - `SimpleLocation`: To identify a chain or an account within a chain.
   - `SimpleAsset`: To represent the asset and amount to be transferred.
   - `SimpleXcmInstruction`: Basic commands like `WithdrawAssetFromSender` (implicit at origin) and `DepositAssetToBeneficiary`.
   - `SimpleXcmMessage`: A list of instructions.
2. **`Structs` and `Enums`:** To define the above types, `Event`s and `Error`s.
3. **`Traits` and `Generics`:**
   - `ChainConfig`: A trait to configure each chain with `AccountId`, `AssetId`, `Balance` and a unique identifier for the chain itself (`ChainId`).
4. **`std::collections::HashMap`:** To simulate the `StorageMap` of asset balances on each chain.
5. **Business Logic:**
   - Debit assets from sender on origin chain.
   - Process XCM message on destination chain to credit beneficiary.
   - Basic validations (sufficient balance, valid destination chain).
6. **`Option<T>` and `Result<T, E>`:** For error handling and values.
7. **`Pattern Matching`:** To process XCM instructions and origins.

### Detailed Structures to Implement:

#### **`ChainId`:**
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChainId(pub u32);
```

#### **`SimpleLocation<AccountId>`:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleLocation<AccountId> {
    ThisChain, // Refers to current chain
    SiblingChainAccount { chain_id: ChainId, account: AccountId }, // Account on another chain
}
```
*Note: For this challenge, we'll focus on `DepositAssetToBeneficiary` where the beneficiary is an `AccountId` on the destination chain. `SimpleLocation` will be used more to specify the message destination itself and the beneficiary.*

#### **`SimulatedAssetId` and `Balance`:**
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimulatedAssetId {
    MainToken, // Our single simulated asset
}
pub type Balance = u128;
```

#### **`SimpleAsset`:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleAsset {
    pub id: SimulatedAssetId,
    pub amount: Balance,
}
```

#### **`SimpleXcmInstruction<AccountId>`:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleXcmInstruction<AccountId> {
    // Instruction for destination chain
    DepositAssetToBeneficiary {
        asset: SimpleAsset,
        beneficiary: AccountId, // Account on destination chain
    },
    // We could have WithdrawAsset, but at origin this will be an implicit action
    // of debiting from sender before sending XCM.
}
```

#### **`SimpleXcmMessage<AccountId>`:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleXcmMessage<AccountId> {
    pub instructions: Vec<SimpleXcmInstruction<AccountId>>,
}
```

#### **`ChainConfig` Trait:**
```rust
pub trait ChainConfig {
    type AccountId: Clone + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
    type AssetId: Clone + Copy + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
    type Balance: Clone + Copy + PartialEq + core::fmt::Debug + PartialOrd + std::ops::AddAssign + std::ops::SubAssign;

    fn this_chain_id() -> ChainId;
}
```

#### **`Event<C: ChainConfig>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<C: ChainConfig> {
    AssetTeleportInitiated {
        from_account: C::AccountId,
        to_chain: ChainId,
        to_account: C::AccountId,
        asset_id: C::AssetId,
        amount: C::Balance,
    },
    AssetDepositedViaXcm {
        from_chain_hint: Option<ChainId>, // Where XCM may have come from (optional)
        to_account: C::AccountId,
        asset_id: C::AssetId,
        amount: C::Balance,
    },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InsufficientBalance,
    InvalidDestinationChain, // If trying to send to self or invalid chain
    UnsupportedAsset,
    CannotSendZeroAmount,
    // Errors in XCM processing by destination chain
    XcmProcessingError(String), // Generic error for XCM processing
}
```

#### **`AssetPallet<C: ChainConfig>` Struct (for each chain):**
```rust
pub struct AssetPallet<C: ChainConfig> {
    // Maps (AccountId, AssetId) -> Balance
    balances: std::collections::HashMap<(C::AccountId, C::AssetId), C::Balance>,
    emitted_events: Vec<Event<C>>,
    // phantom: std::marker::PhantomData<C>, // If not using C in fields, but only in associated types and methods
}
```

### Methods of `AssetPallet<C: ChainConfig>`:

- `pub fn new() -> Self`
- `fn deposit_event(&mut self, event: Event<C>)`
- `pub fn take_events(&mut self) -> Vec<Event<C>>`
- `pub fn balance_of(&self, account: &C::AccountId, asset_id: &C::AssetId) -> C::Balance` (returns 0 if no entry)
- `pub fn set_balance(&mut self, account: C::AccountId, asset_id: C::AssetId, amount: C::Balance)` (helper for tests)

#### **`pub fn initiate_teleport_asset(`**
- `&mut self,`
- `sender: C::AccountId,`
- `destination_chain_id: ChainId,`
- `beneficiary_on_destination: C::AccountId,`
- `asset_id_to_send: C::AssetId,`
- `amount_to_send: C::Balance`
- `) -> Result<SimpleXcmMessage<C::AccountId>, Error>`
  - Check if `destination_chain_id` is not `C::this_chain_id()`. If it is, `Error::InvalidDestinationChain`.
  - Check if `amount_to_send` > 0. If not, `Error::CannotSendZeroAmount`.
  - Check if `asset_id_to_send` is `SimulatedAssetId::MainToken` (or supported asset). If not, `Error::UnsupportedAsset`.
  - Check sender's balance for `asset_id_to_send`. If insufficient, `Error::InsufficientBalance`.
  - Debit `amount_to_send` from `sender`.
  - Construct a `SimpleXcmMessage` with a `DepositAssetToBeneficiary { asset: SimpleAsset { id: asset_id_to_send (converted to SimulatedAssetId), amount: amount_to_send }, beneficiary: beneficiary_on_destination }` instruction.
  - Emit `Event::AssetTeleportInitiated` event.
  - Return `Ok(xcm_message)` that would be "sent" to `destination_chain_id`.

#### **`pub fn process_incoming_xcm_message(`**
- `&mut self,`
- `source_chain_hint: Option<ChainId>,`
- `message: SimpleXcmMessage<C::AccountId>`
- `) -> Result<(), Error>`
  - Iterate over `message.instructions`.
  - For each `SimpleXcmInstruction::DepositAssetToBeneficiary`:
    - Check if `asset.id` is `SimulatedAssetId::MainToken`. If not, `Error::XcmProcessingError("Unsupported asset in XCM".into())`.
    - Credit `asset.amount` to `beneficiary`. (Add to existing balance or create new entry).
    - Emit `Event::AssetDepositedViaXcm` event.
  - If any instruction is not supported or fails, return an `Error::XcmProcessingError`.
  - Return `Ok(())` if all instructions are processed successfully.

### Tests

You will need two instances of `AssetPallet`, one for Chain A and another for Chain B, each with their own `ChainConfig` (especially different `this_chain_id()`).

#### **Test Configuration:**
- `TestAccountId` (e.g., `String` or `u32`)
- `TestChainAConfig` and `TestChainBConfig` implementing `ChainConfig`.
  ```rust
  // Example
  struct TestChainAConfig;
  impl ChainConfig for TestChainAConfig {
      type AccountId = String;
      type AssetId = SimulatedAssetId;
      type Balance = u128;
      fn this_chain_id() -> ChainId { ChainId(1) }
  }
  // Similar for TestChainBConfig with ChainId(2)
  ```

#### **Test Scenarios:**
- **Successful Teleportation:**
  - Set balance for sender on Chain A
  - Initiate teleport from Chain A to Chain B
  - Verify XCM message is created correctly
  - Process XCM message on Chain B
  - Verify balances updated correctly on both chains
  - Verify events emitted

- **Error Cases:**
  - Insufficient balance on origin
  - Zero amount transfer
  - Invalid destination chain (same as origin)
  - Unsupported asset type
  - XCM processing errors

- **Edge Cases:**
  - Multiple teleports
  - Large amounts
  - Non-existent accounts

### Expected Output

A complete XCM teleportation system that:
- Simulates cross-chain asset transfers
- Implements proper validation and error handling
- Demonstrates XCM message construction and processing
- Shows understanding of cross-chain communication patterns
- Handles various error scenarios gracefully

### Theoretical Context

**XCM (Cross-Consensus Message Format):**
- **Purpose:** Enable communication between different consensus systems
- **Messages:** Structured instructions for cross-chain operations
- **Teleportation:** Moving assets by burning on origin and minting on destination
- **Trust:** Requires trust relationship between chains

**Asset Management:**
- **Balances:** Tracked separately on each chain
- **Validation:** Ensure sufficient funds before transfer
- **Events:** Notify about successful operations

**Cross-Chain Security:**
- **Validation:** All operations must be validated on both chains
- **Atomicity:** Operations should be atomic (succeed or fail completely)
- **Trust Model:** Chains must trust each other for teleportation

This challenge demonstrates the fundamental concepts of cross-chain asset transfers and the XCM protocol used in the Polkadot ecosystem. 
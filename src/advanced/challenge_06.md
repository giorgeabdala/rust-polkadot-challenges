## Challenge 6: Unsigned Ping with Validation

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a simulated pallet that accepts an unsigned transaction called "ping". When valid, this transaction will record the current block number when it was received. The core part of the challenge is implementing the `ValidateUnsigned` trait to protect this functionality, primarily by limiting the frequency at which these pings can be sent to prevent spam.

**Concepts and Structures to Implement/Simulate:**

1. **`Pallet<T: Config>`:** The main struct of our pallet.
   - Will store `last_ping_block: Option<T::BlockNumber>` to track the block of the last successful ping.
   - Will maintain a list of emitted events: `emitted_events: Vec<Event>`.

2. **`Config` Trait:**
   - `type BlockNumber: ...` (with necessary operations like `Sub`, `PartialOrd`, `Copy`, `Default`).
   - `type PingInterval: Get<Self::BlockNumber>;` (defines the minimum number of blocks that must pass between unsigned pings).

3. **`Call<T: Config>` Enum:**
   - Will contain one variant: `PingUnsigned`.

4. **`Event` Enum:**
   - `PingReceived { block_number: T::BlockNumber }`

5. **`Error` Enum:**
   - `TooEarlyToPing` (if a ping is attempted before the `PingInterval`).
   - `InvalidCall` (if `validate_unsigned` is called with an unexpected `Call`).

6. **`ValidateUnsigned` Trait (Simulated):**
   - You will implement this trait for your `Pallet<T>`.
   - It will have two main methods: `validate_unsigned` and `pre_dispatch`.

7. **`ValidTransaction` and `TransactionValidityError` (Simulated):**
   - Simplified structures to represent validation results, as used by the `ValidateUnsigned` trait.

### Detailed Structures to Implement:

#### **`Config` Trait:**
```rust
// Helper trait to get configuration values
pub trait Get<V> {
    fn get() -> V;
}

pub trait Config {
    type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::ops::Sub<Output = Self::BlockNumber> + core::fmt::Debug;
    type PingInterval: Get<Self::BlockNumber>; // Minimum blocks between pings
}
```

#### **`Call<T: Config>` Enum:**
    ```rust
// The _phantom is to use the generic T, simulating how it would be in FRAME
#[derive(Clone, Debug, PartialEq)]
pub enum Call<T: Config> {
    PingUnsigned,
    _Phantom(core::marker::PhantomData<T>), // To use T
}
```

#### **`Event<BlockNumber>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<BlockNumber> {
    PingReceived { block_number: BlockNumber },
}
```

#### **`Error` Enum:**
    ```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    TooEarlyToPing,
    InvalidCall, // If validate_unsigned is called with a Call that is not PingUnsigned
}
```

#### **Validation Structures (Simulated):**
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct ValidTransaction {
    pub priority: u64,
    pub requires: Vec<Vec<u8>>,
    pub provides: Vec<Vec<u8>>,
    pub longevity: u64, // In number of blocks
    pub propagate: bool,
}

impl Default for ValidTransaction {
    fn default() -> Self {
        Self {
            priority: 0,
            requires: vec![],
            provides: vec![],
            longevity: 5, // Default validity of 5 blocks
            propagate: true,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TransactionValidityError {
    Invalid(Error), // Using our pallet's Error enum
    Unknown,      // For other types of validity errors
}

pub type TransactionValidity = Result<ValidTransaction, TransactionValidityError>;
```

#### **`ValidateUnsigned<T: Config>` Trait:**
```rust
pub trait ValidateUnsigned<T: Config> {
    fn validate_unsigned(
        // In FRAME, TransactionSource would be a parameter here. Omitted for simplicity.
        call: &Call<T>,
        current_block: T::BlockNumber,
        last_ping_block: Option<T::BlockNumber>, // Passing relevant state
    ) -> TransactionValidity;

    fn pre_dispatch(call: &Call<T>) -> Result<(), TransactionValidityError>;
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
pub struct Pallet<T: Config> {
    last_ping_block: Option<T::BlockNumber>,
    emitted_events: Vec<Event<T::BlockNumber>>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            last_ping_block: None,
            emitted_events: Vec::new(),
        }
    }

    // Function that would be called by the "runtime" after pre_dispatch succeeds
    pub fn ping_unsigned_impl(&mut self, current_block: T::BlockNumber) -> Result<(), Error> {
        // Time validation was already done in validate_unsigned.
        // Here, we just execute the dispatch logic.
        self.last_ping_block = Some(current_block);
        self.emitted_events.push(Event::PingReceived { block_number: current_block });
        Ok(())
    }

    pub fn take_events(&mut self) -> Vec<Event<T::BlockNumber>> {
        std::mem::take(&mut self.emitted_events)
    }

    // Helper method for tests to set initial state
    #[cfg(test)]
    pub fn set_last_ping_block(&mut self, block: Option<T::BlockNumber>) {
        self.last_ping_block = block;
    }
}
```

### Implementation of `ValidateUnsigned for Pallet<T>`:

This is the crucial part. You will implement this trait for your `Pallet` struct.

#### **`validate_unsigned(...)`:**
1. Check if the `call` is `Call::PingUnsigned`. If not, return `Err(TransactionValidityError::Invalid(Error::InvalidCall))`.
2. Use the `last_ping_block` (passed as parameter, since the trait doesn't have `&self`) and `current_block` to check if `current_block - last_ping_block >= T::PingInterval::get()`.
   - If `last_ping_block` is `None`, the ping is allowed.
3. If it's too early, return `Err(TransactionValidityError::Invalid(Error::TooEarlyToPing))`.
4. If valid, construct and return `Ok(ValidTransaction { ... })`.
   - `provides`: `vec![b"my_pallet_ping_unsigned_tag".to_vec()]`. This helps the transaction pool not accept multiple identical pings at the same time.
   - `longevity`: A reasonable value, e.g., `T::PingInterval::get()` (the ping is valid until the next one can be sent).
   - `priority`: Can be a default value, or maybe higher if pings are important.
   - `propagate`: `true`.

#### **`pre_dispatch(...)`:**
1. Check if the `call` is `Call::PingUnsigned`. If not (which would be strange if `validate_unsigned` passed), return an appropriate error (e.g., `Err(TransactionValidityError::Invalid(Error::InvalidCall))`).
2. For this challenge, if the call is `PingUnsigned`, it can simply return `Ok(())`, since the main validation logic (timing) was already done in `validate_unsigned`. In more complex scenarios, `pre_dispatch` might redo some light checks or prepare state.

### Tests

Create a `tests` module. You will need:
- `TestBlockNumber` (e.g., `u64`).
- `TestPingInterval` struct that implements `Get<TestBlockNumber>`.
- `TestConfig` struct that implements `crate::Config`.

**Test Scenarios:**

- **Successful validation:**
  - No previous pings: `validate_unsigned` should return `Ok(ValidTransaction)`.
  - After sufficient interval: `validate_unsigned` should return `Ok(ValidTransaction)`.
- **Validation failure:**
  - Ping too early: `validate_unsigned` should return `Err(TransactionValidityError::Invalid(Error::TooEarlyToPing))`.
  - Invalid call for `validate_unsigned`: Should return `Err(TransactionValidityError::Invalid(Error::InvalidCall))`.
- **`pre_dispatch`:**
  - Valid call: `pre_dispatch` should return `Ok(())`.

### Expected Output

A set of Rust functions that pass all proposed unit tests, demonstrating correct manipulation of unsigned transaction validation, timing constraints, and proper error handling.

### Theoretical Context

#### **Unsigned Transactions:**
- Transactions that don't require a signature from any account
- Used for data that anyone can submit (like oracle data, inherents)
- Must be carefully validated to prevent spam
- Common in off-chain workers and inherent data providers

#### **`ValidateUnsigned` Trait:**
- Core mechanism for validating unsigned transactions in Substrate
- Two-phase validation: `validate_unsigned` (in transaction pool) and `pre_dispatch` (before execution)
- Must prevent spam while allowing legitimate unsigned transactions
- Used by pallets like `pallet-im-online` for validator heartbeats

#### **Transaction Pool:**
- Validates transactions before including them in blocks
- Uses `provides` and `requires` tags to manage transaction dependencies
- `longevity` determines how long a transaction stays valid
- `priority` affects transaction ordering in the pool

This challenge demonstrates the critical balance between allowing useful unsigned transactions while preventing network abuse. 
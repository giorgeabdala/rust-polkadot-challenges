## Challenge 1: Basic Counter Pallet (Fundamentals Review)

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a basic counter pallet that simulates the fundamental structure of a FRAME pallet. This challenge serves as a review of essential concepts before moving on to more advanced topics.

The pallet should allow:
- Incrementing a counter
- Decrementing a counter (with validation to prevent underflow)
- Resetting the counter to zero
- Getting the current counter value

**Main Concepts Covered:**
1. **Basic Pallet Structure:** `Config` trait, storage, calls, events, and errors
2. **Storage:** Using `StorageValue` to store a single value
3. **Dispatchable Functions:** Public functions that can be called externally
4. **Events:** To notify about state changes
5. **Error Handling:** Custom errors for the pallet
6. **Weight System:** Basic understanding of transaction weights

### Detailed Structures to Implement:

#### **`Config` Trait:**
```rust
pub trait Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type WeightInfo: WeightInfo;
}
```

#### **Storage:**
```rust
#[pallet::storage]
#[pallet::getter(fn counter)]
pub type Counter<T> = StorageValue<_, u32, ValueQuery>;
```

#### **Events:**
```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Counter was incremented. [new_value]
    CounterIncremented { new_value: u32 },
    /// Counter was decremented. [new_value]
    CounterDecremented { new_value: u32 },
    /// Counter was reset to zero.
    CounterReset,
}
```

#### **Errors:**
```rust
#[pallet::error]
pub enum Error<T> {
    /// Cannot decrement counter below zero
    CounterUnderflow,
    /// Counter has reached maximum value
    CounterOverflow,
}
```

#### **Dispatchable Functions:**
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Increment the counter by 1
    #[pallet::weight(T::WeightInfo::increment())]
    #[pallet::call_index(0)]
    pub fn increment(origin: OriginFor<T>) -> DispatchResult {
        // Implementation here
    }

    /// Decrement the counter by 1
    #[pallet::weight(T::WeightInfo::decrement())]
    #[pallet::call_index(1)]
    pub fn decrement(origin: OriginFor<T>) -> DispatchResult {
        // Implementation here
    }

    /// Reset counter to zero
    #[pallet::weight(T::WeightInfo::reset())]
    #[pallet::call_index(2)]
    pub fn reset(origin: OriginFor<T>) -> DispatchResult {
        // Implementation here
    }
}
```

#### **Weight Information:**
```rust
pub trait WeightInfo {
    fn increment() -> Weight;
    fn decrement() -> Weight;
    fn reset() -> Weight;
}

impl WeightInfo for () {
    fn increment() -> Weight {
        Weight::from_parts(10_000, 0)
    }
    fn decrement() -> Weight {
        Weight::from_parts(10_000, 0)
    }
    fn reset() -> Weight {
        Weight::from_parts(10_000, 0)
    }
}
```

### Implementation Requirements:

1. **`increment()`:**
   - Check for overflow before incrementing
   - Update storage
   - Emit `CounterIncremented` event
   - Return `Ok(())`

2. **`decrement()`:**
   - Check if counter is greater than 0
   - If zero, return `Error::<T>::CounterUnderflow`
   - Update storage
   - Emit `CounterDecremented` event

3. **`reset()`:**
   - Set counter to 0
   - Emit `CounterReset` event

### Tests

Create a test module with the following scenarios:
- **Successful increment:** Verify counter increases and event is emitted
- **Successful decrement:** Verify counter decreases and event is emitted
- **Underflow prevention:** Verify error when trying to decrement from 0
- **Reset functionality:** Verify counter resets to 0 and event is emitted
- **Multiple operations:** Test sequence of operations

### Expected Output

A complete FRAME pallet implementation that:
- Compiles without errors
- Passes all unit tests
- Follows FRAME conventions and best practices
- Demonstrates proper error handling and event emission

### Theoretical Context

This challenge reviews fundamental FRAME concepts:

- **Pallet Structure:** Every FRAME pallet follows a standard structure with `Config`, storage, calls, events, and errors
- **Storage:** `StorageValue` is used for single values, while `StorageMap` is for key-value pairs
- **Dispatchable Functions:** Functions marked with `#[pallet::call]` can be called from outside the runtime
- **Events:** Used to notify external systems about state changes
- **Weights:** Every dispatchable function must specify its computational cost
- **Error Handling:** Custom errors provide clear feedback about operation failures

This foundation is essential for understanding more complex FRAME concepts in subsequent challenges.

## Challenge 1: Basic Pallet Simulator (Pure Rust)

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a basic pallet simulator in pure Rust that demonstrates the fundamental concepts of a FRAME pallet without requiring external Substrate dependencies. This challenge serves as a review of essential concepts before advancing to more complex topics.

The simulator should allow:
- Incrementing a counter
- Decrementing a counter (with validation to prevent underflow)
- Resetting the counter to zero
- Getting the current counter value
- Simulated event system
- Custom error system

**Main Concepts Covered:**
1. **Simulated Pallet Structure:** Traits, storage, calls, events, and errors
2. **Simulated Storage:** Using Rust structures to simulate storage
3. **Dispatch Functions:** Public functions that can be called externally
4. **Events:** To notify about state changes
5. **Error Handling:** Custom errors for the pallet
6. **Weight System:** Basic understanding of transaction weights

### Detailed Structures to Implement:

#### **Configuration Trait:**
```rust
pub trait Config {
    type Event: From<Event> + Clone + PartialEq + std::fmt::Debug;
    type WeightInfo: WeightInfo;
}
```

#### **Simulated Storage:**
```rust
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Storage {
    counter: Arc<Mutex<u32>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_counter(&self) -> u32 {
        *self.counter.lock().unwrap()
    }

    pub fn set_counter(&self, value: u32) {
        *self.counter.lock().unwrap() = value;
    }
}
```

#### **Event System:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Counter was incremented. [new_value]
    CounterIncremented { new_value: u32 },
    /// Counter was decremented. [new_value]
    CounterDecremented { new_value: u32 },
    /// Counter was reset to zero.
    CounterReset,
}
```

#### **Error System:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Cannot decrement counter below zero
    CounterUnderflow,
    /// Counter reached maximum value
    CounterOverflow,
}

pub type DispatchResult = Result<(), Error>;
```

#### **Weight System:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight(pub u64);

pub trait WeightInfo {
    fn increment() -> Weight;
    fn decrement() -> Weight;
    fn reset() -> Weight;
}

pub struct DefaultWeightInfo;

impl WeightInfo for DefaultWeightInfo {
    fn increment() -> Weight {
        Weight(10_000)
    }
    fn decrement() -> Weight {
        Weight(10_000)
    }
    fn reset() -> Weight {
        Weight(10_000)
    }
}
```

#### **Main Pallet:**
        ```rust
pub struct Pallet<T: Config> {
    storage: Storage,
    events: Vec<T::Event>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            events: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Increment the counter by 1
    pub fn increment(&mut self) -> DispatchResult {
        // Implementation here
        todo!()
    }

    /// Decrement the counter by 1
    pub fn decrement(&mut self) -> DispatchResult {
        // Implementation here
        todo!()
    }

    /// Reset counter to zero
    pub fn reset(&mut self) -> DispatchResult {
        // Implementation here
        todo!()
    }

    /// Get current counter value
    pub fn get_counter(&self) -> u32 {
        self.storage.get_counter()
    }

    /// Get emitted events
    pub fn get_events(&self) -> &[T::Event] {
        &self.events
    }

    /// Clear events (simulates event consumption)
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Helper function to emit events
    fn deposit_event(&mut self, event: Event) {
        self.events.push(T::Event::from(event));
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
   - If zero, return `Error::CounterUnderflow`
   - Update storage
   - Emit `CounterDecremented` event

3. **`reset()`:**
   - Set counter to 0
   - Emit `CounterReset` event

### Test Configuration:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TestEvent(Event);

impl From<Event> for TestEvent {
    fn from(event: Event) -> Self {
        TestEvent(event)
    }
}

pub struct TestConfig;

impl Config for TestConfig {
    type Event = TestEvent;
    type WeightInfo = DefaultWeightInfo;
}

type TestPallet = Pallet<TestConfig>;
```

### Tests

Create a test module with the following scenarios:
- **Successful increment:** Verify counter increases and event is emitted
- **Successful decrement:** Verify counter decreases and event is emitted
- **Underflow prevention:** Verify error when trying to decrement from 0
- **Reset functionality:** Verify counter resets to 0 and event is emitted
- **Multiple operations:** Test sequence of operations
- **Event system:** Verify events are emitted correctly

### Test Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_works() {
        let mut pallet = TestPallet::new();
        
        assert_eq!(pallet.get_counter(), 0);
        
        let result = pallet.increment();
        assert!(result.is_ok());
        assert_eq!(pallet.get_counter(), 1);
        
        let events = pallet.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], TestEvent(Event::CounterIncremented { new_value: 1 }));
    }

    #[test]
    fn decrement_underflow_fails() {
        let mut pallet = TestPallet::new();
        
        let result = pallet.decrement();
        assert_eq!(result, Err(Error::CounterUnderflow));
        assert_eq!(pallet.get_counter(), 0);
        assert_eq!(pallet.get_events().len(), 0);
    }

    // Add more tests here...
}
```

### Expected Output

A complete pallet simulator implementation that:
- Compiles without errors
- Passes all unit tests
- Follows Rust conventions and best practices
- Demonstrates proper error handling and event emission
- Simulates fundamental FRAME concepts without external dependencies

### Theoretical Context

This challenge simulates fundamental FRAME concepts:

- **Pallet Structure:** Every FRAME pallet follows a standard structure with `Config`, storage, calls, events, and errors
- **Storage:** Simulates how Substrate stores state data
- **Dispatch Functions:** Simulates functions that can be called externally
- **Events:** Simulates notifications about state changes
- **Weights:** Simulates the computational cost system
- **Error Handling:** Simulates custom errors that provide clear feedback

This foundation is essential for understanding more complex FRAME concepts in subsequent challenges, but without the need to set up a complete Substrate environment.

**Advantages of This Approach:**
- Focus on fundamental concepts without setup complexity
- Pure Rust, no external dependencies
- Faster to implement and test
- Prepares for real FRAME concepts
- Allows experimentation without blockchain overhead

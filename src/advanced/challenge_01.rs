use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Counter was incremented. [new_value]
    CounterIncremented { new_value: u32 },
    /// Counter was decremented. [new_value]
    CounterDecremented { new_value: u32 },
    /// Counter was reset to zero.
    CounterReset,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Cannot decrement counter below zero
    CounterUnderflow,
    /// Counter reached maximum value
    CounterOverflow,
}

pub type DispatchResult = Result<(), Error>;
pub trait Config {
    type Event: From<Event>  + Clone + PartialEq + Debug;
    type WeightInfo: WeightInfo;
}

#[derive(Debug, Clone)]
pub struct Storage {
    counter: Arc<Mutex<u32>>,

}

impl Storage {
    pub fn new() -> Self {
        Self {
            counter:Arc::new(Mutex::new(0))
        }
    }

    pub fn get_counter(&self) -> u32 {
        *self.counter.lock().unwrap()
    }

    pub fn set_counter(&self, value: u32) {
        *self.counter.lock().unwrap() = value;
    }

}

// Weight: Substrate's computational cost measurement unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight(pub u64);

// WeightInfo: provides benchmarked weights for dispatchable functions
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

pub struct Pallet<T: Config> {
    storage: Storage,
    events: Vec<T::Event>,
    _phantom: std::marker::PhantomData<T>
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            events: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn increment(&mut self) -> DispatchResult {
        let current_value = self.get_counter();
        let new_value = current_value.checked_add(1).ok_or(Error::CounterOverflow)?;
        
        self.storage.set_counter(new_value);
        self.deposit_event(Event::CounterIncremented { new_value });
        Ok(())
    }

    pub fn decrement(&mut self) -> DispatchResult {
        let current_value = self.get_counter();
        let new_value = current_value.checked_sub(1).ok_or(Error::CounterUnderflow)?;
        self.storage.set_counter(new_value);
        self.deposit_event(Event::CounterDecremented {new_value});
        Ok(())
    }

    pub fn reset(&mut self) -> DispatchResult {
        self.storage.set_counter(0);
        self.deposit_event(Event::CounterReset);
        Ok(())
    }

    pub fn get_counter(&self) -> u32 {
        self.storage.get_counter()
    }

    pub fn get_events(&self) -> &[T::Event] {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    fn deposit_event(&mut self, event: Event) {
        self.events.push(T::Event::from(event))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
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
    fn decrement_works() {
        let mut pallet = TestPallet::new();
        assert_eq!(pallet.get_counter(), 0);
        assert!(pallet.increment().is_ok());
        assert_eq!(pallet.get_counter(), 1);
        let result  = pallet.decrement();
        assert!(result.is_ok());
        assert_eq!(pallet.get_counter(), 0);

        let events = pallet.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], TestEvent(Event::CounterIncremented {new_value: 1}));
        assert_eq!(events[1], TestEvent(Event::CounterDecremented {new_value: 0}));
    }


    #[test]
    fn decrement_underflow_fails() {
        let mut pallet = TestPallet::new();
        let result = pallet.decrement();
        assert_eq!(result, Err(Error::CounterUnderflow));
        assert_eq!(pallet.get_counter(), 0);
        assert_eq!(pallet.get_events().len(), 0);
    }

    #[test]
    fn increment_overflow_fails() {
        let mut pallet = TestPallet::new();
        pallet.storage.set_counter(u32::MAX);
        let result = pallet.increment();
        assert!(result.is_err());
        assert_eq!(result, Err(Error::CounterOverflow));
        assert_eq!(pallet.get_counter(), u32::MAX);
        assert_eq!(pallet.get_events().len(), 0);
    }

    #[test]
    fn reset_works() {
        let mut pallet = TestPallet::new();
        let _ = pallet.increment();
        assert_eq!(pallet.get_counter(), 1);
        let result = pallet.reset();
        assert!(result.is_ok());
        assert_eq!(pallet.get_counter(), 0);

        let events = pallet.get_events();
        assert_eq!(events.len(),2);
        assert_eq!(events[0], TestEvent(Event::CounterIncremented {new_value: 1}));
        assert_eq!(events[1], TestEvent(Event::CounterReset));
    }

    #[test]
    fn multiple_operations() {
        let mut pallet = TestPallet::new();

        // Increment twice
        assert!(pallet.increment().is_ok()); // Counter: 1
        assert_eq!(pallet.get_counter(), 1);
        assert!(pallet.increment().is_ok()); // Counter: 2
        assert_eq!(pallet.get_counter(), 2);

        // Check events for increments
        let events_after_increments = pallet.get_events();
        assert_eq!(events_after_increments.len(), 2);
        assert_eq!(events_after_increments[0], TestEvent(Event::CounterIncremented { new_value: 1 }));
        assert_eq!(events_after_increments[1], TestEvent(Event::CounterIncremented { new_value: 2 }));
        pallet.clear_events(); // Clear events before next operations

        // Decrement once
        assert!(pallet.decrement().is_ok()); // Counter: 1
        assert_eq!(pallet.get_counter(), 1);

        // Check event for decrement
        let events_after_decrement = pallet.get_events();
        assert_eq!(events_after_decrement.len(), 1);
        assert_eq!(events_after_decrement[0], TestEvent(Event::CounterDecremented { new_value: 1 }));
        pallet.clear_events();

        // Reset
        assert!(pallet.reset().is_ok()); // Counter: 0
        assert_eq!(pallet.get_counter(), 0);

        // Check event for reset
        let events_after_reset = pallet.get_events();
        assert_eq!(events_after_reset.len(), 1);
        assert_eq!(events_after_reset[0], TestEvent(Event::CounterReset));
        pallet.clear_events();

        // Try to decrement from 0 (should fail)
        let decrement_result_at_zero = pallet.decrement();
        assert_eq!(decrement_result_at_zero, Err(Error::CounterUnderflow));
        assert_eq!(pallet.get_counter(), 0); // Counter should remain 0
        assert_eq!(pallet.get_events().len(), 0); // No events on failure
    }

}






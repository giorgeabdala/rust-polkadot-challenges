
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub enum CounterError {
    ExceedsMax,
    Underflow,
}

pub struct Counter<T, const N: usize> {
    value: T,
    max: T,
    events: [Option<String>; N], 
    event_count: usize,          
}

impl<T, const N: usize> Counter<T, N>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd + Display,
{
    pub fn new(initial: T, max: T) -> Self {
        let events = std::array::from_fn(|_| None);
        Counter {
            value: initial,
            max,
            events,
            event_count: 0,
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn increment(&mut self, amount: T) -> Result<T, CounterError> {
        let new_value = self.value + amount;

        if new_value > self.max {
            return Err(CounterError::ExceedsMax);
        }
        self.value = new_value;
        self.record_event(format!("Incremented by {}", amount));
        Ok(self.value)
    }

    pub fn decrement(&mut self, amount: T) -> Result<T, CounterError> {
        if amount > self.value {
            return Err(CounterError::Underflow);
        }
        let new_value = self.value - amount;
        self.value = new_value;
        self.record_event(format!("Decremented by {}", amount));
        Ok(self.value)
    }
    
    fn record_event(&mut self, msg: String) {
        let idx = self.event_count % N; 
        self.events[idx] = Some(msg);
        self.event_count += 1;
    }

    pub fn events(&self) -> &[Option<String>; N] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_basic_operations_and_events() {
        let mut c: Counter<u32, 3> = Counter::new(0, 5);
        // Initial state
        assert_eq!(c.get(), 0);
        assert_eq!(c.events(), &[None, None, None]);
        // Valid increments
        assert_eq!(c.increment(3).unwrap(), 3);
        assert_eq!(c.events()[0], Some("Incremented by 3".into()));
        assert_eq!(c.increment(1).unwrap(), 4);
        assert_eq!(c.events()[1], Some("Incremented by 1".into()));

        // Valid decrement
        assert_eq!(c.decrement(2).unwrap(), 2);
        assert_eq!(c.events()[2], Some("Decremented by 2".into()));

        // Overflow and underflow
        assert_eq!(c.increment(10).unwrap_err(), CounterError::ExceedsMax);
        assert_eq!(c.decrement(5).unwrap_err(), CounterError::Underflow);

        // Circular buffer (wrap)
        assert_eq!(c.increment(1).unwrap(), 3);
        // The next event overwrites index 0
        assert_eq!(c.events()[0], Some("Incremented by 1".into()));
    }

    #[test]
    fn counter_different_type_and_size() {
        // Using i64 and buffer size 2
        let mut c: Counter<i64, 2> = Counter::new(-10, 10);
        assert_eq!(c.increment(5).unwrap(), -5);
        // Check event buffer
        let ev = c.events();
        assert!(ev[0].as_ref().unwrap().contains("Incremented by"));
    }
}

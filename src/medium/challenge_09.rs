use std::thread;
use std::sync::{Arc, Mutex};

// Simple counter for Arc<Mutex> demonstration
#[derive(Debug, Clone)]
struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

// Simple message for channel demonstration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Message {
    id: u32,
    content: String,
}

impl SharedCounter {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    // TODO: Implement this method
    pub fn increment(&self) {
        let mut num = self.value.lock().expect("Failed to acquire mutex lock");
        *num += 1;
    }

    pub fn get(&self) -> i32 {
        let num = self.value.lock().expect("Failed to acquire mutex lock");
        *num
    }
}

impl Message {
    pub fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
}

// TODO: Implement this function
fn spawn_workers() -> Vec<thread::JoinHandle<()>> {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for id in 0..3 {
        let handle = thread::spawn(move || {
            println!("Thread {} working", id);
        });
        handles.push(handle);
    }
    handles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::mpsc;

    #[test]
    fn test_shared_counter() {
        // This test now validates the public API of `SharedCounter`.
        let counter = SharedCounter::new();
        let mut handles = Vec::new();

        for _ in 0..3 {
            // We clone the `counter` struct itself. Since it derives `Clone`,
            // this clones the Arc internally, increasing its reference count.
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || { // move: transfer ownership to thread
                for _ in 0..10 {
                    // Now, we call the `increment` method directly.
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to finish their work.
        for handle in handles {
            handle.join().unwrap();
        }

        // Assert that the final value is the sum of all increments.
        assert_eq!(counter.get(), 30);
    }

    #[test]
    fn test_channel_communication() {
        let (tx, rx) = mpsc::channel();
        // We use a HashSet to track the messages sent, allowing for order-independent comparison.
        let mut sent_messages = HashSet::new();

        // Prepare and send message 1
        let msg1 = Message::new(1, "First message".to_string());
        sent_messages.insert(msg1.clone());
        let tx1 = tx.clone(); // Clone the sender for the first thread
        thread::spawn(move || {
            tx1.send(msg1).unwrap();
        });

        // Prepare and send message 2
        let msg2 = Message::new(2, "Second message".to_string());
        sent_messages.insert(msg2.clone());
        let tx2 = tx.clone(); // Clone the sender for the second thread
        thread::spawn(move || {
            tx2.send(msg2).unwrap();
        });

        // Essential: Drop the original sender. This closes the channel once all
        // clones (`tx1`, `tx2`) are dropped (when their threads finish).
        // This signals to the receiver that no more messages will be sent.
        drop(tx);

        // We collect all received messages into a HashSet to avoid ordering issues.
        // `rx.iter()` creates an iterator that blocks and yields messages until the channel is closed.
        let received_messages: HashSet<Message> = rx.iter().collect();

        // Now we compare the sets of messages, which is order-independent.
        assert_eq!(sent_messages, received_messages);
        assert_eq!(received_messages.len(), 2);
    }
}

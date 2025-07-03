
use std::cell::RefCell;
use std::rc::Rc;

// Simple node for demonstrating Box<T>
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Box<Node>>,
}

// Shared counter for demonstrating Rc<RefCell<T>>
#[derive(Debug)]
struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }

    // TODO: Implement this method
    pub fn add_child(&mut self, value: i32) {
        let new_node = Node::new(value);
        let boxed_node = Box::new(new_node);
        self.children.push(boxed_node);
    }
}

impl SharedCounter {
    pub fn new() -> Self {
        Self {
            value: Rc::new(RefCell::new(0)),
        }
    }

    // TODO: Implement this method
    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    // TODO: Implement this method
    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_usage() {
        // 1. Create a Node
        let mut root = Node::new(10);

        // 2. Add children using add_child
        root.add_child(20);
        root.add_child(30);

        // 3. Verify children were added
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].value, 20);
        assert_eq!(root.children[1].value, 30);
    }

    #[test]
    fn test_shared_counter() {
        // 1. Create SharedCounter
        let counter1 = SharedCounter::new();
        assert_eq!(counter1.get(), 0);

        // 2. Create multiple references with clone
        let counter2 = SharedCounter {
            value: Rc::clone(&counter1.value),
        };

        // 3. Increment from different references
        counter1.increment();
        counter2.increment();

        // 4. Verify all see the same value
        assert_eq!(counter1.get(), 2);
        assert_eq!(counter2.get(), 2);
    }
}

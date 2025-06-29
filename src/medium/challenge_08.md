# Challenge 8: Smart Pointers Basics

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Box, Rc, RefCell, Memory Management, Reference Counting

## Learning Objectives

By completing this challenge, you will understand:
- Different smart pointer types and their use cases
- Reference counting with Rc<T>
- Interior mutability with RefCell<T>
- Heap allocation with Box<T>
- Avoiding memory leaks and cycles

## Background

Smart pointers provide additional capabilities beyond regular references:
- **Box<T>**: Heap allocation with single ownership
- **Rc<T>**: Reference counting for shared ownership (single-threaded)
- **RefCell<T>**: Interior mutability with runtime borrow checking

### Smart Pointer Usage Guide

| Use Case | Smart Pointer | Why? |
|----------|---------------|------|
| Recursive data structures | `Box<T>` | Fixed size, heap allocation |
| Shared read-only data | `Rc<T>` | Multiple owners, single-threaded |
| Mutable data with shared refs | `RefCell<T>` | Interior mutability |

## Challenge

Create a simplified blockchain node system using basic smart pointers.

### Structures to Implement

#### **Basic Data Types:**
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct NodeId(u32);

#[derive(Debug, Clone)]
struct Block {
    number: u64,
    hash: String,
    transactions: Vec<String>,
}

#[derive(Debug)]
struct Node {
    id: NodeId,
    blocks: Vec<Rc<Block>>,
    peers: Vec<Weak<RefCell<Node>>>,
}
```

#### **Block Tree Structure:**
```rust
#[derive(Debug)]
struct BlockNode {
    block: Rc<Block>,
    children: Vec<Box<BlockNode>>,
}

struct BlockTree {
    root: Option<Box<BlockNode>>,
}
```

#### **Network Structure:**
```rust
struct Network {
    nodes: Vec<Rc<RefCell<Node>>>,
    shared_blocks: Rc<RefCell<HashMap<String, Rc<Block>>>>,
}
```

### Provided Implementations

#### **Basic Constructors:**
```rust
impl Block {
    pub fn new(number: u64, hash: String, transactions: Vec<String>) -> Self {
        Self { number, hash, transactions }
    }
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            blocks: Vec::new(),
            peers: Vec::new(),
        }
    }
    
    pub fn add_block(&mut self, block: Rc<Block>) {
        self.blocks.push(block);
    }
}

impl BlockTree {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            shared_blocks: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
```

### Methods for You to Implement

#### **1. Network Node Management (`add_node`):**
```rust
impl Network {
    // TODO: Implement this method
    pub fn add_node(&mut self, id: NodeId) -> Rc<RefCell<Node>> {
        // IMPLEMENT:
        // 1. Create new Node with the given id
        // 2. Wrap it in Rc<RefCell<Node>>
        // 3. Add to self.nodes vector
        // 4. Return the Rc<RefCell<Node>>
        todo!()
    }
}
```

#### **2. Node Connection (`connect_nodes`):**
```rust
impl Network {
    // TODO: Implement this method
    pub fn connect_nodes(&mut self, node1: &Rc<RefCell<Node>>, node2: &Rc<RefCell<Node>>) {
        // IMPLEMENT:
        // 1. Create weak references to avoid cycles
        // 2. Add node2 as peer to node1 (use Rc::downgrade)
        // 3. Add node1 as peer to node2 (use Rc::downgrade)
        // Remember to borrow_mut() for interior mutability
        todo!()
    }
}
```

#### **3. Block Broadcasting (`broadcast_block`):**
```rust
impl Network {
    // TODO: Implement this method
    pub fn broadcast_block(&self, block: Rc<Block>) {
        // IMPLEMENT:
        // 1. Add block to shared_blocks HashMap (use hash as key)
        // 2. For each node in network:
        //    - Borrow the node mutably
        //    - Add the block to the node's blocks vector
        todo!()
    }
}
```

#### **4. Block Tree Operations (`add_block_to_tree`):**
```rust
impl BlockTree {
    // TODO: Implement this method
    pub fn add_block(&mut self, block: Rc<Block>) {
        // IMPLEMENT:
        // 1. Create new BlockNode with the block
        // 2. If root is None, set it as root
        // 3. Otherwise, add as child to root (simplified - just to first child)
        // Wrap in Box<BlockNode>
        todo!()
    }
}
```

### Tests to Implement

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_nodes() {
        // TODO: Implement this test
        // 1. Create network
        // 2. Add two nodes
        // 3. Verify they were added correctly
        todo!()
    }

    #[test]
    fn test_node_connections() {
        // TODO: Implement this test
        // 1. Create network with two nodes
        // 2. Connect them
        // 3. Verify peer connections exist
        todo!()
    }

    #[test]
    fn test_block_broadcast() {
        // TODO: Implement this test
        // 1. Create network with nodes
        // 2. Create and broadcast a block
        // 3. Verify all nodes received the block
        todo!()
    }

    #[test]
    fn test_block_tree() {
        // TODO: Implement this test
        // 1. Create block tree
        // 2. Add blocks to tree
        // 3. Verify tree structure
        todo!()
    }
}
```

### Smart Pointer Patterns

#### **1. Reference Counting with Rc:**
```rust
let block = Rc::new(Block::new(1, "hash1".to_string(), vec![]));
let block_ref1 = block.clone(); // Increment ref count
let block_ref2 = block.clone(); // Increment ref count
println!("Ref count: {}", Rc::strong_count(&block)); // Should be 3
```

#### **2. Interior Mutability with RefCell:**
```rust
let node = Rc::new(RefCell::new(Node::new(NodeId(1))));
{
    let mut node_ref = node.borrow_mut(); // Runtime borrow check
    node_ref.add_block(Rc::new(Block::new(1, "hash".to_string(), vec![])));
} // Borrow ends here
```

#### **3. Weak References to Break Cycles:**
```rust
let strong_ref = Rc::new(RefCell::new(Node::new(NodeId(1))));
let weak_ref = Rc::downgrade(&strong_ref); // Doesn't increase ref count

// Later, upgrade weak reference
if let Some(strong_again) = weak_ref.upgrade() {
    // Use the strong reference
}
```

### Example Usage

```rust
fn main() {
    let mut network = Network::new();
    
    // Add nodes
    let node1 = network.add_node(NodeId(1));
    let node2 = network.add_node(NodeId(2));
    
    // Connect nodes
    network.connect_nodes(&node1, &node2);
    
    // Create and broadcast block
    let block = Rc::new(Block::new(1, "block1".to_string(), vec!["tx1".to_string()]));
    network.broadcast_block(block.clone());
    
    // Verify block was received
    {
        let node1_ref = node1.borrow();
        println!("Node 1 has {} blocks", node1_ref.blocks.len());
    }
    
    // Test block tree
    let mut tree = BlockTree::new();
    tree.add_block(block);
}
```

### Expected Output

A smart pointer system that:
- Manages shared ownership of blocks with Rc<T>
- Provides interior mutability for nodes with RefCell<T>  
- Uses weak references to prevent memory cycles
- Demonstrates heap allocation patterns with Box<T>
- Shows reference counting and borrowing patterns

### Theoretical Context

**Smart Pointer Fundamentals:**
- **Ownership**: Who owns the data and when it's cleaned up
- **Reference Counting**: Multiple owners sharing data safely
- **Interior Mutability**: Mutating data through shared references
- **Memory Safety**: Preventing leaks and use-after-free

**Key Patterns:**
1. **Rc<T> for Sharing**: Multiple owners of immutable data
2. **RefCell<T> for Mutation**: Runtime-checked mutable borrows  
3. **Weak<T> for Cycles**: Non-owning references to break cycles
4. **Box<T> for Heap**: Single ownership of heap-allocated data

**Substrate Connection:**
- Storage items use reference counting for efficiency
- Runtime modules share data through smart pointers
- Consensus mechanisms use shared state patterns
- Node networking requires cycle-safe references

This challenge teaches essential smart pointer patterns needed for complex Substrate data structures.
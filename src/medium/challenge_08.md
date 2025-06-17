# Challenge 8: Smart Pointers and Memory Management

**Estimated Time:** 40 minutes  
**Difficulty:** Medium  
**Topics:** Box, Rc, Arc, RefCell, Weak, Memory Layout, Reference Counting

## Learning Objectives

By completing this challenge, you will understand:
- Different smart pointer types and their use cases
- Reference counting vs ownership
- Interior mutability patterns
- Avoiding memory leaks and cycles
- Thread-safe vs single-threaded smart pointers

## Background

Smart pointers provide additional capabilities beyond regular references:
- **Box<T>**: Heap allocation with single ownership
- **Rc<T>**: Reference counting for shared ownership
- **Arc<T>**: Atomic reference counting for thread safety
- **RefCell<T>**: Interior mutability with runtime borrow checking
- **Weak<T>**: Non-owning references to break cycles

These are essential for complex data structures and shared state management in Substrate.

## Challenge

Create a blockchain node network simulation using various smart pointers.

### Requirements

1. **Create basic data structures:**
   ```rust
   #[derive(Debug, Clone)]
   struct NodeId(u32);

   #[derive(Debug)]
   struct Block {
       number: u64,
       hash: String,
       parent_hash: String,
       transactions: Vec<String>,
   }

   #[derive(Debug)]
   struct Node {
       id: NodeId,
       blocks: Vec<Rc<Block>>,
       peers: Vec<Weak<RefCell<Node>>>,
       is_validator: bool,
   }
   ```

2. **Create a `BlockTree` using `Box` for recursive structure:**
   ```rust
   #[derive(Debug)]
   struct BlockNode {
       block: Rc<Block>,
       parent: Option<Box<BlockNode>>,
       children: Vec<Box<BlockNode>>,
   }

   struct BlockTree {
       root: Option<Box<BlockNode>>,
       blocks_by_hash: HashMap<String, Weak<Block>>,
   }
   ```

3. **Create a `Network` using `Arc` and `RefCell` for shared mutable state:**
   ```rust
   struct Network {
       nodes: Vec<Arc<RefCell<Node>>>,
       shared_blocks: Arc<RefCell<HashMap<String, Rc<Block>>>>,
   }
   ```

4. **Implement methods:**
   - `Network::new() -> Self`
   - `Network::add_node(&mut self, id: NodeId, is_validator: bool) -> Arc<RefCell<Node>>`
   - `Network::connect_nodes(&mut self, node1: &Arc<RefCell<Node>>, node2: &Arc<RefCell<Node>>)`
   - `Network::broadcast_block(&self, block: Rc<Block>)`
   - `Network::get_node_count(&self) -> usize`
   - `BlockTree::add_block(&mut self, block: Rc<Block>, parent_hash: Option<String>)`
   - `BlockTree::find_block(&self, hash: &str) -> Option<Rc<Block>>`

### Expected Behavior

```rust
let mut network = Network::new();

// Add nodes
let node1 = network.add_node(NodeId(1), true);
let node2 = network.add_node(NodeId(2), false);
let node3 = network.add_node(NodeId(3), true);

// Connect nodes
network.connect_nodes(&node1, &node2);
network.connect_nodes(&node2, &node3);

// Create and broadcast block
let block = Rc::new(Block {
    number: 1,
    hash: "block1".to_string(),
    parent_hash: "genesis".to_string(),
    transactions: vec!["tx1".to_string(), "tx2".to_string()],
});

network.broadcast_block(block.clone());

// Verify block propagation
{
    let node1_ref = node1.borrow();
    assert!(node1_ref.blocks.contains(&block));
}

// Build block tree
let mut tree = BlockTree::new();
tree.add_block(block, Some("genesis".to_string()));
```

## Advanced Requirements

1. **Implement a `SharedCache` with `Arc<Mutex<T>>`:**
   ```rust
   use std::sync::{Arc, Mutex};
   
   struct SharedCache<K, V> {
       data: Arc<Mutex<HashMap<K, V>>>,
       max_size: usize,
   }
   
   impl<K, V> SharedCache<K, V> 
   where 
       K: Eq + Hash + Clone,
       V: Clone,
   {
       fn new(max_size: usize) -> Self;
       fn get(&self, key: &K) -> Option<V>;
       fn insert(&self, key: K, value: V);
       fn len(&self) -> usize;
   }
   ```

2. **Create a `WeakNodeRef` system to avoid cycles:**
   ```rust
   struct NodeManager {
       nodes: Vec<Arc<RefCell<Node>>>,
       node_refs: HashMap<NodeId, Weak<RefCell<Node>>>,
   }
   
   impl NodeManager {
       fn add_node(&mut self, node: Node) -> NodeId;
       fn get_node(&self, id: &NodeId) -> Option<Arc<RefCell<Node>>>;
       fn remove_node(&mut self, id: &NodeId) -> bool;
       fn cleanup_dead_refs(&mut self);
   }
   ```

3. **Implement custom smart pointer with `Drop`:**
   ```rust
   struct TrackedBox<T> {
       data: Box<T>,
       id: u64,
   }
   
   impl<T> TrackedBox<T> {
       fn new(data: T) -> Self;
       fn leak_count() -> usize;
   }
   
   impl<T> Drop for TrackedBox<T> {
       fn drop(&mut self) {
           // Track deallocations
       }
   }
   ```

## Testing

Write tests that demonstrate:
- Shared ownership with `Rc<T>`
- Thread-safe sharing with `Arc<T>`
- Interior mutability with `RefCell<T>`
- Weak references breaking cycles
- Memory cleanup and leak prevention

```rust
#[test]
fn test_shared_block_ownership() {
    let block = Rc::new(Block::new(1, "hash1", "parent"));
    let block_ref1 = block.clone();
    let block_ref2 = block.clone();
    
    assert_eq!(Rc::strong_count(&block), 3);
    drop(block_ref1);
    assert_eq!(Rc::strong_count(&block), 2);
}

#[test]
fn test_weak_references() {
    let node = Rc::new(RefCell::new(Node::new(NodeId(1))));
    let weak_ref = Rc::downgrade(&node);
    
    assert!(weak_ref.upgrade().is_some());
    drop(node);
    assert!(weak_ref.upgrade().is_none());
}

#[test]
fn test_interior_mutability() {
    let node = Rc::new(RefCell::new(Node::new(NodeId(1))));
    
    // Multiple immutable borrows
    {
        let borrow1 = node.borrow();
        let borrow2 = node.borrow();
        assert_eq!(borrow1.id, borrow2.id);
    }
    
    // Single mutable borrow
    {
        let mut borrow = node.borrow_mut();
        borrow.is_validator = true;
    }
}
```

## Smart Pointer Patterns

1. **Shared Ownership:**
   ```rust
   let shared_data = Rc::new(expensive_computation());
   let worker1 = Worker::new(shared_data.clone());
   let worker2 = Worker::new(shared_data.clone());
   ```

2. **Interior Mutability:**
   ```rust
   let shared_state = Rc::new(RefCell::new(State::new()));
   modify_state(&shared_state);
   read_state(&shared_state);
   ```

3. **Breaking Cycles:**
   ```rust
   struct Parent {
       children: Vec<Rc<RefCell<Child>>>,
   }
   
   struct Child {
       parent: Weak<RefCell<Parent>>,
   }
   ```

## Tips

- Use `Rc<T>` for single-threaded shared ownership
- Use `Arc<T>` for multi-threaded shared ownership
- Use `RefCell<T>` for interior mutability with runtime checks
- Use `Weak<T>` to break reference cycles
- Be careful with `RefCell` - runtime panics on borrow violations

## Key Learning Points

- **Ownership Models**: Single vs shared ownership patterns
- **Memory Safety**: How smart pointers prevent common errors
- **Reference Cycles**: Identifying and breaking cycles with weak references
- **Interior Mutability**: Mutating data through shared references
- **Thread Safety**: Choosing between `Rc` and `Arc`

## Substrate Connection

Substrate's smart pointer usage:
- `Arc<T>` for shared runtime components
- `Rc<T>` in single-threaded contexts
- `RefCell<T>` for interior mutability in storage
- Weak references in event system to prevent cycles
- Custom smart pointers for memory pool management

## Bonus Challenges

⚠️ **For Advanced Exploration - Substrate Preparation**

1. **Memory layout optimization** - Understand blockchain storage efficiency
2. **Custom smart pointer patterns** - Practice patterns used in Substrate runtime 
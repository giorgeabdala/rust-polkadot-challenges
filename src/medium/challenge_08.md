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

### 🎯 **Smart Pointer Decision Guide**

#### **When to Use Each Type:**

| Use Case | Smart Pointer | Why? |
|----------|---------------|------|
| Recursive data structures | `Box<T>` | Fixed size, heap allocation |
| Shared read-only data | `Rc<T>` | Multiple owners, single-threaded |
| Shared data across threads | `Arc<T>` | Multiple owners, thread-safe |
| Mutable data with shared refs | `RefCell<T>` | Interior mutability |
| Break reference cycles | `Weak<T>` | Non-owning references |

#### **Performance Characteristics:**

```rust
// Box<T> - Zero runtime cost
let data = Box::new(expensive_data);  // One allocation
let value = *data;                    // Direct access, no overhead

// Rc<T> - Reference counting overhead
let data = Rc::new(expensive_data);   // Allocation + ref count
let clone = data.clone();             // Increment counter (cheap)
drop(clone);                          // Decrement counter

// Arc<T> - Atomic reference counting
let data = Arc::new(expensive_data);  // Allocation + atomic ref count
let clone = data.clone();             // Atomic increment (more expensive)

// RefCell<T> - Runtime borrow checking
let data = RefCell::new(value);
let borrow = data.borrow();           // Runtime check, panic on violation
```

### 🔧 **Memory Layout and Ownership**

#### **Box<T> - Single Ownership:**
```rust
// Stack: pointer → Heap: actual data
let boxed = Box::new([1; 1000]);      // Large array on heap
// When boxed goes out of scope, memory is automatically freed
```

#### **Rc<T> - Shared Ownership:**
```rust
// Heap layout: [ref_count: usize, data: T]
let shared = Rc::new(String::from("shared"));
println!("Ref count: {}", Rc::strong_count(&shared));  // 1

let clone1 = shared.clone();  // ref_count becomes 2
let clone2 = shared.clone();  // ref_count becomes 3
// Data freed only when ref_count reaches 0
```

#### **Interior Mutability Pattern:**
```rust
// Combine Rc + RefCell for shared mutable data
let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

// Multiple owners can mutate through RefCell
let data1 = shared_data.clone();
let data2 = shared_data.clone();

data1.borrow_mut().push(4);  // Mutable borrow at runtime
assert_eq!(*data2.borrow(), vec![1, 2, 3, 4]);
```

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

2. **Create a `WeakNodeRef`
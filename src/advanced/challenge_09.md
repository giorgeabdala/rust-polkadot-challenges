## Challenge 9: Runtime Hooks - Simplified Cleanup System

**Difficulty Level:** Advanced
**Estimated Time:** 1 hour

### Objective Description

In this challenge, you will implement a simplified pallet that demonstrates the use of runtime hooks: `on_initialize`, `on_finalize`, and `on_runtime_upgrade`. The pallet will manage "temporary tasks" with automatic cleanup, demonstrating core Substrate concepts including the Config pattern, generics, and runtime hooks.

### Main Concepts Covered

1. **Runtime Hooks**: Functions executed automatically during block lifecycle
2. **Config Trait**: Substrate's configuration pattern with associated types
3. **Generics**: Working with generic types and PhantomData
4. **Weight Management**: Hooks return computational weight
5. **Automatic Cleanup**: Using on_finalize for maintenance
6. **Event System**: Emitting events from hooks

### Structures to Implement

#### **Config Trait (Essential Substrate Pattern):**
```rust
/// Configuration trait defining pallet dependencies
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug;
    type BlockNumber: Copy + PartialOrd + core::ops::Add<Output = Self::BlockNumber>;
    type TaskLifetime: Get<Self::BlockNumber>; // How long tasks live
}

/// Helper trait for getting constant values
pub trait Get<V> {
    fn get() -> V;
}
```

#### **Task Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Task<AccountId, BlockNumber> {
    pub id: u32,
    pub creator: AccountId,
    pub created_at: BlockNumber,
}
```

#### **Event Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    TaskCreated { task_id: u32, creator: T::AccountId },
    TaskExpired { task_id: u32 },
    RuntimeUpgraded { old_version: u32, new_version: u32 },
}
```

#### **Error Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    MaxTasksReached,
}
```

#### **Origin Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Origin<AccountId> {
    Signed(AccountId),
    Root,
}
```

#### **Pallet Struct:**
```rust
use std::collections::HashMap;

/// Main pallet struct with generic configuration
pub struct Pallet<T: Config> {
    tasks: HashMap<u32, Task<T::AccountId, T::BlockNumber>>,
    next_task_id: u32,
    runtime_version: u32,
    emitted_events: Vec<Event<T>>,
    _phantom: core::marker::PhantomData<T>, // For unused generic parameter
}
```

### Provided Methods

#### **Constructor and Utilities:**
```rust
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_task_id: 1,
            runtime_version: 1,
            emitted_events: Vec::new(),
            _phantom: core::marker::PhantomData,
        }
    }

    fn deposit_event(&mut self, event: Event<T>) {
        self.emitted_events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<Event<T>> {
        std::mem::take(&mut self.emitted_events)
    }

    fn ensure_signed(origin: Origin<T::AccountId>) -> Result<T::AccountId, Error> {
        match origin {
            Origin::Signed(account) => Ok(account),
            _ => Err(Error::BadOrigin),
        }
    }
    
    /// Query methods
    pub fn get_task(&self, task_id: u32) -> Option<&Task<T::AccountId, T::BlockNumber>> {
        self.tasks.get(&task_id)
    }

    pub fn get_active_tasks_count(&self) -> u32 {
        self.tasks.len() as u32
    }
    
    pub fn get_runtime_version(&self) -> u32 {
        self.runtime_version
    }
}
```

### Methods for You to Implement

#### **1. Task Creation (`create_task`):**
```rust
impl<T: Config> Pallet<T> {
    // TODO: Implement this method
    pub fn create_task(
        &mut self,
        origin: Origin<T::AccountId>,
        current_block: T::BlockNumber,
    ) -> Result<(), Error> {
        // IMPLEMENT:
        // 1. Use ensure_signed to extract AccountId from origin
        // 2. Check if tasks.len() < 50 (max tasks limit), return MaxTasksReached if not
        // 3. Create new Task with next_task_id, creator account, and current_block
        // 4. Insert task into tasks HashMap using next_task_id as key
        // 5. Emit TaskCreated event with task_id and creator
        // 6. Increment next_task_id
        // 7. Return Ok(())
        todo!()
    }
}
```

#### **2. Initialize Hook (`on_initialize`):**
```rust
impl<T: Config> Pallet<T> {
    // TODO: Implement this method
    /// Hook executed at the BEGINNING of each block
    pub fn on_initialize(&mut self, _block_number: T::BlockNumber) -> u64 {
        // IMPLEMENT:
        // For this simplified version, just return base weight
        // Return: 10_000
        todo!()
    }
}
```

#### **3. Finalize Hook (`on_finalize`):**
```rust
impl<T: Config> Pallet<T> {
    // TODO: Implement this method
    /// Hook executed at the END of each block for cleanup
    pub fn on_finalize(&mut self, block_number: T::BlockNumber) -> u64 {
        // IMPLEMENT:
        // 1. Get task_lifetime from T::TaskLifetime::get()
        // 2. Find the first expired task where: created_at + task_lifetime <= block_number
        // 3. If found:
        //    - Remove it from tasks HashMap
        //    - Emit TaskExpired event with the task_id
        //    - Return weight: 15_000 (cleanup weight)
        // 4. If no expired task found:
        //    - Return weight: 10_000 (base weight)
        todo!()
    }
}
```

#### **4. Runtime Upgrade Hook (`on_runtime_upgrade`):**
```rust
impl<T: Config> Pallet<T> {
    // TODO: Implement this method
    /// Hook executed during runtime upgrade
    pub fn on_runtime_upgrade(&mut self) -> u64 {
        // IMPLEMENT:
        // 1. Store the old runtime_version
        // 2. Increment runtime_version by 1
        // 3. Emit RuntimeUpgraded event with old_version and new_version
        // 4. Return weight: 50_000
        todo!()
    }
}
```

### Test Configuration

```rust
// Test configuration implementing the Config trait
struct TestConfig;
impl Config for TestConfig {
    type AccountId = u32;
    type BlockNumber = u64;
    type TaskLifetime = TestTaskLifetime;
}

// Helper struct providing task lifetime
struct TestTaskLifetime;
impl Get<u64> for TestTaskLifetime {
    fn get() -> u64 { 5 } // Tasks live for 5 blocks
}
```

### Tests to Implement

Create tests that cover:

#### **Test Scenarios:**

1. **Basic Functionality:**
   - Task creation with valid origin
   - Hook weight returns
   - Event emission

2. **Runtime Hooks:**
   - on_initialize returns correct weight
   - on_finalize cleans up expired tasks
   - on_runtime_upgrade increments version

3. **Error Handling:**
   - BadOrigin when using Root origin for task creation
   - MaxTasksReached when exceeding 50 tasks

4. **Task Lifecycle:**
   - Tasks expire after configured lifetime
   - Cleanup happens automatically

### Example Usage

```rust
fn main() {
    let mut pallet = Pallet::<TestConfig>::new();
    
    // Create a task
    let result = pallet.create_task(
        Origin::Signed(1), // User 1
        1,                 // Block 1
    );
    println!("Task creation: {:?}", result);
    
    // Check events
    let events = pallet.take_events();
    println!("Events: {:?}", events);
    
    // Execute hooks
    let init_weight = pallet.on_initialize(2);
    println!("Initialize weight: {}", init_weight);
    
    // Finalize at block 7 (task should expire: 1 + 5 <= 7)
    let finalize_weight = pallet.on_finalize(7);
    println!("Finalize weight: {}", finalize_weight);
    
    // Check if task was cleaned up
    println!("Active tasks: {}", pallet.get_active_tasks_count());
    
    // Trigger runtime upgrade
    let upgrade_weight = pallet.on_runtime_upgrade();
    println!("Upgrade weight: {}", upgrade_weight);
    println!("Runtime version: {}", pallet.get_runtime_version());
}
```

### Expected Output

A runtime hooks system that:
- Demonstrates essential Substrate pallet patterns
- Shows proper use of Config trait with associated types
- Implements automatic cleanup through runtime hooks
- Handles weight management correctly
- Uses generics and PhantomData appropriately

### Theoretical Context

**Runtime Hooks in Substrate:**
- **`on_initialize`**: Called at the start of each block for setup tasks
- **`on_finalize`**: Called at the end of each block for cleanup
- **`on_runtime_upgrade`**: Called during runtime upgrades for data migration

**Config Trait Pattern:**
- **Associated Types**: Define dependencies without concrete implementations
- **Runtime Flexibility**: Allow different runtimes to configure pallet behavior
- **Type Safety**: Compile-time guarantees about type relationships
- **Modularity**: Clean separation between pallet logic and runtime configuration

**Generic Programming in Substrate:**
- **Reusability**: Same pallet code works with different runtimes
- **Type Safety**: Compiler ensures correct type usage
- **PhantomData**: Handles unused generic parameters in structs
- **Trait Bounds**: Specify required functionality for generic types

**Weight System:**
- **Resource Metering**: Track computational cost of operations
- **Block Limits**: Prevent blocks from becoming too heavy
- **Fee Calculation**: Basis for transaction fee computation
- **Network Performance**: Ensures consistent block times

**Key Patterns:**
1. **Hook Execution Order**: Initialize → Extrinsics → Finalize
2. **Config-driven Behavior**: Runtime configures pallet via Config trait
3. **Automatic Maintenance**: Self-cleaning systems using hooks
4. **Event Transparency**: Track all important state changes

This challenge teaches fundamental Substrate concepts that are essential for building production pallets and understanding the Substrate framework architecture.
 
## Challenge 9: Runtime Hooks - Automatic Cleanup System

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

In this challenge, you will implement a pallet that demonstrates the use of the three main runtime hooks: `on_initialize`, `on_finalize`, and `on_runtime_upgrade`. The pallet will manage a list of "temporary tasks" that are automatically cleaned up after a determined period, simulating a garbage collection system.

### Main Concepts Covered

1. **Runtime Hooks**: Functions executed automatically at specific moments in the block lifecycle
2. **`on_initialize`**: Executed at the beginning of each block for preparation
3. **`on_finalize`**: Executed at the end of each block for cleanup
4. **`on_runtime_upgrade`**: Executed when there's a runtime upgrade
5. **Weight Management**: Weight control in hooks to avoid overloading blocks

### Structures to Implement

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
    type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + 
                     core::ops::Add<Output = Self::BlockNumber> + 
                     core::ops::Sub<Output = Self::BlockNumber> + 
                     core::fmt::Debug;
    type MaxTasksPerBlock: Get<u32>; // Maximum tasks to process per block
    type TaskLifetime: Get<Self::BlockNumber>; // Task lifetime
}

pub trait Get<V> {
    fn get() -> V;
}
```

#### **`Task` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Task<AccountId, BlockNumber> {
    pub id: u32,
    pub creator: AccountId,
    pub created_at: BlockNumber,
    pub data: Vec<u8>,
}
```

#### **`Event<T: Config>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    TaskCreated { 
        task_id: u32, 
        creator: T::AccountId, 
        created_at: T::BlockNumber 
    },
    TaskExpired { 
        task_id: u32, 
        expired_at: T::BlockNumber 
    },
    BlockInitialized { 
        block_number: T::BlockNumber, 
        active_tasks: u32 
    },
    BlockFinalized { 
        block_number: T::BlockNumber, 
        tasks_cleaned: u32 
    },
    RuntimeUpgraded { 
        old_version: u32, 
        new_version: u32 
    },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    TaskDataTooLarge,
    MaxTasksReached,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
use std::collections::HashMap;

pub struct Pallet<T: Config> {
    tasks: HashMap<u32, Task<T::AccountId, T::BlockNumber>>,
    next_task_id: u32,
    runtime_version: u32,
    emitted_events: Vec<Event<T>>,
    _phantom: core::marker::PhantomData<T>,
}
```

### Required Methods of `Pallet<T: Config>`

#### **Constructor and Utilities:**
```rust
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
        Origin::Signed(who) => Ok(who),
        _ => Err(Error::BadOrigin),
    }
}
```

#### **Extrinsic to Create Tasks:**
```rust
pub fn create_task(
    &mut self,
    origin: Origin<T::AccountId>,
    data: Vec<u8>,
    current_block: T::BlockNumber,
) -> Result<(), Error> {
    let who = Self::ensure_signed(origin)?;
    
    // Check data size
    if data.len() > 1024 {
        return Err(Error::TaskDataTooLarge);
    }
    
    // Check task limit
    if self.tasks.len() >= 1000 {
        return Err(Error::MaxTasksReached);
    }
    
    let task_id = self.next_task_id;
    let task = Task {
        id: task_id,
        creator: who.clone(),
        created_at: current_block,
        data,
    };
    
    self.tasks.insert(task_id, task);
    self.next_task_id += 1;
    
    self.deposit_event(Event::TaskCreated {
        task_id,
        creator: who,
        created_at: current_block,
    });
    
    Ok(())
}
```

#### **Runtime Hooks:**
```rust
// Hook executed at the BEGINNING of each block
pub fn on_initialize(&mut self, block_number: T::BlockNumber) -> u64 {
    let active_tasks = self.tasks.len() as u32;
    
    self.deposit_event(Event::BlockInitialized {
        block_number,
        active_tasks,
    });
    
    // Base weight for initialization
    10_000
}

// Hook executed at the END of each block
pub fn on_finalize(&mut self, block_number: T::BlockNumber) -> u64 {
    let max_tasks_to_clean = T::MaxTasksPerBlock::get();
    let task_lifetime = T::TaskLifetime::get();
    let mut tasks_cleaned = 0u32;
    let mut tasks_to_remove = Vec::new();
    
    // Find expired tasks
    for (task_id, task) in &self.tasks {
        if tasks_cleaned >= max_tasks_to_clean {
            break;
        }
        
        let task_age = block_number - task.created_at;
        if task_age >= task_lifetime {
            tasks_to_remove.push(*task_id);
            tasks_cleaned += 1;
            
            self.deposit_event(Event::TaskExpired {
                task_id: *task_id,
                expired_at: block_number,
            });
        }
    }
    
    // Remove expired tasks
    for task_id in tasks_to_remove {
        self.tasks.remove(&task_id);
    }
    
    self.deposit_event(Event::BlockFinalized {
        block_number,
        tasks_cleaned,
    });
    
    // Weight calculation: base + (tasks_cleaned * cleanup_weight)
    10_000 + (tasks_cleaned as u64 * 5_000)
}

// Hook executed during runtime upgrade
pub fn on_runtime_upgrade(&mut self) -> u64 {
    let old_version = self.runtime_version;
    let new_version = old_version + 1;
    
    // Perform upgrade logic
    self.runtime_version = new_version;
    
    // Example: Clear all tasks during upgrade (optional)
    let tasks_cleared = self.tasks.len() as u32;
    self.tasks.clear();
    
    self.deposit_event(Event::RuntimeUpgraded {
        old_version,
        new_version,
    });
    
    // Weight for upgrade operations
    50_000 + (tasks_cleared as u64 * 1_000)
}
```

#### **Query Functions:**
```rust
pub fn get_task(&self, task_id: u32) -> Option<&Task<T::AccountId, T::BlockNumber>> {
    self.tasks.get(&task_id)
}

pub fn get_tasks_by_creator(&self, creator: &T::AccountId) -> Vec<&Task<T::AccountId, T::BlockNumber>> {
    self.tasks.values()
        .filter(|task| &task.creator == creator)
        .collect()
}

pub fn get_active_tasks_count(&self) -> u32 {
    self.tasks.len() as u32
}

pub fn get_runtime_version(&self) -> u32 {
    self.runtime_version
}

pub fn get_expired_tasks(&self, current_block: T::BlockNumber) -> Vec<u32> {
    let task_lifetime = T::TaskLifetime::get();
    self.tasks.iter()
        .filter_map(|(task_id, task)| {
            let task_age = current_block - task.created_at;
            if task_age >= task_lifetime {
                Some(*task_id)
            } else {
                None
            }
        })
        .collect()
}
```

### Origin Enum

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Origin<AccountId> {
    Signed(AccountId),
    Root,
}
```

### Tests

Create comprehensive tests covering:

#### **Test Configuration:**
```rust
struct TestConfig;
impl Config for TestConfig {
    type AccountId = u32;
    type BlockNumber = u64;
    type MaxTasksPerBlock = TestMaxTasksPerBlock;
    type TaskLifetime = TestTaskLifetime;
}

struct TestMaxTasksPerBlock;
impl Get<u32> for TestMaxTasksPerBlock {
    fn get() -> u32 { 5 }
}

struct TestTaskLifetime;
impl Get<u64> for TestTaskLifetime {
    fn get() -> u64 { 10 }
}
```

#### **Test Scenarios:**

1. **Task Creation:**
   - Test successful task creation
   - Test task data size limits
   - Test maximum tasks limit
   - Test event emission

2. **on_initialize Hook:**
   - Test hook execution at block start
   - Test event emission with active task count
   - Test weight calculation

3. **on_finalize Hook:**
   - Test automatic cleanup of expired tasks
   - Test respecting max tasks per block limit
   - Test weight calculation based on cleaned tasks
   - Test event emission for expired tasks

4. **on_runtime_upgrade Hook:**
   - Test runtime version increment
   - Test upgrade event emission
   - Test weight calculation

5. **Query Functions:**
   - Test task retrieval by ID
   - Test tasks by creator filtering
   - Test active tasks count
   - Test expired tasks identification

6. **Integration Tests:**
   - Test complete block lifecycle (initialize → finalize)
   - Test multiple blocks with task expiration
   - Test runtime upgrade scenarios

### Expected Output

A complete runtime hooks system that:
- Demonstrates proper use of all three runtime hooks
- Implements automatic cleanup with weight management
- Handles task lifecycle management
- Shows understanding of block lifecycle
- Provides proper weight calculations for each hook
- Handles runtime upgrades correctly

### Theoretical Context

**Runtime Hooks in Substrate:**
- **`on_initialize`**: Called at the beginning of block execution, before any extrinsics
- **`on_finalize`**: Called at the end of block execution, after all extrinsics
- **`on_runtime_upgrade`**: Called when the runtime version changes

**Weight Management:**
- Each hook must return its computational weight
- Weights prevent blocks from becoming too heavy
- Critical for network performance and security

**Use Cases:**
- **Cleanup**: Removing expired data automatically
- **Maintenance**: Periodic system maintenance tasks
- **Upgrades**: Migrating data during runtime upgrades
- **Monitoring**: Tracking system state changes

This challenge demonstrates essential patterns for building self-maintaining blockchain systems that can automatically manage their state over time.
 
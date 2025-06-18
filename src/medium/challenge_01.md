# Challenge 1: Advanced Collections

**Estimated Time:** 30 minutes  
**Difficulty:** Medium  
**Topics:** Vec, HashMap, BTreeMap, HashSet, Advanced Iterators

## Learning Objectives

By completing this challenge, you will understand:
- Advanced collection types and their use cases
- When to use HashMap vs BTreeMap
- Set operations and deduplication
- Complex iterator patterns
- Collection performance characteristics

## Background

Rust provides a rich variety of collections, each optimized for specific use cases. Understanding the performance differences and behavior is fundamental for writing efficient code, especially in blockchain where every operation matters.

### 📊 Collections Overview

| Collection | Ordering | Lookup | Insertion | Primary Use |
|------------|----------|--------|-----------|-------------|
| `Vec<T>` | Index | O(1) by index | O(1) at end | Sequential list |
| `HashMap<K,V>` | ❌ No | O(1) average | O(1) average | Fast lookup |
| `BTreeMap<K,V>` | ✅ Yes | O(log n) | O(log n) | Ordered lookup |
| `HashSet<T>` | ❌ No | O(1) average | O(1) average | Fast membership |
| `BTreeSet<T>` | ✅ Yes | O(log n) | O(log n) | Ordered set |

### 🔍 Detailed Analysis

#### **HashMap<K,V> vs BTreeMap<K,V>**

**HashMap** - The "Flash" of collections:
```rust
// Typical use: cache, indexes, counters
let mut users: HashMap<u32, String> = HashMap::new();
users.insert(42, "Alice".to_string()); // O(1) - very fast!
let user = users.get(&42);              // O(1) - instant!
```

**Advantages:**
- ⚡ O(1) access and insertion - extremely fast
- 🚀 Ideal for frequent lookups
- 💾 Lower memory overhead per element

**Disadvantages:**
- 🎲 No guaranteed order (depends on hash)
- 🔄 Random iteration order
- ⚠️ Worst case O(n) with many collisions

---

**BTreeMap** - The "Organizer" of collections:
```rust
// Typical use: sorted data, ranges, persistence
let mut scores: BTreeMap<String, u32> = BTreeMap::new();
scores.insert("Alice".to_string(), 100); // O(log n) - consistent
scores.insert("Bob".to_string(), 85);
// Always iterates in alphabetical order!
for (name, score) in &scores {
    println!("{}: {}", name, score); // Alice: 100, Bob: 85
}
```

**Advantages:**
- 📋 Always sorted by key
- 🎯 Consistent O(log n) performance
- 🔍 Range query support
- 💾 Better for deterministic persistence

**Disadvantages:**
- 🐌 Slower than HashMap for simple lookups
- 🏗️ Higher memory overhead (tree structure)

#### **HashSet<T> vs BTreeSet<T>**

**HashSet** - For ultra-fast membership testing:
```rust
let mut active_users: HashSet<u32> = HashSet::new();
active_users.insert(42);
if active_users.contains(&42) { // O(1) - instant!
    println!("User is active!");
}
```

**BTreeSet** - For ordered sets:
```rust
let mut leaderboard: BTreeSet<(u32, String)> = BTreeSet::new();
leaderboard.insert((100, "Alice".to_string()));
leaderboard.insert((85, "Bob".to_string()));
// Always iterated in ascending score order!
```

### 🎯 Decision Guide: When to Use Each Collection?

#### Use **HashMap** when:
- ✅ Need very fast lookups
- ✅ Order doesn't matter
- ✅ Cache, indexes, counters
- ✅ Mapping IDs to objects

#### Use **BTreeMap** when:
- ✅ Need sorted data
- ✅ Range queries (e.g., "all between X and Y")
- ✅ Deterministic iteration is important
- ✅ Substrate storage (blockchain determinism)

#### Use **HashSet** when:
- ✅ Ultra-fast membership verification
- ✅ Data deduplication
- ✅ Active sessions, permissions, flags

#### Use **BTreeSet** when:
- ✅ Ordered unique set
- ✅ Leaderboards, rankings
- ✅ Ordered mathematical set operations

### 🔗 Substrate/Polkadot Connection

In Substrate development, collection choice directly impacts:

```rust
// Substrate Storage - always deterministic!
#[pallet::storage]
pub type Accounts<T> = StorageMap<_, Blake2_128Concat, AccountId, AccountInfo>;
// ↑ Works like BTreeMap - deterministic ordering

// For ordered validator sets
pub type Validators<T> = StorageValue<_, BTreeSet<AccountId>>;

// For fast counters (no ordering needed)
pub type Nonces<T> = StorageMap<_, Blake2_128Concat, AccountId, u32>;
```

**Why ordering matters in blockchain?**
- 🔄 **Determinism**: All nodes must process in the same order
- 🔍 **Reproducibility**: Debug and audit need consistency  
- ⚖️ **Consensus**: Different order = different hash = fork!

### 💡 Performance Tips

1. **For small data** (<100 elements): BTreeMap might be faster than HashMap due to cache locality
2. **For large data** (>1000 elements): HashMap usually wins
3. **Memory layout**: Vec > BTreeMap > HashMap (density)
4. **When in doubt**: Benchmark with real data!

These collections are the fundamental building blocks for efficient systems in the Polkadot ecosystem!

## Challenge

Create a user management system that demonstrates advanced collection usage.

### Requirements

1. **Create a `User` struct** with:
   - `id: u32`
   - `username: String`
   - `email: String`
   - `roles: Vec<String>`

2. **Create a `UserManager` struct** with:
   - `users: HashMap<u32, User>`
   - `username_index: BTreeMap<String, u32>`
   - `active_sessions: HashSet<u32>`

3. **Implement methods:**
   - `UserManager::new() -> Self`
   - `add_user(&mut self, user: User) -> Result<(), String>`
   - `get_user(&self, id: u32) -> Option<&User>`
   - `find_by_username(&self, username: &str) -> Option<&User>`
   - `get_users_by_role(&self, role: &str) -> Vec<&User>`
   - `start_session(&mut self, user_id: u32) -> bool`
   - `end_session(&mut self, user_id: u32) -> bool`
   - `get_active_users(&self) -> Vec<&User>`
   - `get_sorted_usernames(&self) -> Vec<&String>`

### Expected Behavior

```rust
let mut manager = UserManager::new();

let user1 = User {
    id: 1,
    username: "alice".to_string(),
    email: "alice@example.com".to_string(),
    roles: vec!["admin".to_string(), "user".to_string()],
};

manager.add_user(user1)?;
manager.start_session(1);

// Fast ID lookup
let user = manager.get_user(1).unwrap();

// Fast username lookup (sorted)
let user = manager.find_by_username("alice").unwrap();

// Role-based filtering
let admins = manager.get_users_by_role("admin");

// Set operations
let active_users = manager.get_active_users();

// Sorted iteration
let sorted_names = manager.get_sorted_usernames();
```

## Testing

Write tests that demonstrate:
- Adding users and preventing duplicates
- Fast lookups by ID and username
- Role-based filtering using iterators
- Session management with sets
- Sorted username retrieval

## Tips

- Use `HashMap` for fast ID-based lookups
- Use `BTreeMap` for sorted username index
- Use `HashSet` for fast session membership testing
- Use iterator methods like `filter`, `map`, `collect`
- Handle duplicate usernames appropriately

## Key Learning Points

- **Collection Choice**: Right data structure for the use case
- **Multiple Indexes**: Maintaining consistency across collections
- **Iterator Patterns**: Functional programming with collections
- **Performance**: Understanding O(1) vs O(log n) vs O(n) operations

## Substrate Connection

This pattern mirrors Substrate's storage design:
- `StorageMap<AccountId, AccountInfo>` (HashMap-like)
- `StorageDoubleMap` for multiple indexes
- Set operations for validator sets
- Sorted iteration for deterministic execution

## Bonus Challenges

⚠️ **For Advanced Exploration**

1. **Performance optimization** - Compare HashMap vs BTreeMap performance characteristics
2. **Advanced iterator patterns** - Chain complex operations with iterators 
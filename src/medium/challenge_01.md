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

Rust provides several collection types beyond basic Vec. Each has specific use cases:
- **HashMap<K,V>**: Fast key-value lookup, unordered
- **BTreeMap<K,V>**: Ordered key-value pairs, sorted keys
- **HashSet<T>**: Fast membership testing, unordered
- **BTreeSet<T>**: Ordered unique elements

These are fundamental for Substrate storage patterns and runtime data management.

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

1. Add a `remove_user()` method that maintains all indexes
2. Implement user groups with nested collections
3. Add statistics methods using iterator combinators
4. Create a generic `IndexedCollection<K, V>` trait 
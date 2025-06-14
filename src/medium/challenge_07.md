# Challenge 7: Task Manager with Simulated Storage and Advanced Filtering (English Version in Code)

**Difficulty Level:** Intermediate Plus

## Objective Description:
You will implement a task management system. Tasks will have an ID, description, and status. The system should allow:

*   Adding new tasks (the ID must be generated automatically).
*   Getting a specific task by its ID. The reference to the obtained task must have a lifetime linked to the task manager.
*   Updating the status of an existing task.
*   Listing tasks using a closure as a predicate for filtering. The references to the listed tasks must also have managed lifetimes.
*   Removing a task.

To simulate storage (like Substrate's `StorageMap` or `StorageValue`), you will define a `KeyValueStorage<K, V>` trait and an in-memory implementation `InMemoryStorage<K, V>`. The `TaskManager` will be generic over this storage trait.

## Rust Concepts to be Applied (Substrate focus):

*   **Structs:** `Task`, `TaskId`, `TaskManager`.
*   **Enum:** `TaskStatus` (e.g., `Pending`, `InProgress`, `Completed`).
*   **Traits:** `KeyValueStorage<K, V>` to abstract the storage mechanism.
*   **Generics:** `TaskManager<S: KeyValueStorage<TaskId, Task>>` to allow different storage implementations.
*   **Lifetimes (`'a`)**: Essential when returning references to "stored" data, ensuring they do not outlive the `TaskManager` or the `Storage` they originate from.
*   **Storage (Simulated):** The `InMemoryStorage` will use a `HashMap` internally. Methods like `get`, `get_mut`, `set`, `remove` will simulate storage interactions.
*   **Matching:** To handle `Option<T>` (when fetching tasks) and `Result<T, E>` (in operations that can fail).
*   **Vectors (`Vec<T>`):** For lists of tasks and IDs.
*   **Closures:**
    *   To provide custom filtering logic (`list_tasks_by_filter_closure`).
*   **Error Handling:** Use `Result<T, String>` to return descriptive errors.

## Focused Data Structures and Concepts (As Requested):

*   **Lifetimes:** Mainly when obtaining task references from "storage".
*   **Storage (simulated):** Through the `KeyValueStorage` trait and its `InMemoryStorage` implementation.
*   **Traits/Generics:** In the definition of `KeyValueStorage` and `TaskManager`.
*   **Structs/Enum:** For `Task`, `TaskId`, `TaskStatus`.
*   **Matching:** On `Option` and `Result`.
*   **Vectors:** For collections.
*   **Closures:** For filtering.

## Initial Definitions (skeleton for you to start with):

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow; // Might be useful

// --- Task ID ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct TaskId(u32);

impl TaskId {
fn new(id: u32) -> Self {
TaskId(id)
}
}

// --- Task Status ---
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TaskStatus {
Pending,
InProgress,
Completed,
}

// --- Task ---
#[derive(Debug, Clone, PartialEq)] // Eq removed as String is not trivially Eq without manual impl
pub struct Task {
pub id: TaskId,
pub description: String,
pub status: TaskStatus,
}

impl Task {
fn new(id: TaskId, description: String) -> Self {
Task {
id,
description,
status: TaskStatus::Pending,
}
}
}

// --- Key-Value Storage Trait (Simulating StorageMap/StorageValue) ---
pub trait KeyValueStorage<K: Eq + Hash, V> {
fn get(&self, key: &K) -> Option<&V>;
fn get_mut(&mut self, key: &K) -> Option<&mut V>;
fn set(&mut self, key: K, value: V) -> Result<(), String>; // Error as String
fn remove(&mut self, key: &K) -> Option<V>; // Key as reference for removal
fn get_all_values(&self) -> Vec<&V>; // To list all tasks
// fn get_all_entries(&self) -> Vec<(&K, &V)>; // Optional, if you need keys and values
}

// --- In-Memory Storage Implementation ---
pub struct InMemoryStorage<K: Eq + Hash + Clone, V> { // K needs to be Clone for some operations
data: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V> InMemoryStorage<K, V> {
pub fn new() -> Self {
InMemoryStorage {
data: HashMap::new(),
}
}
}

impl<K: Eq + Hash + Clone, V> Default for InMemoryStorage<K, V> {
fn default() -> Self {
Self::new()
}
}

// TODO: Implement KeyValueStorage for InMemoryStorage

// --- Task Manager ---
pub struct TaskManager<S: KeyValueStorage<TaskId, Task>> {
storage: S,
next_id: u32,
}

// TODO: Implement methods for TaskManager:
// new()
// add_task(&mut self, description: String) -> TaskId
// get_task<'a>(&'a self, id: &TaskId) -> Option<&'a Task> // Note the lifetime 'a
// update_task_status(&mut self, id: &TaskId, new_status: TaskStatus) -> Result<(), String>
// list_tasks_by_filter_closure<'storage_lifetime, P>(&'storage_lifetime self, predicate: P) -> Vec<&'storage_lifetime Task>
//   where P: Fn(&&Task) -> bool, S: 'storage_lifetime // or S: KeyValueStorage<TaskId, Task> + 'storage_lifetime
// remove_task(&mut self, id: &TaskId) -> Result<Task, String>


// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task_test() {}

    #[test]
    fn get_task_test() {}

    #[test]
    fn update_status_test() {}

    #[test]
    fn update_status_nonexistent_task_fail() {}

    #[test]
    fn test_list_tasks_with_closure_filter() {}

    #[test]
    fn remove_task_test() {}

    #[test]
    fn remove_nonexistent_task_test() {}

    #[test]
    fn test_lifetimes_on_get_task() {}
}
```

## Theoretical Context:

*   **Lifetimes (`'a`):** In Rust, lifetimes ensure that references never point to invalid memory. When a function returns a reference to data it "owns" or manages (like `TaskManager` managing `Tasks` in its storage), the lifetime of this reference must be explicitly linked to the lifetime of the structure that holds the data. For example, in `get_task<'a>(&'a self, id: &TaskId) -> Option<&'a Task>`. This tells the compiler that the referenced `Task` cannot outlive the `TaskManager` (`self`).

*   **Simulated Storage:** In Substrate, `#[pallet::storage]` defines storage items that are persisted on the blockchain. `StorageValue<T>` stores a single value, while `StorageMap<K, V>` stores a map. They use SCALE encoding and interact with a database backend (RocksDB/ParityDB). Our `KeyValueStorage` and `InMemoryStorage` are a very simplified simulation to practice Rust concepts. `get_mut` is conceptually similar to `StorageMap::mutate` or `StorageValue::mutate`, which allow modifying the value in place.

*   **Traits and Generics:** Allow writing abstract and reusable code. `KeyValueStorage` defines a contract, and `TaskManager` can use any `S` that satisfies this contract. This is fundamental in Substrate's pallet architecture (e.g., `Config` trait).

*   **Closures:** Are anonymous functions that can capture variables from the environment where they are defined. They are incredibly powerful for passing behavior as an argument.
    *   `Fn`: Captures by immutable reference.
    *   `FnMut`: Captures by mutable reference.
    *   `FnOnce`: Captures by value (consumes the captured variables).
        In this challenge, you will use `Fn` for filter predicates, as they only read task data to decide if it should be included in the result, without modifying the task or the closure's environment mutably.

## Expected Output:
A functional implementation of the `TODOs`, with passing tests (you will need to write the tests too!). The code should compile without lifetime warnings and demonstrate the correct use of the requested concepts.

# Challenge 4: Generic Entity Registry with Capacity
**Difficulty Level: Intermediate+**

## Objective Description:
You will implement a generic structure called `Registry<T>` that can store and manage different types of entities. Each entity must have a unique identifier. The registry will have a maximum capacity, and add/remove operations must handle this capacity and the possibility of entities not being found, using `Result` for error handling.

## Essential Requirements:

### Types and Traits:

1.  Define a type alias `EntityId` for `u32`.
2.  Create a trait `Identifiable` with a single method `id(&self) -> EntityId`. Any type `T` we want to store in the `Registry` must implement this trait.
3.  Create an enum `RegistryError` with at least three variants:
    *   `CapacityFull`: When trying to add an entity to a full registry.
    *   `NotFound`: When trying to remove or find an entity that does not exist.
    *   `AlreadyExists(EntityId)`: When trying to add an entity with an ID that already exists.
4.  Define a generic struct `Registry<T>`.
    *   The generic parameter `T` must have the following trait bounds: `Identifiable + Clone + std::fmt::Debug + PartialEq`.
    *   The `Registry` must have the following fields:
        *   `entities: Vec<T>`: A vector to store the entities.
        *   `capacity: usize`: The maximum capacity of the registry.

### Functionalities (methods for `impl<T: Identifiable + Clone + std::fmt::Debug + PartialEq> Registry<T>`):

1.  `new(capacity: usize) -> Self`:
    *   Creates a new instance of `Registry` with the provided capacity and an empty vector of entities.
2.  `add_entity(&mut self, entity: T) -> Result<(), RegistryError>`:
    *   Checks if an entity with the same ID already exists. If so, returns `Err(RegistryError::AlreadyExists(entity.id()))`.
    *   If the registry is full (number of entities equals capacity), returns `Err(RegistryError::CapacityFull)`.
    *   Otherwise, adds the entity to the `entities` vector and returns `Ok(())`.
3.  `remove_entity(&mut self, id: EntityId) -> Result<T, RegistryError>`:
    *   Searches for the entity by `id`.
    *   If found, removes it from the `entities` vector and returns `Ok(removed_entity)`.
    *   If not found, returns `Err(RegistryError::NotFound)`.
4.  `get_entity(&self, id: EntityId) -> Option<&T>`:
    *   Searches for the entity by `id`.
    *   If found, returns `Some(&found_entity)`.
    *   If not found, returns `None`.
5.  `list_all_entities(&self) -> Vec<&T>`:
    *   Returns a vector of references to all entities in the registry.
6.  `current_count(&self) -> usize`:
    *   Returns the current number of entities in the registry.
7.  `is_full(&self) -> bool`:
    *   Returns `true` if the registry is at maximum capacity, `false` otherwise.

### Example Entity (for testing):

```rust
#[derive(Clone, Debug, PartialEq)]
struct User {
    user_id: EntityId,
    username: String,
    // Array as an example of a field with a fixed size
    permissions: [bool; 3], // Ex: [can_read, can_write, can_execute]
}

impl Identifiable for User {
    fn id(&self) -> EntityId {
        self.user_id
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Product {
    product_id: EntityId,
    name: String,
    price: u64,
}

impl Identifiable for Product {
    fn id(&self) -> EntityId {
        self.product_id
    }
}
```

## Tests:
You should include unit tests to verify the behavior of `Registry<T>` with different types of entities (e.g., `User` and `Product`).
Consider the following scenarios:

*   Creating a new registry.
*   Adding entities until capacity is full.
*   Trying to add to a full registry.
*   Trying to add an entity with an already existing ID.
*   Removing an existing entity.
*   Trying to remove a non-existent entity.
*   Getting an existing entity.
*   Trying to get a non-existent entity.
*   Listing all entities.
*   Verifying `current_count` and `is_full`.

## Expected Output:
A robust and generic implementation of the `Registry<T>` struct and its methods, passing all unit tests.

## Theoretical Context:

*   **Structs (`struct`)**:
    *   Used to create custom data types by grouping related fields. The `Registry` and example entities (`User`, `Product`) are structs.
    *   Reference: [Defining and Instantiating Structs - Rust Book](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

*   **Vectors (`Vec<T>`)**:
    *   Dynamically sized collections, stored on the heap. Ideal when the number of elements is not known at compile time or may change. `entities: Vec<T>` in our `Registry`.
    *   In Substrate, `Vec<T>` is commonly used in storage (e.g., `StorageValue<Vec<MyStruct>>`) or as part of events and calls.
    *   Reference: [Storing Lists of Values with Vectors - Rust Book](https://doc.rust-lang.org/book/ch08-01-vectors.html)

*   **Generics (`<T>`, trait bounds)**:
    *   Allow writing code that operates on abstract types, making it reusable. `Registry<T>` is generic over the entity type `T`.
    *   Trait bounds (e.g., `T: Identifiable + Clone`) specify what functionalities a generic type `T` must offer. In Substrate, generics are extensively used. For example, a pallet's `Config` trait is often generic, and types like `frame_system::Config::AccountId` are generic parameters.
    *   Reference: [Generic Data Types - Rust Book](https://doc.rust-lang.org/book/ch10-01-syntax.html)

*   **Traits (like `Identifiable`)**:
    *   Define a set of methods that a type can implement, allowing abstraction over behavior. Similar to interfaces in other languages.
    *   The `Config` trait in FRAME pallets is a central example of using traits to define the interface a pallet exposes to the runtime (chunk 41: "Every Polkadot SDK pallet defines a Rust trait called Config").
    *   Reference: [Traits: Defining Shared Behavior - Rust Book](https://doc.rust-lang.org/book/ch10-02-traits.html)

*   **`Result<T, E>` Enum**:
    *   Used for error handling, representing a success (`Ok(T)`) or a failure (`Err(E)`).
    *   Promotes explicit and robust error handling, fundamental in the development of smart contracts and runtimes, where unhandled errors can have severe consequences (chunk 81: "For defining errors, ink! uses idiomatic Rust error handling with the Result<T,E> type... If an error is returned, the contract reverts").
    *   Reference: [Recoverable Errors with Result - Rust Book](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

*   **Arrays (`[T; N]`) (Contextual)**:
    *   In the `User` example, the field `permissions: [bool; 3]` demonstrates how an array can be used within a struct for a fixed set of data. In Substrate runtimes, arrays can appear in storage definitions or event types where a fixed number of elements is expected.
```
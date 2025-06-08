## Challenge 2: Basic Validator Status Manager

**Difficulty Level:** Intermediate

### Objective:
You will implement a simple Rust module to manage the status (Active or Inactive) of a **fixed** set of validators. This exercise will focus on how to use arrays for collections with a size known at compile time, tuples for grouping related data of different types, and `Option` to represent the presence or absence of a status.

### Essential Requirements:

1.  **Data Types:**
    *   Define a type alias `ValidatorId` for `u32`.
    *   Create an enum `ValidatorStatus` with two variants: `Active` and `Inactive`.
    *   Define a constant `MAX_VALIDATORS: usize` with the value `5`. This will be the size of our validator array.
    *   The main state will be an **array** of tuples. Each array element will represent a validator slot and contain a tuple: `(ValidatorId, Option<ValidatorStatus>)`. Initially, all validators will have an ID, but their status will be `None`.

2.  **Functionalities:**
    *   `initialize_validators(ids: &[ValidatorId]) -> [(ValidatorId, Option<ValidatorStatus>); MAX_VALIDATORS]`:
        *   This function receives a slice of `ValidatorId`.
        *   It must initialize and return the array of validators.
        *   The first `ids.len()` elements (limited by `MAX_VALIDATORS`) of the array should be populated with the provided IDs and `Option<ValidatorStatus>` as `None`.
        *   If `ids.len()` is less than `MAX_VALIDATORS`, the remaining array slots should be filled with a default ID (e.g., `0`) and status `None`.
        *   If `ids.len()` is greater than `MAX_VALIDATORS`, only the first `MAX_VALIDATORS` IDs from the slice should be used.
    *   `set_validator_status(validators: &mut [(ValidatorId, Option<ValidatorStatus>); MAX_VALIDATORS], id: ValidatorId, status: ValidatorStatus) -> bool`:
        *   This function attempts to set the status of a specific validator in the array.
        *   It receives a **mutable** reference to the array of validators, the `ValidatorId`, and the `ValidatorStatus`.
        *   If the `ValidatorId` is found in the array, its status should be updated to `Some(status)`, and the function should return `true`.
        *   If the `ValidatorId` is not found, the array should remain unchanged and the function should return `false`.
    *   `get_validator_status(validators: &[(ValidatorId, Option<ValidatorStatus>); MAX_VALIDATORS], id: ValidatorId) -> Option<(ValidatorId, Option<ValidatorStatus>)>`:
        *   This function searches for and returns a validator’s information.
        *   It receives an **immutable** reference to the array of validators and the `ValidatorId`.
        *   If the `ValidatorId` is found, the function should return `Some` containing the tuple `(ValidatorId, Option<ValidatorStatus>)`.
        *   If the `ValidatorId` is not found, the function should return `None`.

### Tests:
You must include unit tests to verify the behavior of your functions. Consider the following scenarios:
*   Correct initialization of validators (IDs and `None` status).
*   Initialization with fewer IDs than `MAX_VALIDATORS`.
*   Initialization with more IDs than `MAX_VALIDATORS`.
*   Setting the status of an existing validator.
*   Attempting to set the status of a non-existent validator.
*   Getting the status of a validator with a defined status.
*   Getting the status of an existing validator with `None` status.
*   Attempting to get the status of a non-existent validator.

### Expected Output:
A set of Rust functions that pass all proposed unit tests, demonstrating correct manipulation of arrays, tuples, and `Option`.

### Theoretical Context:

#### Arrays (`[T; N]`):
*   Are fixed-size collections of elements of the same type `T`, with size `N` known at compile time.
*   Stored on the stack if the element type and size allow, making them more efficient than heap allocations for small, fixed-size collections.
*   In the `no_std` environment of Substrate runtimes—where heap allocators can be limited or customized—arrays are a natural choice for fixed-size data.
*   Reference: [The Array Type - Rust Book](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

#### Tuples (`(T1, T2, ..., Tn)`):
*   Allow grouping a fixed number of values of potentially different types into a single composite type.
*   Elements are accessed by index (e.g., `my_tuple.0`, `my_tuple.1`).
*   Useful for returning multiple values from a function or for simple and ad-hoc data structures where naming the fields (like in a `struct`) is not strictly necessary.
*   Reference: [The Tuple Type - Rust Book](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

#### `Option<T>` Enum:
*   A fundamental type in Rust used to represent a value that may or may not exist.
*   Has two variants: `Some(T)` (contains a value of type `T`) and `None` (contains no value).
*   Helps prevent common errors in other languages related to null or undefined values, as the Rust compiler ensures you explicitly handle the `None` case.
*   In Substrate, `Option<V>` is commonly used in `StorageValue` or `StorageMap` to represent whether a value exists for a key. For example, a storage item may be `Optional`, indicating the query could return `None` if the entry does not exist (as seen in metadata, chunk 229: `modifier: "Optional"`).
*   Reference: [`std::option::Option` - Rust Docs](https://doc.rust-lang.org/std/option/enum.Option.html)

#### Borrowing (`&T`, `&mut T`):
*   Rust uses the concept of "borrowing" to allow parts of code to access data without taking ownership of it.
*   `&T` creates an immutable reference (you can read but not modify).
*   `&mut T` creates a mutable reference (you can read and modify).
*   This is crucial for writing efficient and safe code, avoiding unnecessary data copies and preventing data races. In our challenge, `set_validator_status` will need a mutable reference to the array, while `get_validator_status` will use an immutable reference.
*   Reference: [References & Borrowing - Rust Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---
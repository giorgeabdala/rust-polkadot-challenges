## Challenge 3: Simulating a Simple Storage Migration

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

In this challenge, you will simulate a very basic storage migration for a "pallet". The idea is to understand how stored state can be transformed when a pallet's logic evolves.

Let's consider two versions of a storage item:
- **V1:** Stores a simple `u32` value.
- **V2:** Stores a tuple `(u32, bool)`. The intention is that the `u32` is the value from V1, and the `bool` is a new indicator (for example, `is_migrated_data: true`).

You will implement a structure that simulates a pallet's storage and a function that performs the migration from V1 to V2.

**Main Concepts Covered:**
- **`Structs` and `Enums`:** To define the structure of our simulated "pallet" and storage versioning.
- **`Option<T>`:** To represent values that may or may not exist in storage.
- **Pattern Matching:** To handle different states and versions of storage.
- **Migration Logic:** Implement data transformation from V1 to V2.
- **Storage Versioning:** Control when migration should occur.

### Structures to Implement:

#### **`Config` Trait (Minimal):**
```rust
pub trait Config {
    // For this challenge, can be empty or define types you find useful,
    // but not strictly necessary for the main migration logic.
    // Example: type Weight = u64; (for migration function return)
}
```

#### **`StorageVersion` Enum:**
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StorageVersion {
    V1_SimpleU32,
    V2_U32WithFlag,
}
```

#### **`PalletStorageSim<T: Config>` Struct:**
This struct simulates our pallet's storage state.
    ```rust
pub struct PalletStorageSim<T: Config> {
    // Current version of storage schema.
    pub current_version: StorageVersion,

    // Simulates V1 storage. Contains value if version is V1_SimpleU32.
    // Will be 'None' after successful migration to V2.
    storage_v1_value: Option<u32>,

    // Simulates V2 storage. Contains value if version is V2_U32WithFlag.
    // Will be populated during migration.
    storage_v2_value: Option<(u32, bool)>,

    _phantom: core::marker::PhantomData<T>, // To use T: Config
}
```

### Methods of `PalletStorageSim<T: Config>`:

#### **`pub fn new() -> Self`**
- Initializes `current_version` to `StorageVersion::V1_SimpleU32`.
- Initializes `storage_v1_value` and `storage_v2_value` to `None`.

#### **`pub fn set_initial_v1_value(&mut self, value: u32)`**
- Sets `storage_v1_value` to `Some(value)`.
- **Important:** This function should only have effect if `current_version` is `V1_SimpleU32`. If already in V2, you can choose to do nothing or return an error/panic (for this challenge, doing nothing is sufficient).

#### **`pub fn get_current_v2_value(&self) -> Option<(u32, bool)>`**
- Returns a copy of `storage_v2_value` **only if** `current_version` is `V2_U32WithFlag`. Otherwise, returns `None`.

#### **`pub fn run_migration_if_needed(&mut self) -> u64 /* Simulated Weight */`**
This function simulates the `on_runtime_upgrade` hook that would be called during a runtime upgrade.
- Checks `self.current_version`:
  - If `StorageVersion::V1_SimpleU32`:
    - Performs migration:
      - If `self.storage_v1_value` is `Some(old_val)`, then `self.storage_v2_value` becomes `Some((old_val, true))`.
      - If `self.storage_v1_value` is `None`, then `self.storage_v2_value` becomes `None`.
    - "Cleans" old storage: `self.storage_v1_value = None`.
    - Updates version: `self.current_version = StorageVersion::V2_U32WithFlag`.
    - Returns simulated "weight" (e.g., `2` to indicate 1 read and 2 writes - version and new value). If V1 value was `None`, weight can be `1` (1 version read, 1 version write).
  - If already `StorageVersion::V2_U32WithFlag` (or any newer version, if there were any):
    - No action needed.
    - Returns weight `0`.

### Tests

Create a `tests` module and use a simple `TestConfig` struct.

**Test Scenarios:**

1. **Initialization:**
   - Verify that `PalletStorageSim::new()` sets `current_version` to `V1_SimpleU32` and values to `None`.

2. **Set V1 Value:**
   - Create pallet, call `set_initial_v1_value(100)`.
   - Verify that `storage_v1_value` is `Some(100)`.
   - Verify that `get_current_v2_value()` returns `None`.

3. **Migration with Existing Value:**
   - Set a V1 value (e.g., `100`).
   - Call `run_migration_if_needed()`.
   - Verify that `current_version` is `V2_U32WithFlag`.
   - Verify that `storage_v1_value` is `None`.
   - Verify that `storage_v2_value` is `Some((100, true))`.
   - Verify that `get_current_v2_value()` returns `Some((100, true))`.
   - Verify that returned weight is > 0.

4. **Migration with Missing V1 Value:**
   - Create pallet (without calling `set_initial_v1_value`).
   - Call `run_migration_if_needed()`.
   - Verify that `current_version` is `V2_U32WithFlag`.
   - Verify that `storage_v1_value` is `None`.
   - Verify that `storage_v2_value` is `None`.
   - Verify that `get_current_v2_value()` returns `None`.
   - Verify that returned weight is > 0 (due to version update).

5. **Double Migration Attempt:**
   - Perform successful migration.
   - Call `run_migration_if_needed()` again.
   - Verify that state (`current_version`, `storage_v1_value`, `storage_v2_value`) remains unchanged.
   - Verify that returned weight is `0`.

6. **Attempt to Set V1 Value After Migration:**
   - Perform migration.
   - Try calling `set_initial_v1_value(200)`.
   - Verify that `storage_v1_value` remains `None` (or the behavior you defined for this case) and `storage_v2_value` is not affected.

### Expected Output

A functional implementation of `PalletStorageSim<T>` and its methods, passing all unit tests. The code should clearly demonstrate migration logic and version control.

### Theoretical Context

**Runtime Upgrades:** In Substrate, the blockchain logic (the runtime, compiled to Wasm) can be updated on-chain. This allows the blockchain to evolve without hard forks.

**Storage Migrations:** When the structure of data stored by a pallet changes in a new runtime version, a storage migration is necessary. This migration is code that runs once during the upgrade to transform data from the old format to the new.

**`OnRuntimeUpgrade` Trait:** Pallets can implement this trait. Its `on_runtime_upgrade()` function is called by the `Executive` pallet during the runtime upgrade process, after the new runtime code is deployed, but before anything else (like `on_initialize` or transactions) is processed.

**`StorageVersion`:** It's common practice for pallets to maintain their own storage "schema" version. Migration logic checks this version to decide if migration should be executed.
- `VersionedMigration` is a common helper for this in FRAME, but we're simulating the concept manually.

**Importance:** Without correct migrations, the new runtime version could fail to read old data or interpret it incorrectly, leading to inconsistencies or panics.

This simplified challenge focuses on the core mechanics of data transformation and versioning, which are crucial for understanding storage migrations in Substrate.

Advanced Level Flow:
Foundation    ▁▂▃   (Challenges 1-3: Pallet, Weight, Storage)
Core APIs     ▄▅▆   (Challenges 4-6: RPC, Origin, Unsigned)
Integration   ▆▇█   (Challenges 7-9: Inherents, Workers, Hooks) 
Advanced      █▇▆   (Challenges 10-12: Pool, XCM, Runtime)

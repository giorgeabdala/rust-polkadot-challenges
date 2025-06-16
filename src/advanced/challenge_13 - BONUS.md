# Challenge 13: From Simulator to Live Blockchain

**Difficulty Level:** Advanced  
**Estimated Time:** 2.5 hours  
**Type:** Guided Tutorial & Challenge

### 🚀 **Objective: Go Live!**

**Congratulations! You've mastered Rust and simulated a pallet. Now, it's time for the real world.**

This final tutorial will guide you through the entire lifecycle of a real Substrate pallet. We will take the logic from our "Pallet Simulator" and deploy it to a live, local blockchain. By the end, you will have interacted with your own custom logic via a real blockchain UI.

This is the bridge from learning to doing.

**Main Concepts Covered:**
1.  **Substrate Node Template:** Cloning and understanding the standard Substrate project structure.
2.  **FRAME Pallet Development:** Creating a new pallet from scratch inside a node.
3.  **Runtime Integration:** Connecting your pallet to the blockchain's runtime.
4.  **Real On-chain Storage:** Migrating from a `HashMap` simulation to a real `StorageMap`.
5.  **Compilation & Deployment:** Building and running a local Substrate node.
6.  **UI Interaction:** Using Polkadot-JS Apps to interact with your live pallet.

---

### **Part 1: Setting Up a Real Substrate Node (Tutorial)**

In this part, we will prepare a standard Substrate development environment. Follow these steps carefully.

#### **Step 1: Install Substrate Prerequisites**

First, ensure you have all the necessary dependencies. If you've already set up your environment for Substrate, you can skip this. Otherwise, run the following command:

```bash
curl https://get.substrate.io -sSf | bash -s -- --fast
```
This script will install `rustup` and configure it with the correct nightly toolchain and WebAssembly (`wasm`) target required for Substrate development.

#### **Step 2: Clone the Substrate Node Template**

This is the standard starting point for any new Substrate project.

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
```

Take a moment to look at the structure. You'll see `node`, `pallets`, and `runtime` directories, which are the core of a Substrate project.

#### **Step 3: Create Your Pallet Files**

We will create a new directory for our pallet.

```bash
mkdir pallets/counter
```

Now, create the main file for our pallet.

```bash
touch pallets/counter/src/lib.rs
```

#### **Step 4: The Pallet Boilerplate**

Open the new file `pallets/counter/src/lib.rs` and add the standard FRAME boilerplate. This is the skeleton of any pallet.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // 1. Pallet Configuration
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // 2. Pallet Storage
    // TODO: We will define our storage here!

    // 3. Pallet Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO: We will define our events here!
    }

    // 4. Pallet Errors
    #[pallet::error]
    pub enum Error<T> {
        // TODO: We will define our errors here!
    }

    // 5. Pallet Extrinsics (Callable Functions)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO: We will implement our functions here!
    }
}
```

This looks similar to our simulator, but uses powerful macros (`#[pallet::...]`) to generate the necessary code.

#### **Step 5: Add Dependencies**

Open `pallets/counter/Cargo.toml` (you'll need to create this file) and add the necessary dependencies for your pallet.

```toml
[package]
name = "pallet-counter"
version = "1.0.0"
description = "A simple counter pallet."
authors = ["Your Name"]
edition = "2021"
license = "Apache-2.0"
publish = false

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }

frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }

[dev-dependencies]
sp-core = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }
sp-io = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
]
```
*Note: Substrate dependencies are often linked via git to ensure compatibility.*

#### **Step 6: Connect Your Pallet to the Runtime**

Now, we tell the blockchain's runtime that our pallet exists.

1.  **Add the pallet to `runtime/Cargo.toml`:**
    ```toml
    # In runtime/Cargo.toml, add this to the [dependencies] section
    pallet-counter = { path = "../pallets/counter", default-features = false }

    # ... and this to the [features] std section
    "pallet-counter/std",
    ```

2.  **Implement the `Config` trait in `runtime/src/lib.rs`:**
    ```rust
    // In runtime/src/lib.rs

    // Find the `construct_runtime!` macro and add your pallet
    construct_runtime!(
        pub enum Runtime {
            System: frame_system,
            // ... other pallets
            Counter: pallet_counter, // Add this line
        }
    );

    // Now, implement the Config trait for your pallet
    impl pallet_counter::Config for Runtime {
        type RuntimeEvent = RuntimeEvent;
    }
    ```

Phew! That was a lot of setup. But now the exciting part begins.

---

### **Part 2: Implement the Pallet Logic (Your Challenge)**

The skeleton is ready. Your mission is to fill it in.

#### **The Goal**

Re-implement the logic from our simulator, but this time using FRAME's storage, events, and errors.

#### **Your Task**

Modify `pallets/counter/src/lib.rs` to:

1.  **Define Storage:** Create a `StorageValue` to hold a single `u32` counter. Call it `Count`.
    *   **Hint:** Use `#[pallet::storage]` and `pub type Count<T> = StorageValue<_, u32, ValueQuery>;`. `ValueQuery` ensures it defaults to 0 if not set.

2.  **Define Events:** Create events for `CounterIncremented` and `CounterReset`.

3.  **Define Errors:** Create an `CounterOverflow` error.

4.  **Implement `increment` function:**
    *   It should be a `#[pallet::call]` function.
    *   It should read the current count from storage.
    *   Check for overflow. If it would overflow, return `Error::<T>::CounterOverflow`.
    *   Increment the count and write it back to storage.
    *   Deposit a `CounterIncremented` event.
    *   Return `Ok(())`.

5.  **Implement `reset` function:**
    *   It should be a `#[pallet::call]` function.
    *   It should set the count in storage to 0.
    *   Deposit a `CounterReset` event.

**Try to implement this yourself before looking at the solution in the next section!** This is the core skill of a Substrate developer.

---

### **Part 3: The Solution and Going Live (Tutorial)**

Here is the complete implementation for `pallets/counter/src/lib.rs`.

#### **Solution**
```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // --- SOLUTION ---
    // The storage item for our counter.
    #[pallet::storage]
    #[pallet::getter(fn count)]
    pub type Count<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The counter was incremented.
        CounterIncremented { new_value: u32 },
        /// The counter was reset.
        CounterReset { new_value: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The counter has reached its maximum value and cannot be incremented further.
        CounterOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Increment the counter by 1.
        #[pallet::weight(10_000)] // A basic weight, can be configured later
        pub fn increment(origin: OriginFor<T>) -> DispatchResult {
            // Ensure the caller is a signed account
            ensure_signed(origin)?;

            // Read the current value, checking for overflow
            let current_count = Count::<T>::get();
            let new_count = current_count.checked_add(1).ok_or(Error::<T>::CounterOverflow)?;

            // Update the storage
            Count::<T>::put(new_count);

            // Emit an event
            Self::deposit_event(Event::CounterIncremented { new_value: new_count });

            Ok(())
        }

        /// Reset the counter to 0.
        #[pallet::weight(10_000)]
        pub fn reset(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let new_count = 0;

            // Update the storage
            Count::<T>::put(new_count);

            // Emit an event
            Self::deposit_event(Event::CounterReset { new_value: new_count });
            
            Ok(())
        }
    }
}
```

#### **Step 7: Compile and Run the Node**

This is the moment of truth. From the root of the `substrate-node-template` directory, run:

```bash
cargo build --release
```
This will take a while (15-30 minutes on most machines). Grab a coffee!

Once it's done, start your local blockchain:
```bash
./target/release/node-template --dev
```
You should see blocks being produced in your terminal!

#### **Step 8: Interact with Your Pallet!**

1.  Open a web browser and navigate to the **Polkadot-JS Apps UI**: [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/).
2.  It will automatically connect to your local node.
3.  Go to the **"Developer" -> "Extrinsics"** tab.
4.  In the "submit the following extrinsic" dropdown, you should see your pallet: `counter`.
5.  Select `counter` and then choose the `increment()` function.
6.  Click "Submit Transaction" and sign it (using a development account like Alice).
7.  Watch the "Recent Events" on the right side of the screen. You should see your `counter.CounterIncremented` event!

**You have successfully built, deployed, and interacted with a custom piece of blockchain logic. You are now a Substrate developer.** 
```markdown
# Challenge 1  
**Difficulty Level:** Intermediate  

## Title: “FixedTuple” – Generic Structure with Tuple and Array  

### Objective  
Create a generic tuple-struct in Rust that combines:  
1. A value of type `T` (any type that implements `Clone`).  
2. A fixed-size array of `U` with length `N` (const generic), where `U: Default + Copy`.  

### Implement a constructor  
```rust
fn new(value: T, items: &[U]) -> Self
```  
This function should copy up to `N` elements from `items` to the internal array.  
If `items.len() < N`, the remaining elements should be filled with `U::default()`.  
If `items.len() > N`, the excess should be ignored.

### Additionally:
- Derive `parity_scale_codec::Encode`, `parity_scale_codec::Decode`, and `scale_info::TypeInfo` for the struct.
- Ensure the generic type `T` also implements `Encode + Decode + TypeInfo`.

### Tests
In the same file, add a test module with at least the following scenarios:
1. T = `u32`, U = `u8`, N = 4, items = `&[1, 2]` → array should be `[1, 2, 0, 0]`.
2. T = `&'static str`, U = `u8`, N = 3, items = `&[5, 6, 7, 8]` → array = `[5, 6, 7]`.
3. SCALE round-trip: encode and decode your struct, then compare for equality.

### Expected Output
When running `cargo test`, all tests should pass and show that:
```text
FixedTuple(42, [1, 2, 0, 0])
FixedTuple("hey", [5, 6, 7])
```  
can be successfully encoded and decoded with no errors.

### Theoretical Context
- **Generics and `const N: usize`:** allow flexible types and compile-time array sizes.
- **Tuple-struct:** syntax like `struct Name<T, …>(T, [U; N]);` — combines a value and inlined array.
- **Trait bounds (`U: Default + Copy`, `T: Clone + Encode + Decode + TypeInfo`):** ensure required behavior.
- **Derive `Encode`, `Decode` (crate `parity-scale-codec`) and `TypeInfo` (crate `scale-info`)**: provide implementations for (de)serialization and runtime type introspection, standard practice in FRAME pallets.
- **Further reading:**
    - parity-scale-codec: https://crates.parity.io/parity_scale_codec/
    - scale-info: https://docs.rs/scale-info/latest/scale_info/

---

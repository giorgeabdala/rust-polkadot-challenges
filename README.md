# Rust Polkadot Challenges

A comprehensive Rust course focused on Polkadot SDK/Substrate development, structured in three progressive levels.

## 📚 Course Structure

### 🟢 **Beginner Level** (8 challenges - 3h35min)
Essential Rust fundamentals:
- Ownership and Borrowing
- Structs and Enums  
- Pattern Matching
- Basic Error Handling
- Basic Collections

**Dependencies:** Standard library only ✅

### 🟡 **Medium Level** (10 challenges - 6h45min)
Intermediate concepts for Substrate:
- Advanced Collections
- Generics and Traits
- Explicit Lifetimes
- Advanced Error Handling
- Async Programming ⚠️
- SCALE Codec
- Macros
- Smart Pointers
- Concurrency
- Testing and Documentation ⚠️

**Special dependencies:**
- **Challenge 5 (Async):** `tokio`, `futures`
- **Challenge 10 (Benchmarks):** `criterion` (dev-dependency)

### 🔴 **Advanced Level** (8 challenges - 8h)
Advanced Polkadot SDK concepts:
- Storage Patterns
- Transaction Pool
- Consensus Mechanisms
- Cross-Chain Communication (XCM)
- Runtime Development
- Pallet Architecture

**Dependencies:** Some challenges use `serde` and `serde_json` for serialization

## 🚀 Getting Started

### 1. Clone the repository
```bash
git clone <repository-url>
cd RustPolkadotChallenges
```

### 2. Start with the appropriate level
```bash
# For beginners
cd src/beginner

# For intermediate level  
cd src/medium

# For advanced level
cd src/advanced
```

### 3. Install dependencies when needed

Most challenges use only Rust's standard library. When external dependencies are required, you'll find specific instructions at the beginning of the challenge.

#### For challenges with dependencies:

**Medium Challenge 5 (Async Programming):**
```bash
cargo add tokio --features full
cargo add futures
```

**Medium Challenge 10 (Benchmarking):**
```bash
cargo add --dev criterion --features html_reports
```

**Advanced Challenge 3 (JSON-RPC):**
```bash
cargo add serde --features derive
cargo add serde_json
```

**Advanced Challenge 7 (Inherents):**
```bash
cargo add serde --features derive
cargo add bincode
```

### 4. Run tests
```bash
cargo test
cargo check
```

## 📋 Dependencies by Level

### Beginner (0 external dependencies)
- ✅ All challenges use only `std`

### Medium (2 challenges with dependencies)
- ✅ Challenges 1-4, 6-9: only `std`
- ⚠️ Challenge 5: `tokio`, `futures`
- ⚠️ Challenge 10: `criterion` (benchmarks only)

### Advanced (2 challenges with dependencies)
- ✅ Challenges 1-2, 4-6, 8-12: only `std`
- ⚠️ Challenge 3: `serde`, `serde_json` (for JSON-RPC)
- ⚠️ Challenge 7: `serde`, `bincode` (for inherent data)

## 🎯 Learning Objectives

This course prepares you for:
- ✅ Safe and efficient Rust development
- ✅ Deep understanding of Polkadot SDK
- ✅ Creating Substrate pallets
- ✅ Blockchain runtime development
- ✅ Consensus and networking implementation
- ✅ Cross-chain communication (XCM)

## 📖 How to Use This Course

1. **Follow the order:** Challenges are progressive
2. **Read completely:** Each challenge has context and examples
3. **Implement first:** Try to solve before looking at solutions
4. **Always test:** Use `cargo test` and `cargo check`
5. **Explore bonuses:** Extra challenges for deeper learning

## 🔧 Requirements

- **Rust:** 1.70+ (recommended: latest version)
- **Cargo:** Included with Rust
- **Editor:** VS Code with rust-analyzer (recommended)

## 📝 Challenge Structure

Each challenge includes:
- 🎯 **Learning objectives**
- 📚 **Theoretical background**
- 💻 **Practical requirements**
- ✅ **Usage examples**
- 🧪 **Testing section**
- 🔗 **Substrate connection**
- 🏆 **Bonus challenges**

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a branch for your feature
3. Commit your changes
4. Open a Pull Request

## 📄 License

This project is under the MIT license. See the LICENSE file for details.

## 🆘 Support

If you encounter problems:
1. Check that you have the correct Rust version
2. Confirm that dependencies are installed
3. Run `cargo clean` and try again
4. Open an issue in the repository

---

**Total estimated time:** 18h20min
**Total challenges:** 26
**Complete preparation for Polkadot SDK development** 🚀

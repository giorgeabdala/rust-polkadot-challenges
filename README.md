# Rust Polkadot Challenges

A comprehensive Rust course focused on Polkadot SDK/Substrate development, structured in three progressive levels with 29 total challenges.

## 📚 Course Structure

### 🟢 **Beginner Level** (9 challenges - 4h15min)
Essential Rust fundamentals:
- Environment Setup and Cargo Basics
- Ownership and Borrowing
- Structs and Enums  
- Pattern Matching
- Basic Error Handling with Propagation
- Basic Collections

**Dependencies:** Standard library only ✅

### 🟡 **Medium Level** (10 challenges - 6h55min)
Intermediate concepts for Substrate:
- Advanced Collections
- Generics and Traits
- Explicit Lifetimes
- Advanced Error Handling
- Async Programming ⚠️
- SCALE Codec
- Macros and Performance Benchmarking
- Smart Pointers
- Concurrency
- Testing and Documentation ⚠️

**Special dependencies:**
- **Challenge 5 (Async):** `tokio`, `futures`
- **Challenge 10 (Benchmarks):** `criterion` (dev-dependency)

### 🔴 **Advanced Level** (12 challenges - 8h)
Advanced Polkadot SDK concepts (reorganized for optimal learning flow):
- **Foundation:** Pallet Architecture and Weight System
- **Storage:** Migration Patterns and Persistence  
- **External APIs:** Custom RPC and Authorization
- **Transactions:** Unsigned Validation and Inherents
- **Processing:** Off-chain Workers and Runtime Hooks
- **Advanced:** Transaction Pool, XCM, and Runtime Integration

**Dependencies:** Some challenges use `serde` and `serde_json` for serialization

## 🚀 Getting Started

### 1. Clone the repository
```bash
git clone <repository-url>
cd RustPolkadotChallenges
```

### 2. Start with the appropriate level
```bash
# For beginners - start with Challenge 0
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
- ✅ Complete Rust development environment setup
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
5. **Explore bonuses:** ⚠️ Optional advanced concepts - only for deeper exploration

⏱️ **Time Estimates:** All estimated times include only the core requirements and basic testing. Bonus challenges (marked with ⚠️) are optional advanced concepts that require additional time and are designed for students who want deeper exploration of specific topics.

**📊 Total Course Time:**
- **Core Requirements Only:** 19h10min (4h15min + 6h55min + 8h)
- **With All Bonus Challenges:** ~25h30min (+30-35% additional time)
- **Recommended Pace:** 2-3 challenges per week for steady progress

**🎯 Bonus Challenge Philosophy:** We've streamlined bonus challenges to focus only on concepts that are either:
- Essential patterns used in Substrate development
- Advanced exploration for students seeking deeper understanding
- Previously, many bonus challenges were simply "more of the same" - these have been removed to reduce cognitive load

## 🔧 Requirements

- **Rust:** 1.70+ (recommended: latest version)
- **Cargo:** Included with Rust
- **Editor:** VS Code with rust-analyzer (recommended)

## 🚨 Troubleshooting Common Issues

### Installation Problems

**Rust Installation Fails:**
```bash
# Clear previous installation
rustup self uninstall

# Reinstall with verbose output
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -v
```

**Platform-Specific Issues:**
- **Windows:** Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- **Linux:** `sudo apt update && sudo apt install build-essential pkg-config libssl-dev`
- **macOS:** `xcode-select --install`

### Compilation Errors

**"linker not found" or "link.exe not found":**
- Windows: Install Visual Studio Build Tools
- Linux: Install `build-essential`
- macOS: Install Xcode command line tools

**"error: failed to run custom build command for openssl-sys":**
```bash
# Linux
sudo apt install libssl-dev pkg-config

# macOS
brew install openssl
export OPENSSL_DIR=/usr/local/opt/openssl
```

**Memory/Performance Issues:**
```bash
# Reduce parallel jobs if system has limited RAM
export CARGO_BUILD_JOBS=1

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

### Development Environment

**rust-analyzer not working:**
1. Restart VS Code
2. Check if `rust-analyzer` extension is installed and enabled
3. Run `cargo check` in terminal to ensure project compiles
4. Check VS Code settings: `"rust-analyzer.check.command": "clippy"`

**Slow compilation:**
```bash
# Use faster linker
# Linux
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# macOS
export RUSTFLAGS="-C link-arg=-fuse-ld=/usr/local/bin/zld"

# Enable incremental compilation
export CARGO_INCREMENTAL=1
```

**"overflow evaluating the requirement" errors:**
- Usually indicates circular dependency or complex generic constraints
- Check for infinite recursion in trait implementations
- Simplify generic constraints step by step

### Runtime Errors

**Stack overflow in recursive code:**
```bash
# Increase stack size temporarily
export RUST_MIN_STACK=8388608

# Or use explicit stack in code
std::thread::Builder::new()
    .stack_size(8 * 1024 * 1024)
    .spawn(|| { /* your code */ })
```

**"thread 'main' panicked at 'index out of bounds'":**
- Always use `.get()` instead of direct indexing for Vec/arrays
- Use `checked_add`, `checked_sub` for arithmetic that might overflow

### Substrate-Specific Issues

**"substrate" or "polkadot-sdk" dependency conflicts:**
```bash
# Clear cargo cache
cargo clean
rm Cargo.lock

# Update dependencies
cargo update
```

**WASM compilation failures:**
```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Update to latest nightly (if using nightly features)
rustup update nightly
```

### Getting Help

1. **Read the error message carefully** - Rust errors are usually very descriptive
2. **Check the Rust Book**: https://doc.rust-lang.org/book/
3. **Search issues**: Many problems are already solved on Stack Overflow
4. **Use `cargo clippy`**: Often suggests better ways to write code
5. **Community**: Rust Discord, Reddit r/rust, Substrate Stack Exchange

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

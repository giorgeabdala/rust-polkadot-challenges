# Challenge 0: Environment Setup and First Steps

**Estimated Time:** 25 minutes  
**Difficulty:** Beginner  
**Topics:** Rust Installation, Cargo Basics, Hello World, Project Structure

## Learning Objectives

By completing this challenge, you will understand:
- How to install and configure Rust and Cargo
- Basic Cargo commands and project structure
- How to create, build, and run Rust programs
- Essential development tools for Rust
- Setting up your development environment for Substrate development

## Background

Before diving into Rust programming concepts, it's essential to have a properly configured development environment. Rust comes with `cargo`, a powerful build system and package manager that makes managing Rust projects straightforward.

This challenge will guide you through the essential setup and introduce you to the basic workflow you'll use throughout the course.

## Prerequisites

- A computer with internet connection
- Basic familiarity with command line/terminal
- A code editor (VS Code recommended)

## Challenge

Set up your Rust development environment and create your first Rust program.

### Part 1: Install Rust and Cargo

1. **Install Rust using rustup:**
   ```bash
   # On Linux/macOS
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # On Windows, download and run rustup-init.exe from https://rustup.rs/
   ```

2. **Configure your shell:**
   ```bash
   source ~/.cargo/env
   ```

3. **Verify installation:**
   ```bash
   rustc --version
   cargo --version
   rustup --version
   ```

4. **Install essential components:**
   ```bash
   # Install stable toolchain (if not already installed)
   rustup install stable
   rustup default stable
   
   # Install Rust formatter and linter
   rustup component add rustfmt clippy
   ```

### Part 2: Set up Development Tools

1. **Install VS Code extensions (recommended):**
   - rust-analyzer (official Rust language server)
   - CodeLLDB (for debugging)
   - Better TOML (for Cargo.toml files)

2. **Alternative editors:**
   - **IntelliJ IDEA:** Install Rust plugin
   - **Vim/Neovim:** Configure with rust-analyzer
   - **Emacs:** Use rust-mode with lsp-mode

### Part 3: Create Your First Rust Project

1. **Create a new Cargo project:**
   ```bash
   cargo new hello_substrate --bin
   cd hello_substrate
   ```

2. **Understand the project structure:**
   ```
   hello_substrate/
   ├── Cargo.toml      # Project configuration and dependencies
   ├── src/
   │   └── main.rs     # Main source file
   └── .gitignore      # Git ignore file
   ```

3. **Examine the generated files:**
   - Look at `Cargo.toml` - understand the `[package]` section
   - Look at `src/main.rs` - see the basic "Hello, world!" program

### Part 4: Basic Cargo Commands

Learn these essential commands:

1. **Build your project:**
   ```bash
   cargo build        # Debug build
   cargo build --release  # Optimized release build
   ```

2. **Run your project:**
   ```bash
   cargo run          # Build and run in one command
   ```

3. **Check your code:**
   ```bash
   cargo check        # Fast compilation check without executable
   cargo clippy       # Lint your code for common mistakes
   cargo fmt          # Format your code
   ```

4. **Test your code:**
   ```bash
   cargo test         # Run all tests
   ```

### Part 5: Customize Your First Program

Replace the contents of `src/main.rs` with:

```rust
fn main() {
    println!("🚀 Welcome to Rust for Polkadot SDK!");
    println!("🦀 This is my first Rust program");
    
    // Let's practice some basic Rust concepts
    let name = "Future Substrate Developer";
    let version = "1.0.0";
    
    println!("👋 Hello, {}!", name);
    println!("📦 Project version: {}", version);
    
    // Basic arithmetic
    let challenges_completed = 0;
    let total_challenges = 26;
    let progress = (challenges_completed as f32 / total_challenges as f32) * 100.0;
    
    println!("📊 Progress: {:.1}% ({}/{})", progress, challenges_completed, total_challenges);
    
    // Demonstrate basic data types
    demonstrate_types();
}

fn demonstrate_types() {
    println!("\n🔢 Rust Basic Types:");
    
    // Integers
    let balance: u64 = 1_000_000;
    println!("💰 Account balance: {} tokens", balance);
    
    // Floating point
    let exchange_rate: f64 = 1.337;
    println!("💱 Exchange rate: {:.3}", exchange_rate);
    
    // Boolean
    let is_validator: bool = true;
    println!("⛏️  Is validator: {}", is_validator);
    
    // String types
    let network = "Polkadot";
    let chain_name = String::from("Substrate");
    println!("🌐 Network: {}, Chain: {}", network, chain_name);
    
    // Arrays and vectors
    let validators = ["Alice", "Bob", "Charlie"];
    let mut block_numbers = vec![1, 2, 3, 4, 5];
    block_numbers.push(6);
    
    println!("👥 Validators: {:?}", validators);
    println!("📦 Block numbers: {:?}", block_numbers);
}
```

### Part 6: Run and Test

1. **Run your customized program:**
   ```bash
   cargo run
   ```

2. **Check code quality:**
   ```bash
   cargo clippy
   cargo fmt --check
   ```

3. **Build for release:**
   ```bash
   cargo build --release
   ```

## Expected Output

Your program should produce output similar to:
```
🚀 Welcome to Rust for Polkadot SDK!
🦀 This is my first Rust program
👋 Hello, Future Substrate Developer!
📦 Project version: 1.0.0
📊 Progress: 0.0% (0/26)

🔢 Rust Basic Types:
💰 Account balance: 1000000 tokens
💱 Exchange rate: 1.337
⛏️  Is validator: true
🌐 Network: Polkadot, Chain: Substrate
👥 Validators: ["Alice", "Bob", "Charlie"]
📦 Block numbers: [1, 2, 3, 4, 5, 6]
```

## Testing Your Setup

Create a simple test to verify everything works:

1. **Add this to the bottom of `src/main.rs`:**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_basic_arithmetic() {
           let a = 5;
           let b = 3;
           assert_eq!(a + b, 8);
           assert_eq!(a * b, 15);
       }

       #[test]
       fn test_string_operations() {
           let greeting = "Hello";
           let name = "Substrate";
           let message = format!("{}, {}!", greeting, name);
           assert_eq!(message, "Hello, Substrate!");
       }
   }
   ```

2. **Run the tests:**
   ```bash
   cargo test
   ```

## Essential Cargo.toml Configuration

Update your `Cargo.toml` to include useful metadata:

```toml
[package]
name = "hello_substrate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "My first Rust program for Polkadot SDK development"
repository = "https://github.com/yourusername/hello_substrate"
license = "MIT"

[dependencies]
# Dependencies will be added in future challenges

[dev-dependencies]
# Development dependencies for testing

# Cargo configuration for faster builds
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = true
```

## Key Learning Points

After completing this challenge, you should understand:

- **Cargo Workflow**: `cargo new`, `cargo build`, `cargo run`, `cargo test`
- **Project Structure**: Where to put source files and how Cargo organizes projects
- **Basic Rust Syntax**: Variables, functions, basic data types
- **Development Tools**: rustfmt, clippy, and their importance
- **Testing**: How to write and run basic tests

## Troubleshooting Common Issues

### Installation Problems
- **Windows**: Make sure to install Visual Studio Build Tools
- **Linux**: Install build essentials: `sudo apt install build-essential`
- **macOS**: Install Xcode command line tools: `xcode-select --install`

### Editor Issues
- **VS Code**: Ensure rust-analyzer extension is installed and active
- **Path Issues**: Run `source ~/.cargo/env` or restart your terminal

### Build Errors
- **Outdated Rust**: Run `rustup update`
- **Permissions**: Ensure you have write access to your project directory

## Next Steps

With your environment set up, you're ready to tackle the fundamental Rust concepts in the upcoming challenges:

1. **Challenge 1**: Ownership and Move Semantics
2. **Challenge 2**: References and Borrowing
3. **Challenge 3**: Structs and Implementation Blocks

## Substrate Connection

This setup prepares you for Substrate development by:
- **Cargo**: Same tool used for Substrate projects
- **Rust Edition 2021**: Current standard for Substrate development
- **Development Tools**: Essential for maintaining code quality in blockchain projects
- **Testing**: Critical for blockchain applications where bugs can be costly

## Bonus Tasks

1. **Explore cargo help**: Run `cargo help` and explore different subcommands
2. **Create a library**: Try `cargo new --lib my_substrate_lib`
3. **Add documentation**: Add `///` comments to your functions and run `cargo doc --open`
4. **Experiment with features**: Learn about `Cargo.toml` features and conditional compilation

---

🎉 **Congratulations!** You've successfully set up your Rust development environment and created your first program. You're now ready to dive deeper into Rust concepts that will prepare you for Substrate development! 
// Advanced Level Challenges - Rust for Polkadot SDK Development
// 
// This module contains 12 advanced-level challenges designed to teach
// Substrate-specific concepts and blockchain development patterns.
//
// Total estimated time: 8 hours (480 minutes)
//
// Challenge progression (reorganized for better learning flow):
// 1. Basic Pallet Simulator (2h) - Foundation pallet concepts
// 2. Transaction Weight Simulation (1h) - Weight system understanding
// 3. Storage Migration (2h) - Storage evolution and migration patterns
// 4. Simple Custom RPC (2h) - External API and runtime queries  
// 5. Simple Custom Origin Pallet (1.5h) - Authorization and origin concepts
// 6. Unsigned Ping with Validation (2h) - Unsigned transactions and validation
// 7. Inherents - Timestamp and External Data (2h) - External data integration
// 8. Asynchronous Worker Simulator (2h) - Off-chain workers and async processing
// 9. Runtime Hooks - Automatic Cleanup (1.5h) - Runtime lifecycle hooks
// 10. Transaction Pool and Prioritization (2h) - Transaction management
// 11. Simple Asset Teleportation via XCM (2.5h) - Cross-chain communication
// 12. Substrate Node Template and Runtime Configuration (2h) - Complete runtime integration

pub mod challenge_01;  // Basic Pallet Simulator (Pure Rust)
pub mod challenge_02;  // Transaction Weight Simulation with WeightInfo
pub mod challenge_03;  // Simulating a Simple Storage Migration
pub mod challenge_04;  // Simple Custom RPC for Counter Pallet
pub mod challenge_05;  // Simple Custom Origin Pallet
pub mod challenge_06;  // Unsigned Ping with Validation
pub mod challenge_07;  // Inherents - Timestamp and External Data
pub mod challenge_08;  // Asynchronous Worker Simulator for Data Collection
pub mod challenge_09;  // Runtime Hooks - Automatic Cleanup System
pub mod challenge_10;  // Transaction Pool and Prioritization
pub mod challenge_11;  // Simple Asset Teleportation via Mocked XCM
pub mod challenge_12;  // Substrate Node Template and Runtime Configuration
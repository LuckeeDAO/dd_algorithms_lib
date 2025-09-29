#![no_std]

//! # Decentralized Decision Library
//!
//! A library for decentralized decision making, fair division algorithms, and random number generation.
//! This library focuses on consensus-friendly, objective algorithms.
//!
//! ## Features
//!
//! ### ⚖️ Fair Division Algorithms
//! - Equal weights and weighted fair division
//! - Super fair division algorithms
//! - Optimal resource allocation
//!
//! ### 🎲 Decentralized Random Number Generation
//! - Single and multiple random number generation
//! - Collision resistance and uniqueness guarantees
//! - Offset mechanism for pattern prevention
//!
//! ## Quick Start
//!
//! ```rust
//! use dd_algorithms_lib::{
//!     // Fair Division
//!     calculate_fair_division_equal_weights,
//!     calculate_fair_division_weighted,
//!     
//!     // Random Generation
//!     get_one_dd_rand_num,
//!     get_k_dd_rand_num,
//! };
//! ```
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples.

// Core modules
pub mod algorithms;
pub mod types;

// Unit tests for algorithms live in `src/test.rs`
#[cfg(test)]
mod test;

// Re-export main functionality
pub use algorithms::*;
pub use types::*;

// Common error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid input parameters
    InvalidInput,
    /// Calculation failed
    CalculationFailed,
    /// Not enough participants (minimum 2)
    NotEnoughParticipants,
}

/// Result type for the library
pub type Result<T> = core::result::Result<T, Error>;

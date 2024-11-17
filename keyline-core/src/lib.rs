//! Core functionality shared across Keyline components
//! 
//! This crate provides fundamental types and utilities used by all Keyline crates.
//! It includes error handling, common types, and shared utilities.

pub mod error;
pub mod types;
pub mod utils;

// Re-export common types for convenience
pub use error::{Error, Result};

/// Prelude module containing commonly used types
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::types::*;
}

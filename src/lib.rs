// #![deny(missing_docs)]

//! # KvStore
//!
//! `KvStore` is a simple key-value store implemented in Rust.
//!
//! /// Key-value store implementation
pub mod engines;
pub mod error;
pub mod network;
pub mod protocol;

// Re-export main types for convenience
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsErrors, KvsResult};

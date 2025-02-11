// #![deny(missing_docs)]

//! # KvStore
//!
//! `KvStore` is a simple key-value store implemented in Rust.
//!
//! /// Key-value store implementation
pub mod error;
pub mod kv;
pub mod network;
pub mod protocol;
// Re-export main types for convenience
pub use error::{KvsErrors, KvsResult};
pub use kv::KvStore;

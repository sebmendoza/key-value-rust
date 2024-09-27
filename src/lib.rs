#![deny(missing_docs)]

//! # KvStore
//!
//! `KvStore` is a simple key-value store implemented in Rust.
//!
//! ## Example
//!
//! ```
//! use kvs::KvStore;
//!
//! let mut store = KvStore::new();
//! store.set("key".to_string(), "value".to_string());
//! let value = store.get("key".to_string());
//! assert_eq!(value, Some("value".to_string()));
//! store.remove("key".to_string());
//! let value = store.get("key".to_string());
//! assert_eq!(value, None);
//! ```

use std::collections::HashMap;

/// A simple in-memory key-value store.
///
/// `KvStore` has one field called store. This is where the database is held.
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

// another way to initialize empty kvstore
impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Creates a new empty `KvStore`.
    ///
    /// # Example
    /// ```
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    //// Inserts a key-value pair into the store. Overwrites the value if the key already exists.
    ///
    /// # Example
    /// ```
    /// let mut store = KvStore::new();
    /// store.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Gets the value for a given key. Returns `None` if the key is not found.
    ///
    /// # Example
    /// ```
    /// let value = store.get("key".to_string());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).map(|s| s.to_owned())
    }

    /// Removes a key-value pair from the store.
    ///
    /// # Example
    /// ```
    /// store.remove("key".to_string());
    /// ```    
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}

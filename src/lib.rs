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

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::{collections::HashMap, io::Write};

/// Error handling module for KvStore.
pub mod error;
use error::{KvsErrors, KvsResult};

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

/// A simple in-memory key-value store.
///
/// `KvStore` has one field called store. This is where the database is held.
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
    log_file: File,
    buff_writer: BufWriter<File>,
}

// another way to initialize empty kvstore
// impl Default for KvStore {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl KvStore {
    /// Creates a new empty `KvStore`.
    ///
    /// # Example
    /// ```
    /// let mut store = KvStore::new();
    /// ```
    pub fn new(file_handle: File, buff_writer: BufWriter<File>) -> KvStore {
        KvStore {
            store: HashMap::new(),
            log_file: file_handle,
            buff_writer
        }
    }

    //// Inserts a key-value pair into the store. Overwrites the value if the key already exists.
    ///
    /// # Example
    /// ```
    /// let mut store = KvStore::new();
    /// store.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> KvsResult<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        let serialized = serde_json::to_string(&cmd)?;

        writeln!(self.buff_writer, "{}", serialized)?;
        self.buff_writer.flush()?;

        self.store.insert(key, value);
        Ok(())
    }

    /// Gets the value for a given key. Returns `None` if the key is not found.
    ///
    /// # Example
    /// ```
    /// let value = store.get("key".to_string());
    /// ```
    pub fn get(&self, key: String) -> KvsResult<Option<String>> {
        match self.store.get(&key) {
            Some(value) => Ok(Some(value.to_owned())),
            None => Ok(None),
        }
    }

    /// Removes a key-value pair from the store.
    ///
    /// # Example
    /// ```
    /// store.remove("key".to_string());
    /// ```    
    pub fn remove(&mut self, key: String) -> KvsResult<()> {
        if !self.store.contains_key(&key) {
            return Err(KvsErrors::KeyNotFound(key));
        }

        let cmd = Command::Remove { key: key.clone() };
        let serialized = serde_json::to_string(&cmd)?;

        
        writeln!(self.buff_writer, "{}", serialized)?;
        self.buff_writer.flush()?;
        self.store.remove(&key);

        Ok(())
    }

    /// Open the KvStore at a given path. Return the KvStore.   
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<KvStore> {
        let path: PathBuf = path.into(); // Need it to convert to pure PathBuf https://doc.rust-lang.org/beta/std/convert/trait.Into.html
        let file: File = File::options()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        let buffed_writer = BufWriter::new(file.try_clone()?);

        let new: KvStore = KvStore {
            store: HashMap::new(),
            log_file: file,
            buff_writer: buffed_writer
        };
        Ok(new)
        // Need a Buffer!!!!!1 That's how we read the actual lines in the file.
        // https://www.geeksforgeeks.org/i-o-buffering-and-its-various-techniques/
    }
}

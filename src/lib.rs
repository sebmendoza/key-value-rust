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
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

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
    // log_file: File,
    buff_writer: BufWriter<File>,
    buff_reader: BufReader<File>,
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
    pub fn new(
        // file_hsandle: File,
        buff_writer: BufWriter<File>,
        buff_reader: BufReader<File>,
    ) -> KvStore {
        KvStore {
            store: HashMap::new(),
            // log_file: file_handle,
            buff_writer,
            buff_reader,
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
        // self.buff_writer.flush()?;
        // self.buff_writer.write(serialized.as_bytes())?;
        self.store.insert(key, value);
        Ok(())
    }

    /// Gets the value for a given key. Returns `None` if the key is not found.
    ///
    /// # Example
    /// ```
    /// let value = store.get("key".to_string());
    /// ```
    pub fn get(&mut self, key: String) -> KvsResult<Option<String>> {
        self.store = self.populate_store()?;
        match self.store.get(&key) {
            Some(value) => {
                println!("{}", value);
                Ok(Some(value.to_owned()))
            }
            None => {
                print!("Key not found"); // Print "Key not found"
                Ok(None)
            }
        }
    }

    /// Removes a key-value pair from the store.
    ///
    /// # Example
    /// ```
    /// store.remove("key".to_string());
    /// ```    
    pub fn remove(&mut self, key: String) -> KvsResult<()> {
        self.store = self.populate_store()?;
        if !self.store.contains_key(&key) {
            print!("Key not found");
            return Err(KvsErrors::KeyNotFound());
        }

        let cmd: Command = Command::Remove { key: key.clone() };
        let serialized = serde_json::to_string(&cmd)?;
        // self.buff_writer.write(serialized.as_bytes())?;
        writeln!(self.buff_writer, "{}", serialized)?;
        self.store.remove(&key);
        // self.buff_writer.flush()?;

        Ok(())
    }

    /// Open the KvStore at a given path. Return the KvStore.   
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<KvStore> {
        let mut path: PathBuf = path.into(); // Need it to convert to pure PathBuf https://doc.rust-lang.org/beta/std/convert/trait.Into.html
        if path.is_dir() {
            path.push("log.txt");
        }
        let file: File = File::options()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        let writer = BufWriter::new(file.try_clone()?);
        let reader = BufReader::new(file.try_clone()?);
        let new = KvStore::new(writer, reader);
        Ok(new)
        // Need a Buffer!!!!!1 That's how we read the actual lines in the file.
        // https://www.geeksforgeeks.org/i-o-buffering-and-its-various-techniques/
    }

    fn populate_store(&mut self) -> KvsResult<HashMap<String, String>> {
        let mut populated_store = HashMap::new();
        for line in self.buff_reader.by_ref().lines() {
            let line = line?;
            let processed_str = line.trim();
            if processed_str.is_empty() {
                continue;
            }
            let cmd: Command = serde_json::from_str(processed_str)?; // Deseraialize

            match cmd {
                Command::Set { key, value } => {
                    // println!("This is the cmd for set: {} {}", key, value);
                    populated_store.insert(key, value);
                }
                Command::Remove { key } => {
                    // println!("This is the cmd for remove: {}", key);
                    populated_store.remove(&key);
                }
            }
        }
        Ok(populated_store)
    }
}

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
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const COMPACTION_THRESHOLD: u64 = 1024 * 1024; // 1MB
const REDUNDANCY_THRESHOLD: f64 = 0.5; // 50% redundancy

/// Error handling module for KvStore.
pub mod error;
use error::{KvsErrors, KvsResult};

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug)]
struct DiskInfo {
    log_index: u64,
    pos: u64,
}

#[derive(Debug)]
struct LogFile {
    id: u64,
    path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

impl LogFile {
    pub fn new(id: u64, dir_path: &PathBuf) -> KvsResult<Self> {
        let file_path = dir_path.join(format!("{}.log", id));
        let file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&file_path)?;
        let reader = BufReader::new(file.try_clone()?);
        let writer = BufWriter::new(file);
        Ok(LogFile {
            id,
            path: file_path,
            reader,
            writer,
        })
    }

    fn count_total_length(&mut self) -> KvsResult<u64> {
        self.reader.seek(SeekFrom::Start(0))?;
        let mut count = 0;
        let mut line = String::new();
        while self.reader.read_line(&mut line)? > 0 {
            if !line.trim().is_empty() {
                count += 1;
            }
            line.clear();
        }
        Ok(count)
    }
}

/// A simple in-memory key-value store.
///
/// `KvStore` has a store, buff_writer, and buff_reader.
#[derive(Debug)]
pub struct KvStore {
    mem_index: HashMap<String, DiskInfo>, // maps keys to commandINFO
    logs: HashMap<u64, LogFile>,          // maps log_id to LogFile
    current_log_id: u64,
    dir_path: PathBuf,
}

// another way to initialize empty kvstore
// impl Default for KvStore {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl KvStore {
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
            value,
        };
        let current_log = self.logs.get_mut(&self.current_log_id)
            .ok_or(KvsErrors::LogNotFound())?;
        
        let pos = current_log.writer.seek(SeekFrom::End(0))?;
        let serialized = serde_json::to_string(&cmd)?;
        writeln!(current_log.writer, "{}", serialized)?;
        current_log.writer.flush()?;

        self.mem_index.insert(
            key,
            DiskInfo {
                log_index: self.current_log_id,
                pos,
            },
        );

        if pos >= COMPACTION_THRESHOLD {
            self.maybe_compact()?;
        }

        Ok(())
    }

    /// Gets the value for a given key. Returns `None` if the key is not found.
    ///
    /// # Example
    /// ```
    /// let value = store.get("key".to_string());
    /// ```
    pub fn get(&mut self, key: String) -> KvsResult<Option<String>> {
        if let Some(disk_info) = self.mem_index.get(&key) {
            let log = self.logs.get_mut(&disk_info.log_index)
                .ok_or(KvsErrors::LogNotFound())?;
            
            log.reader.seek(SeekFrom::Start(disk_info.pos))?;
            let mut line = String::new();
            log.reader.read_line(&mut line)?;
            
            let cmd: Command = serde_json::from_str(line.trim())?;
            if let Command::Set { key: _, value } = cmd {
                print!("{}", value); // Print the value directly for found keys
                return Ok(Some(value));
            }
        }
        print!("Key not found"); // Print "Key not found" for missing keys
        Ok(None)
    }

    /// Removes a key-value pair from the store.
    ///
    /// # Example
    /// ```
    /// store.remove("key".to_string());
    /// ```    
    pub fn remove(&mut self, key: String) -> KvsResult<()> {
        if !self.mem_index.contains_key(&key) {
            print!("Key not found");
            return Err(KvsErrors::KeyNotFound());
        }

        let cmd = Command::Remove { key: key.clone() };
        let current_log = self.logs.get_mut(&self.current_log_id)
            .ok_or(KvsErrors::LogNotFound())?;
        
        let serialized = serde_json::to_string(&cmd)?;
        writeln!(current_log.writer, "{}", serialized)?;
        current_log.writer.flush()?;

        self.mem_index.remove(&key);
        Ok(())
    }

    /// Open the KvStore at a given path. Return the KvStore.   
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<KvStore> {
        let dir_path = path.into();
        fs::create_dir_all(&dir_path)?;

        let mut logs = HashMap::new();
        let mut current_log_id = 0;

        // Load existing logs
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("log") {
                let id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.parse::<u64>().ok())
                    .ok_or(KvsErrors::InvalidLogFile())?;
                current_log_id = current_log_id.max(id);
                logs.insert(id, LogFile::new(id, &dir_path)?);
            }
        }

        // Create initial log if none exist
        if logs.is_empty() {
            logs.insert(0, LogFile::new(0, &dir_path)?);
        }

        // Create store and populate index
        let mut store = KvStore {
            mem_index: HashMap::new(),
            logs,
            current_log_id,
            dir_path,
        };
        store.mem_index = store.populate_mem_index()?;
        
        Ok(store)
    }

    fn populate_mem_index(&mut self) -> KvsResult<HashMap<String, DiskInfo>> {
        let mut new_mem_index: HashMap<String, DiskInfo> = HashMap::new();

        //Process the different log files
        let log_ids: Vec<_> = self.logs.keys().copied().collect();
        for log_id in log_ids {
            let log = self.logs.get_mut(&log_id).ok_or(KvsErrors::LogNotFound())?;
            log.reader.seek(SeekFrom::Start(0))?; // Rewind the buffer reader to the start of the file
            let mut pos: u64 = 0;

            while let Some(line) = log.reader.by_ref().lines().next() {
                let line: String = line?;
                let processed_str = line.trim();
                if processed_str.is_empty() {
                    continue;
                }

                let cmd: Command = serde_json::from_str(processed_str)?; // Deserialize

                match cmd {
                    Command::Set { key, .. } => {
                        // println!("This is the cmd for set: {} {}", key, value);
                        new_mem_index.insert(
                            key,
                            DiskInfo {
                                log_index: log_id,
                                pos,
                            },
                        );
                    }
                    Command::Remove { key } => {
                        // println!("This is the cmd for remove: {}", key);
                        new_mem_index.remove(&key);
                    }
                }

                pos = log.reader.stream_position()?;
            }
        }
        Ok(new_mem_index)
    }

    fn maybe_compact(&mut self) -> KvsResult<()> {
           // First, check if compaction is really needed
           let (total_bytes, stale_bytes) = self.analyze_log_waste()?;
        
           // Only compact if we have significant waste (> 50% redundancy)
           if stale_bytes as f64 / total_bytes as f64 <= REDUNDANCY_THRESHOLD {
               return Ok(());
           }
   
           // Proceed with compaction
           self.compact()?;
           Ok(())
    }

    fn analyze_log_waste(&mut self) -> KvsResult<(u64, u64)> {
        let mut total_bytes = 0;
        let mut active_bytes = 0;

        // Calculate total bytes and active bytes
        for log in self.logs.values_mut() {
            let log_size = log.reader.seek(SeekFrom::End(0))?;
            total_bytes += log_size;
        }

        // Calculate active bytes by checking current index entries
        for disk_info in self.mem_index.values() {
            let log = self.logs.get_mut(&disk_info.log_index)
                .ok_or(KvsErrors::LogNotFound())?;
            
            log.reader.seek(SeekFrom::Start(disk_info.pos))?;
            let mut line = String::new();
            log.reader.read_line(&mut line)?;
            active_bytes += line.len() as u64;
        }

        Ok((total_bytes, total_bytes - active_bytes))
    }

    fn compact(&mut self) -> KvsResult<()> {
        let new_log_id = self.current_log_id + 1;
        let mut new_log = LogFile::new(new_log_id, &self.dir_path)?;
        let mut new_index = HashMap::with_capacity(self.mem_index.len());
        let mut new_pos = 0;

        // Sort entries by log_id to minimize random seeks
        let mut entries: Vec<_> = self.mem_index.iter().collect();
        entries.sort_by_key(|(_, info)| info.log_index);

        // Process entries in batches
        for (key, disk_info) in entries {
            let log = self.logs.get_mut(&disk_info.log_index)
                .ok_or(KvsErrors::LogNotFound())?;
            
            log.reader.seek(SeekFrom::Start(disk_info.pos))?;
            let mut line = String::new();
            log.reader.read_line(&mut line)?;
            
            // Only write active entries
            if let Ok(Command::Set { key: _, value }) = serde_json::from_str(line.trim()) {
                let new_cmd = Command::Set {
                    key: key.clone(),
                    value,
                };
                let serialized = serde_json::to_string(&new_cmd)?;
                writeln!(new_log.writer, "{}", serialized)?;
                
                new_index.insert(key.clone(), DiskInfo {
                    log_index: new_log_id,
                    pos: new_pos,
                });
                
                new_pos += serialized.len() as u64 + 1;
            }
        }

        new_log.writer.flush()?;

        // Clean up old logs
        let old_logs: Vec<_> = self.logs.keys().cloned().collect();
        for log_id in old_logs {
            let path = self.dir_path.join(format!("{}.log", log_id));
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        // Update store state
        self.logs.clear();
        self.logs.insert(new_log_id, new_log);
        self.mem_index = new_index;
        self.current_log_id = new_log_id;

        Ok(())
    }
}

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur in the KvStore.
pub enum KvsErrors {
    /// Error indicating that the specified key was not found.
    #[error("Key not found")]
    KeyNotFound(),

    /// Error indicating serialization or deserialization went wrong
    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),

    /// Error indicating while interacting with I/O (files)
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// Error indicating there was issue parsing into a number
    #[error("Error parsing value {0}")]
    Parse(#[from] std::num::ParseIntError),

    /// Error Log file isn't proper somehow.
    #[error("Invalid Log File")]
    InvalidLogFile(),

    /// Error could not find log based on log_id.
    #[error("Log Not Found")]
    LogNotFound(),

    /// Error address not in IP:PORT format
    #[error("Invalid Address")]
    InvalidAddress(),

    /// Error engine must be 'kvs' or 'sled'
    #[error("Invalid Engine")]
    InvalidEngine(),

    /// Error engine must be 'kvs' or 'sled'
    #[error("FailedConnection")]
    FailedConnection(),

    /// Error indicating network-related issues
    #[error("Network error: {0}")]
    Network(#[from] NetworkErrors), // Add this variant

    /// Error indicating that something went wrong.
    #[error("Something went wrong")]
    GeneralError,
}

#[derive(Error, Debug)]
pub enum NetworkErrors {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    // Add IO error conversion
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    // Add serde_json error conversion
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// A specialized `Result` type for KvStore operations.
pub type KvsResult<T> = Result<T, KvsErrors>;
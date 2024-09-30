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

    /// Error indicating that something went wrong.
    #[error("Something went wrong")]
    GeneralError,
}

/// A specialized `Result` type for KvStore operations.
pub type KvsResult<T> = Result<T, KvsErrors>;

use crate::prelude::parsing::LogParsingError;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub enum SiemError {
    /// Io Error
    Io(String),
    /// Seriaization/Deserialization error
    Serialization(String),
    /// Error parsing a log
    Parsing(LogParsingError),
    /// Error indexing a log
    Indexing(String),
    /// Error accessing the storage system
    Storage(StorageError),
    /// A task execution failed
    Task(String),
    /// A command executed failed
    Command(CommandExecutionError),
    /// A component sufered an error during the startup process
    Configuration(String),
    Messaging(MessagingError),
    Other(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub enum CommandExecutionError {
    Communication(String),
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
#[non_exhaustive]
pub enum StorageError {
    #[error("Not Exists")]
    NotExists,
    #[error("Connection Error")]
    ConnectionError,
    #[error("Already Exists")]
    AlredyExists,
}

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
#[non_exhaustive]
pub enum MessagingError {
    #[error("Disconnected")]
    Disconnected,
    #[error("TimeoutReached")]
    TimeoutReached,
    #[error("Full")]
    Full,
}

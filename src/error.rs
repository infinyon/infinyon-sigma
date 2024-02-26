use crate::prelude::parsing::LogParsingError;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;
// use crossbeam_channel:

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    IoError(io::Error),
    #[error("Invalid configuration provided.")]
    ConfigurationError,
    #[error("{0}")]
    SigmaValueError(String),
    #[error("Regular expression {0} is invalid")]
    SigmaRegularExpressionError(String),
    #[error("Sigma rule identifier must be an UUID")]
    SigmaIdentifierError,
    #[error("Serde Error: {0}")]
    SerdeError(serde_yaml::Error),
    #[error("Could not convert into the specified target ({0}), since it is not supported.")]
    InvalidDestination(String),
    #[error("{0}")]
    GenericError(String),
}

pub type SiemResult<T> = Result<T, SiemError>;

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

#[derive(Serialize, Deserialize, Debug, Clone,Error)]
#[non_exhaustive]
pub enum StorageError {
    #[error("Not Exists")]
    NotExists,
    #[error("Connection Error")]
    ConnectionError,
    #[error("Already Exists")]
    AlredyExists,
}

#[derive(Serialize, Deserialize, Debug, Clone,Error)]
#[non_exhaustive]
pub enum MessagingError {
    #[error("Disconnected")]
    Disconnected,
    #[error("TimeoutReached")]
    TimeoutReached,
    #[error("Full")]
    Full,
}

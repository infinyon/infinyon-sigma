use anyhow::Result;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{error::StorageError, prelude::types::LogString};

pub trait SiemComponentStateStorage: DynClone + Send {
    /// Read a key value from the database
    fn get_value(&self, key: &str) -> Result<String>;
    /// Write to the database a key/value pair
    fn set_value(&mut self, key: LogString, value: String, replace: bool) -> Result<()>;

    /// Get a file
    fn get_file(&self, filepath: String) -> Result<Vec<u8>>;

    /// Get the size of a file
    fn get_file_size(&self, filepath: String) -> Result<u64>;

    /// Get a file part
    fn get_file_range(&self, filepath: String, start: u64, end: u64) -> Result<Vec<u8>>;

    /// Sets the content of a file
    fn set_file(&mut self, filepath: String, content: Vec<u8>) -> Result<()>;

    /// Sets the content of a file
    fn set_file_range(
        &mut self,
        filepath: String,
        content: Vec<u8>,
        start: u64,
        end: u64,
    ) -> Result<()>;

    fn duplicate(&self) -> Box<dyn SiemComponentStateStorage>;
}
clone_trait_object!(SiemComponentStateStorage);

#[derive(Clone)]
pub struct DummyStateStorage {}

impl SiemComponentStateStorage for DummyStateStorage {
    fn get_value(&self, _key: &str) -> Result<String> {
        Err(StorageError::NotExists.into())
    }

    fn set_value(&mut self, _key: LogString, _value: String, _replace: bool) -> Result<()> {
        Ok(())
    }

    fn get_file(&self, _filepath: String) -> Result<Vec<u8>> {
        Err(StorageError::NotExists.into())
    }

    fn get_file_size(&self, _filepath: String) -> Result<u64> {
        Err(StorageError::NotExists.into())
    }

    fn get_file_range(&self, _filepath: String, _start: u64, _end: u64) -> Result<Vec<u8>> {
        Err(StorageError::NotExists.into())
    }

    fn set_file(&mut self, _filepath: String, _content: Vec<u8>) -> Result<()> {
        Ok(())
    }

    fn set_file_range(
        &mut self,
        _filepath: String,
        _content: Vec<u8>,
        _start: u64,
        _end: u64,
    ) -> Result<()> {
        Err(StorageError::NotExists.into())
    }

    fn duplicate(&self) -> Box<dyn SiemComponentStateStorage> {
        Box::new(self.clone())
    }
}

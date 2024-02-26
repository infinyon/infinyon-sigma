use crate::prelude::types::LogString;

use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Serialize, Debug)]
pub enum UpdateTextMapList {
    Add((LogString, Vec<LogString>)),
    Remove(LogString),
    Replace(TextMapListDataset),
}

#[derive(Debug, Clone)]
pub struct TextMapListSynDataset {
    dataset: Arc<TextMapListDataset>,
}

#[derive(Serialize, Debug)]
pub struct TextMapListDataset {
    data: BTreeMap<LogString, Vec<LogString>>,
}

impl TextMapListDataset {
    pub fn new() -> TextMapListDataset {
        return TextMapListDataset {
            data: BTreeMap::new(),
        };
    }
    pub fn insert(&mut self, key: LogString, data: Vec<LogString>) {
        self.data.insert(key, data);
    }
    pub fn get(&self, key: &str) -> Option<&Vec<LogString>> {
        self.data.get(key)
    }
    pub fn internal_ref(&self) -> &BTreeMap<LogString, Vec<LogString>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_data_in_dataset() {
        let mut dataset = TextMapListDataset::new();
        dataset.insert(
            LogString::Borrowed("192.168.1.1"),
            vec![
                LogString::Borrowed("Local IP "),
                LogString::Borrowed("Remote IP"),
            ],
        );
        assert_eq!(
            dataset.get("192.168.1.1"),
            Some(
                &(vec![
                    LogString::Borrowed("Local IP "),
                    LogString::Borrowed("Remote IP")
                ])
            )
        );
    }
}

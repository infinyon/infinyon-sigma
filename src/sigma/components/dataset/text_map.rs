use crate::prelude::types::LogString;

use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
#[derive(Serialize, Debug)]
pub enum UpdateTextMap {
    Add((LogString, LogString)),
    Remove(LogString),
    Replace(TextMapDataset),
}


#[derive(Debug, Clone)]
pub struct TextMapSynDataset {
    dataset: Arc<TextMapDataset>
}
#[derive(Serialize, Debug)]
pub struct TextMapDataset {
    data: BTreeMap<LogString, LogString>,
}

impl TextMapDataset {
    pub fn new() -> TextMapDataset {
        return TextMapDataset {
            data: BTreeMap::new(),
        };
    }
    pub fn insert<S>(&mut self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        self.data.insert(key.into(), data.into());
    }
    pub fn get(&self, key: &str) -> Option<&LogString> {
        self.data.get(key)
    }
    pub fn internal_ref(&self) -> &BTreeMap<LogString, LogString> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_data_in_map() {
        let mut dataset = TextMapDataset::new();
        dataset.insert(
            LogString::Borrowed("192.168.1.1"),
            LogString::Borrowed("Local IP"),
        );
        assert_eq!(
            dataset.get("192.168.1.1"),
            Some(&LogString::Borrowed("Local IP"))
        );
    }
}

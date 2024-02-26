use crate::prelude::types::LogString;

use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Serialize, Debug)]
pub enum UpdateTextSet {
    Add(LogString),
    Remove(LogString),
    Replace(TextSetDataset),
}
#[derive(Debug, Clone)]
pub struct TextSetSynDataset {
    dataset: Arc<TextSetDataset>,
}

#[derive(Serialize, Debug)]
pub struct TextSetDataset {
    data: BTreeSet<LogString>,
}

impl TextSetDataset {
    pub fn new() -> TextSetDataset {
        return TextSetDataset {
            data: BTreeSet::new(),
        };
    }
    pub fn insert<S>(&mut self, val: S)
    where
        S: Into<LogString>,
    {
        self.data.insert(val.into());
    }
    pub fn contains(&self, val: &LogString) -> bool {
        self.data.contains(val)
    }
    pub fn internal_ref(&self) -> &BTreeSet<LogString> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_be_in_set() {
        let mut dataset = TextSetDataset::new();
        dataset.insert(LogString::Borrowed("192.168.1.1"));
        assert_eq!(dataset.contains(&LogString::Borrowed("192.168.1.1")), true);
    }
}

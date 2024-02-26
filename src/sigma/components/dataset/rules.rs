use crate::prelude::rule::SiemRule;
use crate::prelude::types::LogString;

use serde::Serialize;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub enum UpdateRules {
    Add(SiemRule),
    Remove(LogString),
    Replace(RulesDataset),
}

#[derive(Serialize, Debug)]
pub struct RulesDataset {
    rules: BTreeMap<LogString, SiemRule>,
}

impl RulesDataset {
    pub fn new() -> RulesDataset {
        return RulesDataset {
            rules: BTreeMap::new(),
        };
    }
    pub fn insert(&mut self, rule: SiemRule) {
        self.rules.insert(rule.name.clone(), rule);
    }
    pub fn get(&self, id: &LogString) -> Option<&SiemRule> {
        self.rules.get(id)
    }
}

fn extract_rule_from_update(update: UpdateRules) -> SiemRule {
    match update {
        UpdateRules::Add(r) => r,
        UpdateRules::Remove(_) => unreachable!(),
        UpdateRules::Replace(_) => unreachable!(),
    }
}

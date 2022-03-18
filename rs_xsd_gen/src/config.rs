use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

/// controls enum generation for base types used as elements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum BaseTypeConfig {
    /// automatically discover the base types and use all of them
    Auto,
    /// automatically discover the base types but only use the ones from this whitelist
    Whitelist(HashSet<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) base_type_whitelist: HashMap<String, BaseTypeConfig>,
}

impl Config {
    pub(crate) fn generate_base_type(&self, base_name: &str, child_name: &str) -> bool {
        match self.base_type_whitelist.get(base_name) {
            None => {
                panic!("base type {} requires a configuration entry", base_name);
            }
            Some(x) => match x {
                BaseTypeConfig::Auto => true,
                BaseTypeConfig::Whitelist(x) => x.contains(child_name),
            },
        }
    }
}

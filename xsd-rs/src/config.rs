use std::collections::{HashMap, HashSet};

use serde::Deserialize;

/// controls enum generation for base types used as elements
#[derive(Clone, Debug, Deserialize)]
pub(crate) enum BaseTypeEntry {
    /// automatically discover the base types and use all of them
    Auto,
    /// automatically discover the base types but only use the ones from this whitelist
    Whitelist(HashSet<String>),
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct BaseTypeConfig {
    pub(crate) whitelist: HashMap<String, BaseTypeEntry>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) base_types: BaseTypeConfig,
    pub(crate) mappings: xsd_model::config::Config,
}

impl BaseTypeConfig {
    pub(crate) fn generate_base_type(&self, base_name: &str, child_name: &str) -> bool {
        match self.whitelist.get(base_name) {
            None => {
                panic!("base type {} requires a configuration entry", base_name);
            }
            Some(x) => match x {
                BaseTypeEntry::Auto => true,
                BaseTypeEntry::Whitelist(x) => x.contains(child_name),
            },
        }
    }
}

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    /// name of the variant
    pub name: String,
    /// optional comment
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumericEnum<T>
where
    T: Copy + Ord,
{
    /// name of the enum
    pub name: String,
    /// map from value -> name
    pub variants: BTreeMap<T, Variant>,
}

/// Mappings not provided natively
#[derive(Debug, Serialize, Deserialize)]
pub enum Mapping {
    NumericEnum(std::rc::Rc<NumericEnum<u8>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// maps an existing Type -> to an implementation dependent mapping
    pub mappings: HashMap<String, Mapping>,
}

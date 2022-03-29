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
    /// map from value -> name
    pub variants: BTreeMap<T, Variant>,
}

/// available type mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeMapping {
    /// map an existing u8 type to an enum
    EnumU8(std::rc::Rc<NumericEnum<u8>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// maps an existing Type -> to an implementation dependent mapping
    pub mappings: HashMap<String, TypeMapping>,
}

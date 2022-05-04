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

/// represents a newtype around an array
///
/// struct <name> {
///    inner: [u8, <size>]
/// }
///
/// The buffer is always de(serialized) from hex bytes
///
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedArray {
    /// size of the fixed array in bytes
    pub size: u8,
    /// name of the outer struct type
    pub name: String,
}

/// Mappings not provided natively
#[derive(Debug, Serialize, Deserialize)]
pub enum SubstitutedType {
    /// fixed size array of bytes
    NamedArray(std::rc::Rc<NamedArray>),
    /// enumeration w/ numeric representation
    NumericEnum(std::rc::Rc<NumericEnum<u8>>),
}

/// identifies a particular attribute or element within a struct
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct FieldId {
    pub parent_name: String,
    pub field_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// map of substituted types which may be mapped from and XSD type or a FieldId
    pub types: HashMap<String, SubstitutedType>,
    /// maps an existing XSD Type to substituted type
    pub type_mappings: HashMap<String, String>,
    /// map particular fields to a substituted type
    pub field_mappings: HashMap<FieldId, String>,
}

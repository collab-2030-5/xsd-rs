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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstitutedType {
    /// fixed size array of bytes
    NamedArray(std::rc::Rc<NamedArray>),
    /// enumeration w/ numeric representation
    NumericEnum(std::rc::Rc<NumericEnum<u8>>),
}

/// identifies a particular attribute or element within a struct
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct FieldKey {
    pub struct_name: String,
    pub field_name: String,
}

/// identifies a particular attribute or element within a struct
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldMapping {
    pub field_key: FieldKey,
    pub substituted: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// map of substituted types which may be mapped from and XSD type or a FieldId
    pub types: HashMap<String, SubstitutedType>,
    /// maps an existing XSD Type to substituted type
    pub type_mappings: HashMap<String, String>,
    /// map particular fields to a substituted type
    pub field_mappings: Vec<FieldMapping>,
}

impl Config {
    pub fn resolve(self) -> ResolvedConfig {
        let mut type_mappings: HashMap<String, SubstitutedType> = Default::default();
        let mut field_mappings: HashMap<FieldKey, SubstitutedType> = Default::default();

        for (key, t) in self.type_mappings {
            let resolved = self.types.get(&t).expect("Unknown type");
            type_mappings.insert(key, resolved.clone());
        }

        for fm in self.field_mappings {
            let resolved = self.types.get(&fm.substituted).expect("Unknown type");
            field_mappings.insert(fm.field_key, resolved.clone());
        }

        ResolvedConfig {
            type_mappings,
            field_mappings,
        }
    }
}

/// resolved version of the configuration
pub struct ResolvedConfig {
    pub type_mappings: HashMap<String, SubstitutedType>,
    pub field_mappings: HashMap<FieldKey, SubstitutedType>,
}

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::resolved::AnyType;
use crate::{PrimitiveType, SimpleType, TypeId, WrapperType};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum DurationEncoding {
    UInt32,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum NumericDuration {
    Seconds(DurationEncoding),
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    /// name of the variant
    pub name: String,
    /// optional comment
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct NamedArray {
    /// size of the fixed array in bytes
    pub size: u8,
    /// name of the outer struct type
    pub name: String,
}

/// A particular bit within a byte
#[derive(Debug, Deserialize)]
pub struct Bit {
    pub name: String,
    pub comment: String,
}

/// Bits within a byte
#[derive(Debug, Deserialize)]
pub struct Byte {
    pub bit_0: Option<Bit>,
    pub bit_1: Option<Bit>,
    pub bit_2: Option<Bit>,
    pub bit_3: Option<Bit>,
    pub bit_4: Option<Bit>,
    pub bit_5: Option<Bit>,
    pub bit_6: Option<Bit>,
    pub bit_7: Option<Bit>,
}

impl Byte {
    pub fn iter(&self) -> impl Iterator<Item = (u8, &Bit)> {
        ByteIterator { pos: 0, byte: self }
    }
}

pub struct ByteIterator<'a> {
    pos: usize,
    byte: &'a Byte,
}

impl<'a> Iterator for ByteIterator<'a> {
    type Item = (u8, &'a Bit);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos > 7 {
                return None;
            }
            let current = self.pos;
            self.pos += 1;
            let mask: u8 = 1 << current;
            match current {
                0 => {
                    if let Some(x) = &self.byte.bit_0 {
                        return Some((mask, x));
                    }
                }
                1 => {
                    if let Some(x) = &self.byte.bit_1 {
                        return Some((mask, x));
                    }
                }
                2 => {
                    if let Some(x) = &self.byte.bit_2 {
                        return Some((mask, x));
                    }
                }
                3 => {
                    if let Some(x) = &self.byte.bit_3 {
                        return Some((mask, x));
                    }
                }
                4 => {
                    if let Some(x) = &self.byte.bit_4 {
                        return Some((mask, x));
                    }
                }
                5 => {
                    if let Some(x) = &self.byte.bit_5 {
                        return Some((mask, x));
                    }
                }
                6 => {
                    if let Some(x) = &self.byte.bit_6 {
                        return Some((mask, x));
                    }
                }
                7 => {
                    if let Some(x) = &self.byte.bit_7 {
                        return Some((mask, x));
                    }
                }
                _ => return None,
            }
        }
    }
}

/// A bit-field represented as an array where each bit has a meaning
#[derive(Debug, Deserialize)]
pub struct BitField {
    /// Name of the Rust struct
    pub name: String,
    /// comment on the Rust struct
    pub comment: Option<String>,
    /// an array of bytes where each
    pub bytes: Vec<Byte>,
}

/// Mappings not provided natively in XSD
#[derive(Debug, Clone, Deserialize)]
pub enum SubstitutedTypeVariant {
    /// fixed size array of bytes
    NamedArray(std::rc::Rc<NamedArray>),
    /// enumeration w/ numeric representation
    NumericEnum(std::rc::Rc<NumericEnum<u8>>),
    /// Bitfield represented by xs:hexBinary
    HexBitField(std::rc::Rc<BitField>),
    /// Duration encoding as a numeric value
    NumericDuration(NumericDuration),
}

/// Mappings not provided natively in XSD
#[derive(Debug, Clone, Deserialize)]
pub struct Substitution {
    target: TypeId,
    variant: SubstitutedTypeVariant,
}

impl From<Substitution> for SimpleType {
    fn from(t: Substitution) -> Self {
        match t.variant {
            SubstitutedTypeVariant::NamedArray(x) => WrapperType::NamedArray(t.target, x).into(),
            SubstitutedTypeVariant::NumericEnum(x) => WrapperType::EnumU8(t.target, x).into(),
            SubstitutedTypeVariant::HexBitField(x) => WrapperType::HexBitField(t.target, x).into(),
            SubstitutedTypeVariant::NumericDuration(x) => PrimitiveType::NumericDuration(x).into(),
        }
    }
}

impl Substitution {
    /*
    pub(crate) fn new(target: TypeId, variant: SubstitutedTypeVariant) -> Self {
        Self { target, variant }
    }
     */

    pub fn type_name(&self) -> Option<&str> {
        match &self.variant {
            SubstitutedTypeVariant::NamedArray(x) => Some(&x.name),
            SubstitutedTypeVariant::NumericEnum(x) => Some(&x.name),
            SubstitutedTypeVariant::HexBitField(x) => Some(&x.name),
            SubstitutedTypeVariant::NumericDuration(_) => None,
        }
    }
}

impl From<Substitution> for AnyType {
    fn from(x: Substitution) -> Self {
        Self::Simple(x.into())
    }
}

/// identifies a particular attribute or element within a struct
#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct FieldId {
    /// TypeId of the struct that contains the field
    pub parent_id: TypeId,
    /// Name of the field within the struct
    pub field_name: String,
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}'->{}", self.parent_id, self.field_name)
    }
}

/// identifies a particular attribute or element within a struct
#[derive(Debug, Deserialize)]
pub struct FieldMapping {
    /// id of the field
    pub field_id: FieldId,
    /// type to be substituted
    pub substituted: TypeId,
}

#[derive(Debug, Deserialize)]
pub struct MappingConfig {
    /// map of substituted types which may be mapped from and XSD type or a FieldId
    pub types: HashMap<TypeId, Substitution>,
    /// maps an existing XSD Type to substituted type
    pub type_mappings: HashMap<TypeId, TypeId>,
    /// map particular fields to a substituted type
    pub field_mappings: Vec<FieldMapping>,
}

impl MappingConfig {
    pub fn resolve(self) -> ResolvedConfig {
        let mut type_mappings: HashMap<TypeId, Substitution> = Default::default();
        let mut field_mappings: HashMap<FieldId, Substitution> = Default::default();

        for (key, t) in self.type_mappings {
            let resolved = self.types.get(&t).expect("Unknown type");
            type_mappings.insert(key, resolved.clone());
        }

        for fm in self.field_mappings {
            let resolved = self.types.get(&fm.substituted).expect("Unknown type");
            field_mappings.insert(fm.field_id, resolved.clone());
        }

        ResolvedConfig {
            type_mappings,
            field_mappings,
        }
    }
}

/// controls enum generation for base types used as elements
#[derive(Clone, Debug, Deserialize)]
pub enum BaseTypeEntry {
    /// automatically discover the base types and use all of them
    Auto,
    /// automatically discover the base types but only use the ones from this whitelist
    Whitelist(HashSet<String>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseTypeConfig {
    pub(crate) whitelist: HashMap<String, BaseTypeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_types: BaseTypeConfig,
    pub mappings: MappingConfig,
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

/// resolved version of the configuration
#[derive(Debug)]
pub struct ResolvedConfig {
    pub type_mappings: HashMap<TypeId, Substitution>,
    pub field_mappings: HashMap<FieldId, Substitution>,
}

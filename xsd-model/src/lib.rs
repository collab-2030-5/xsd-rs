extern crate core;

/// configuration types that can be read/written from JSON
pub mod config;
/// map types used in the models
pub mod map;
/// resolved version of the xsd-model suitable for code-gen
pub mod resolved;
/// types used resolving (TypeId -> AnyType)
pub mod resolver;
/// unresolved representation of the xsd-model
pub mod unresolved;

/// raw XSD parser
pub(crate) mod parser;

/// parse the XSD into its raw format but only return the impl Debug so it can be logged or printed
pub fn parse_xsd(xsd: &str) -> impl std::fmt::Debug + '_ {
    crate::parser::parse(xsd).unwrap()
}

use crate::config::NumericDuration;
use crate::parser::types::Facet;
use crate::parser::xsd_elements::FacetType;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct TypeId {
    /// Shorthand name for the namespace where the type resides
    pub ns: String,
    /// The name of the type without the namespace
    pub name: String,
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ns, self.name)
    }
}

impl TypeId {
    pub fn parse(type_name: &str, fallback_ns: &str) -> Self {
        match type_name.split_once(':') {
            None => Self {
                ns: fallback_ns.to_owned(),
                name: type_name.to_owned(),
            },
            Some((ns, name)) => Self {
                ns: ns.to_owned(),
                name: name.to_owned(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub name: Option<String>,
    pub uri: String,
}

#[derive(Copy, Clone, Debug)]
pub enum HexByteConstraints {
    /// Constrained to a single byte
    Single,
    /// Possibly constrained to 2 or more bytes
    Bytes { max: Option<usize> },
}

impl Default for HexByteConstraints {
    fn default() -> Self {
        Self::Bytes { max: None }
    }
}

impl HexByteConstraints {
    fn apply_facet(self, facet_type: &FacetType) -> HexByteConstraints {
        match facet_type {
            FacetType::MaxLength(x) => {
                let max: usize = x.parse().unwrap();
                match self {
                    HexByteConstraints::Single => panic!(
                        "Already constrained to a single byte, cannot constrain to {}",
                        max
                    ),
                    HexByteConstraints::Bytes { max: Some(x) } => {
                        panic!(
                            "Already constrained to a maximum of {} bytes, cannot constrain to {}",
                            x, max
                        );
                    }
                    HexByteConstraints::Bytes { max: None } => match max {
                        0 => panic!("maximum length cannot be zero"),
                        1 => Self::Single,
                        _ => Self::Bytes { max: Some(max) },
                    },
                }
            }
            _ => panic!("Unexpected Facet type for xs:hexBinary: {:?}", facet_type),
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct StringConstraints {
    pub max_length: Option<usize>,
}

impl StringConstraints {
    fn apply_facet(mut self, facet_type: &FacetType) -> Self {
        match facet_type {
            FacetType::MaxLength(x) => {
                let max: usize = x.parse().unwrap();
                self.set_max_length(max);
                self
            }
            FacetType::Enumeration(_) => {
                tracing::warn!("ignoring {:?} for string type", facet_type);
                self
            }
            FacetType::Pattern(_) => {
                tracing::warn!("ignoring {:?} for string type", facet_type);
                self
            }
            _ => panic!("unsupported facet for string type: {:?}", facet_type),
        }
    }

    fn set_max_length(&mut self, max: usize) {
        match self.max_length {
            None => self.max_length = Some(max),
            Some(x) => {
                panic!("Max length already set to {}, cannot set to {}", x, max);
            }
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct NumericConstraint<T>
where
    T: Display + std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T> NumericConstraint<T>
where
    T: Display + std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    pub(crate) fn apply_facet(mut self, facet: &FacetType) -> Self {
        match facet {
            FacetType::MaxInclusive(s) => {
                let max: T = s.parse().unwrap();
                self.set_max(max);
                self
            }
            FacetType::MinInclusive(s) => {
                let min: T = s.parse().unwrap();
                self.set_min(min);
                self
            }
            x => panic!("Unsupported {} facet: {:?}", std::any::type_name::<T>(), x),
        }
    }

    fn set_min(&mut self, min: T) {
        match &self.min {
            None => self.min = Some(min),
            Some(x) => panic!("Min already set to {}, cannot set to {}", x, min),
        }
    }

    fn set_max(&mut self, max: T) {
        match &self.max {
            None => self.min = Some(max),
            Some(x) => panic!("Max already set to {}, cannot set to {}", x, max),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub type_id: TypeId,
    pub comment: Option<String>,
    pub variants: Vec<NamedEnumVariant>,
}

#[derive(Clone, Debug)]
pub struct NamedEnumVariant {
    /// name of the variant in the generated Rust
    pub name: String,
    /// string value that maps to this variant in the XML itself
    pub value: String,
    pub comment: Option<String>,
}

/// numeric types built into XSD that we support
#[derive(Clone, Debug)]
pub enum NumericType {
    I8(NumericConstraint<i8>),
    U8(NumericConstraint<u8>),
    I16(NumericConstraint<i16>),
    U16(NumericConstraint<u16>),
    I32(NumericConstraint<i32>),
    U32(NumericConstraint<u32>),
    I64(NumericConstraint<i64>),
    U64(NumericConstraint<u64>),
    F32(NumericConstraint<f32>),
    F64(NumericConstraint<f64>),
}

impl NumericType {
    fn apply_facet(self, facet: &FacetType) -> Self {
        match self {
            NumericType::I8(x) => NumericType::I8(x.apply_facet(facet)),
            NumericType::U8(x) => NumericType::U8(x.apply_facet(facet)),
            NumericType::I16(x) => NumericType::I16(x.apply_facet(facet)),
            NumericType::U16(x) => NumericType::U16(x.apply_facet(facet)),
            NumericType::I32(x) => NumericType::I32(x.apply_facet(facet)),
            NumericType::U32(x) => NumericType::U32(x.apply_facet(facet)),
            NumericType::I64(x) => NumericType::I64(x.apply_facet(facet)),
            NumericType::U64(x) => NumericType::U64(x.apply_facet(facet)),
            NumericType::F32(x) => NumericType::F32(x.apply_facet(facet)),
            NumericType::F64(x) => NumericType::F64(x.apply_facet(facet)),
        }
    }
}

impl From<NumericType> for PrimitiveType {
    fn from(x: NumericType) -> Self {
        Self::Number(x)
    }
}

impl From<NumericType> for SimpleType {
    fn from(x: NumericType) -> Self {
        Self::Primitive(x.into())
    }
}

/// Simple types that don't require any code generation, they just map directly to a Rust type
#[derive(Clone, Debug)]
pub enum PrimitiveType {
    /// true or false
    Boolean,
    /// multiple bytes with a possible maximum length
    HexBytes(HexByteConstraints),
    /// string with constraints
    String(StringConstraints),
    /// numeric types
    Number(NumericType),
    /// Duration encoded as a number
    NumericDuration(NumericDuration),
}

impl PrimitiveType {
    pub(crate) fn try_resolve_xs_type(id: &TypeId) -> Option<PrimitiveType> {
        if id.ns != "xs" {
            return None;
        }

        Some(Self::resolve_xs_type_suffix(&id.name))
    }

    pub(crate) fn apply_facets(self, facets: &[Facet]) -> PrimitiveType {
        facets
            .iter()
            .fold(self, |sum, facet| sum.apply_facet(&facet.facet_type))
    }

    /// apply facets to the primitive, constraining it's allowed values and possibly changing its type
    pub(crate) fn apply_facet(self, facet: &FacetType) -> PrimitiveType {
        match self {
            PrimitiveType::HexBytes(x) => PrimitiveType::HexBytes(x.apply_facet(facet)),
            PrimitiveType::String(x) => PrimitiveType::String(x.apply_facet(facet)),
            PrimitiveType::Number(x) => PrimitiveType::Number(x.apply_facet(facet)),
            PrimitiveType::Boolean => {
                panic!("Facets not supported for boolean type: {:?}", facet);
            }
            PrimitiveType::NumericDuration(_) => {
                panic!(
                    "Facets not supported for numeric duration type: {:?}",
                    facet
                );
            }
        }
    }

    fn resolve_xs_type_suffix(suffix: &str) -> PrimitiveType {
        match suffix {
            "token" | "string" | "normalizedString" | "anyURI" => {
                PrimitiveType::String(StringConstraints::default())
            }
            "base64Binary" => PrimitiveType::String(StringConstraints::default()),
            "hexBinary" => PrimitiveType::HexBytes(HexByteConstraints::default()),
            "dateTime" => PrimitiveType::String(StringConstraints::default()),
            "byte" => NumericType::I8(NumericConstraint::default()).into(),
            "unsignedByte" => NumericType::U8(NumericConstraint::default()).into(),
            "short" => NumericType::I16(NumericConstraint::default()).into(),
            "unsignedShort" => NumericType::U16(NumericConstraint::default()).into(),
            "int" => NumericType::I32(NumericConstraint::default()).into(),
            "unsignedInt" => NumericType::U32(NumericConstraint::default()).into(),
            "long" => NumericType::I64(NumericConstraint::default()).into(),
            "unsignedLong" => NumericType::U64(NumericConstraint::default()).into(),
            "float" => NumericType::F32(NumericConstraint::default()).into(),
            "double" => NumericType::F64(NumericConstraint::default()).into(),
            _ => panic!("Unknown 'xs' type: {}", suffix),
        }
    }
}

impl From<PrimitiveType> for SimpleType {
    fn from(x: PrimitiveType) -> Self {
        SimpleType::Primitive(x)
    }
}

/// Wrapper types are simple as far as the XML/XSD go but they require code generation
/// in the form of a wrapper object which is more complex than what's allowed in the XSD
#[derive(Clone, Debug)]
pub enum WrapperType {
    /// string enum type built into XSD
    Enum(std::rc::Rc<Enumeration>),
    /// numeric enum type
    EnumU8(TypeId, std::rc::Rc<config::NumericEnum<u8>>),
    /// Fixed size number of bytes mapped to a fixed array
    NamedArray(TypeId, std::rc::Rc<config::NamedArray>),
    /// Bitfield represented as Hex-bytes
    HexBitField(TypeId, std::rc::Rc<config::BitField>),
}

impl WrapperType {
    pub fn type_id(&self) -> &TypeId {
        match self {
            WrapperType::Enum(x) => &x.type_id,
            WrapperType::EnumU8(id, _) => id,
            WrapperType::NamedArray(id, _) => id,
            WrapperType::HexBitField(id, _) => id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            WrapperType::Enum(x) => &x.type_id.name,
            WrapperType::EnumU8(_, x) => &x.name,
            WrapperType::NamedArray(_, x) => &x.name,
            WrapperType::HexBitField(_, x) => &x.name,
        }
    }
}

impl From<WrapperType> for SimpleType {
    fn from(x: WrapperType) -> Self {
        SimpleType::Wrapper(x)
    }
}

/// Types that can be represented with a single string value
/// These types are suitable for use in an attribute or element
#[derive(Clone, Debug)]
pub enum SimpleType {
    /// Types that DON'T require code generation
    Primitive(PrimitiveType),
    /// Type that REQUIRE code generation
    Wrapper(WrapperType),
}

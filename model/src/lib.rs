extern crate core;

/// configuration types that can be read/written from JSON
pub mod config;
/// map types used in the models
pub mod map;
/// resolved version of the model suitable for code-gen
pub mod resolved;
/// types used resolving (TypeId -> AnyType)
pub mod resolver;
/// unresolved representation of the model
pub mod unresolved;

/// raw XSD parser
pub mod parse;

use crate::config::NumericDuration;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

#[derive(Default, Copy, Clone, Debug)]
pub struct StringConstraints {
    pub max_length: Option<usize>,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct NumericConstraint<T> {
    pub min: Option<T>,
    pub max: Option<T>,
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

impl From<NumericType> for SimpleType {
    fn from(x: NumericType) -> Self {
        Self::Primitive(PrimitiveType::Number(x))
    }
}

/// Simple types that don't require any code generation, they just map directly to a Rust type
#[derive(Clone, Debug)]
pub enum PrimitiveType {
    /// true or false
    Boolean,
    /// single byte encoded as a hex (2 characters e.g. "FF"), underlying type is xs:string
    HexByte,
    /// multiple bytes with a maximum length, underlying type is xs:string with constraints
    HexBytes(Option<usize>),
    /// string with constraints
    String(StringConstraints),
    /// numeric types
    Number(NumericType),
    /// Duration encoded as a number
    NumericDuration(NumericDuration),
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
    EnumU8(std::rc::Rc<config::NumericEnum<u8>>),
    /// Fixed size number of bytes mapped to a fixed array
    NamedArray(std::rc::Rc<config::NamedArray>),
    /// Bitfield represented as Hex-bytes
    HexBitField(std::rc::Rc<config::BitField>),
}

impl WrapperType {
    pub fn name(&self) -> &str {
        match self {
            WrapperType::Enum(x) => x.type_id.name.as_str(),
            WrapperType::EnumU8(x) => x.name.as_str(),
            WrapperType::NamedArray(x) => x.name.as_str(),
            WrapperType::HexBitField(x) => x.name.as_str(),
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

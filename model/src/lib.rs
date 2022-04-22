/// configuration types that can be read/written from JSON
pub mod config;
/// resolved version of the model suitable for code-gen
pub mod resolved;
/// raw JSON representation of the model
pub mod unresolved;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub name: Option<String>,
    pub uri: String,
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct StringConstraint {
    pub max_length: Option<usize>,
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct NumericConstraint<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

/// maps to simple types with possible constraints
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SimpleType {
    /// true or false
    Boolean,
    /// a single byte encoded as a hex (2 characters e.g. "FF")
    HexByte,
    /// multiple bytes with a maximum length
    HexBytes(Option<usize>),
    String(StringConstraint),
    I8(NumericConstraint<i8>),
    U8(NumericConstraint<u8>),
    I16(NumericConstraint<i16>),
    U16(NumericConstraint<u16>),
    I32(NumericConstraint<i32>),
    U32(NumericConstraint<u32>),
    I64(NumericConstraint<i64>),
    U64(NumericConstraint<u64>),
    /// numeric enum type
    EnumU8(std::rc::Rc<config::NumericEnum<u8>>),
    /// Fixed size number of bytes mapped to a fixed array
    NamedArray(std::rc::Rc<config::NamedArray>),
}

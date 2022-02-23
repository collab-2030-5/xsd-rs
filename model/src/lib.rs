use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub name: Option<String>,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub xsd_ns: Option<Namespace>,
    pub target_ns: Option<Namespace>,
    pub simple_types: HashMap<String, SimpleType>,
    pub structs: Vec<Struct>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variant {
    pub comment: Option<String>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enumeration<T>
where
    T: Copy + Clone + Eq + Hash,
{
    pub variants: HashMap<T, String>,
}

// maps to simple types with possible constraints
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SimpleType {
    /// alias for another simple type
    Alias(String),
    /// a single byte encoded as a hex (2 characters e.g. "FF")
    HexByte,
    /// multiple bytes with a maximum length
    HexBytes(usize),
    /// number -> enum
    Enum(Enumeration<u8>),
    String(StringConstraint),
    I8(NumericConstraint<i8>),
    U8(NumericConstraint<u8>),
    I16(NumericConstraint<i16>),
    U16(NumericConstraint<u16>),
    I32(NumericConstraint<i32>),
    U32(NumericConstraint<u32>),
    I64(NumericConstraint<i64>),
    U64(NumericConstraint<u64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: String,
    pub info: FieldTypeInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ElementType {
    Single,
    Array,
    Option,
    Error(String),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum AttributeType {
    Single,
    Option,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FieldTypeInfo {
    Attribute(AttributeType),
    Element(ElementType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub comment: Option<String>,
    pub name: String,
    /// single optional base struct
    pub base_type: Option<String>,
    pub fields: Vec<StructField>,
}

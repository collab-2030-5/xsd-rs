use std::collections::HashMap;

#[derive(Debug)]
pub struct Model {
    pub simple_types: HashMap<String, SimpleType>,
    pub structs: Vec<Struct>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct StringConstraint {
    pub max_length: Option<usize>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct NumericConstraint<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

// maps to simple types with possible constraints
#[derive(Copy, Clone, Debug)]
pub enum SimpleType {
    // a single byte encoded as a hex (2 characters e.g. "FF")
    HexByte,
    // multiple bytes with a maximum length
    HexBytes(usize),
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

#[derive(Debug)]
pub struct StructField {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: String,
    pub info: FieldTypeInfo,
}

#[derive(Copy, Clone, Debug)]
pub enum ElementType {
    Single,
    Array,
    Option,
}

#[derive(Copy, Clone, Debug)]
pub enum AttributeType {
    Single,
    Option,
}

#[derive(Clone, Debug)]
pub enum FieldTypeInfo {
    Attribute(AttributeType),
    Element(ElementType),
}

#[derive(Debug)]
pub struct Struct {
    pub comment: Option<String>,
    pub name: String,
    /// single optional base struct
    pub base_type: Option<String>,
    pub fields: Vec<StructField>,
}

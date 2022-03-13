use std::collections::HashMap;

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub name: Option<String>,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedModel {
    pub xsd_ns: Option<Namespace>,
    pub target_ns: Option<Namespace>,
    pub simple_types: HashMap<String, UnresolvedSimpleType>,
    pub structs: Vec<UnresolvedStruct>,
}

// maps to simple types with possible constraints
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnresolvedSimpleType {
    /// alias for another simple type
    Unresolved(String),
    Resolved(SimpleType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedField {
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
pub struct UnresolvedStruct {
    pub comment: Option<String>,
    pub name: String,
    /// single optional base struct
    pub base_type: Option<String>,
    pub fields: Vec<UnresolvedField>,
}

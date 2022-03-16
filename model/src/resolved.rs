use crate::{Namespace, SimpleType};

use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum ElementType {
    Simple(SimpleType),
    Struct(std::rc::Rc<Struct>),
}

impl From<SimpleType> for ElementType {
    fn from(x: SimpleType) -> Self {
        ElementType::Simple(x)
    }
}

impl From<Rc<Struct>> for ElementType {
    fn from(x: Rc<Struct>) -> Self {
        ElementType::Struct(x)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ElemMultiplicity {
    Single,
    Optional,
    Vec,
}

#[derive(Copy, Clone, Debug)]
pub enum AttrMultiplicity {
    Single,
    Optional,
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Element(ElemMultiplicity, ElementType),
    Attribute(AttrMultiplicity, SimpleType),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub comment: Option<String>,
    pub name: String,
    pub base_type: Option<Rc<Struct>>,
    pub fields: Vec<Field>,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct Model {
    pub target_ns: Option<Namespace>,
    pub structs: Vec<Rc<Struct>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    /// true if the struct is inherited from by another struct
    pub is_base: bool,
    /// true if the struct is used as an element in another struct
    pub use_as_element: bool,
}

impl Model {
    pub fn base_fields(&self) -> impl Iterator<Item = Rc<Struct>> + '_ {
        self.structs
            .iter()
            .filter(|x| x.metadata.is_base && x.metadata.use_as_element)
            .cloned()
    }
}

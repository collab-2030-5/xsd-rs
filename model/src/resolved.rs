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

impl Struct {
    /// test if this struct inherits from a base struct, directly or indirectly
    pub fn inherits_from(&self, base: &Rc<Struct>) -> bool {
        if let Some(child) = &self.base_type {
            if Rc::ptr_eq(child, base) {
                true
            } else {
                child.inherits_from(base)
            }
        } else {
            false
        }
    }
}

impl Model {
    /// return all of the structs that are 1) base structs AND 2) used as fields in other structs
    pub fn base_fields(&self) -> Vec<Rc<Struct>> {
        self.structs
            .iter()
            .filter(|x| x.metadata.is_base && x.metadata.use_as_element)
            .cloned()
            .collect()
    }

    /// All structs that inherit from a base struct, directly or indirectly
    pub fn sub_structs_of(&self, base: &Rc<Struct>) -> Vec<Rc<Struct>> {
        let mut structs = Vec::new();

        for other in self.structs.iter() {
            if other.inherits_from(base) && !structs.iter().any(|x| Rc::ptr_eq(x, other)) {
                structs.push(other.clone());
            }
        }

        structs
    }
}

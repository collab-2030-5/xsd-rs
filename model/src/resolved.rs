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
}

#[derive(Debug)]
pub struct Model {
    pub target_ns: Option<Namespace>,
    pub structs: Vec<Rc<Struct>>,
}

impl Model {
    /// true if this struct is inherited from by any other struct in the model
    pub fn is_base(&self, st: &Rc<Struct>) -> bool {
        for other in self.structs.iter() {
            if let Some(other) = &other.base_type {
                if Rc::ptr_eq(other, st) {
                    return true;
                }
            }
        }
        false
    }

    /// true if this struct is used as a field in any other struct
    pub fn used_as_field(&self, st: &Rc<Struct>) -> bool {
        for other in self.structs.iter() {
            for field in other.fields.iter() {
                if let FieldType::Element(_, ElementType::Struct(other)) = &field.field_type {
                    if Rc::ptr_eq(st, other) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// find all the base types in the model that are used as fields in other structs
    pub fn base_fields(&self) -> Vec<Rc<Struct>> {
        let mut fields = Vec::new();
        for st in self.structs.iter() {
            if self.is_base(&st) && self.used_as_field(&st) {
                // add it to the fields list
                if !fields.iter().any(|x| Rc::ptr_eq(x, st)) {
                    fields.push(st.clone())
                }
            }
        }
        fields
    }
}

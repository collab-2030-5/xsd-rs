use crate::{SimpleType, TypeId};
use std::collections::BTreeMap;

use std::rc::Rc;

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
    Element(ElemMultiplicity, AnyType),
    Attribute(AttrMultiplicity, SimpleType),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: FieldType,
}

impl Field {
    pub fn bare_name(&self) -> String {
        match self.name.split_once(':') {
            None => self.name.to_owned(),
            Some((_ns, name)) => name.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub comment: Option<String>,
    pub id: TypeId,
    pub base_type: Option<Rc<Struct>>,
    pub fields: Vec<Field>,
    pub metadata: StructMetadata,
    pub variants: Option<Vec<ChoiceVariant>>,
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub comment: Option<String>,
    pub id: TypeId,
    pub variants: Vec<ChoiceVariant>,
}

#[derive(Debug, Clone)]
pub struct ChoiceVariant {
    pub comment: Option<String>,
    pub element_name: String,
    pub type_info: AnyType,
}

/*
#[derive(Debug, Clone)]
pub struct Union {
    pub comment: Option<String>,
    pub id: TypeId,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone)]
pub struct UnionVariant {
    pub comment: Option<String>,
    pub name: String,
    pub type_info: SimpleType,
}
*/

/// Simple or composite like a struct
#[derive(Debug, Clone)]
pub enum AnyType {
    Simple(SimpleType),
    Struct(Struct),
    Choice(Rc<Choice>),
}

impl AnyType {
    pub fn is_struct(&self) -> bool {
        !std::matches!(self, Self::Simple(_))
    }
}

impl From<SimpleType> for AnyType {
    fn from(x: SimpleType) -> Self {
        Self::Simple(x)
    }
}

#[derive(Debug)]
pub struct Model {
    pub types: BTreeMap<TypeId, AnyType>,
    pub simple_types: BTreeMap<TypeId, AnyType>,
}

#[derive(Debug, Copy, Clone)]
pub struct StructMetadata {
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

    pub fn dedup_fields(&self) -> Vec<&Field> {
        let mut output: Vec<&Field> = Default::default();
        self.dedup_fields_impl(&mut output);
        output
    }

    fn dedup_fields_impl<'a>(&'a self, fields: &mut Vec<&'a Field>) {
        if let Some(base) = &self.base_type {
            base.dedup_fields_impl(fields);
        }

        for field in self.fields.iter() {
            match fields.iter().position(|f| f.name == field.name) {
                Some(x) => {
                    fields[x] = field;
                }
                None => {
                    fields.push(field);
                }
            }
        }
    }
}

impl Model {
    pub fn structs(&self) -> impl Iterator<Item = Struct> + '_ {
        self.types.values().filter_map(|t| match t {
            AnyType::Struct(x) => Some(x.clone()),
            _ => None,
        })
    }

    /// return all of the structs that are 1) base structs AND 2) used as fields in other structs
    pub fn base_fields(&self) -> Vec<Struct> {
        self.structs()
            .filter(|x| x.metadata.is_base && x.metadata.use_as_element)
            .collect()
    }

    /// All structs that inherit from a base struct, directly or indirectly
    pub fn sub_structs_of(&self, base: &Rc<Struct>) -> Vec<Struct> {
        let mut structs = Vec::new();

        for other in self.structs() {
            if other.inherits_from(base) && !structs.iter().any(|x: &Struct| x.id == other.id) {
                structs.push(other.clone());
            }
        }

        // can we sort here?
        structs.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));

        structs
    }
}

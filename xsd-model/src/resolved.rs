use crate::{SimpleType, TypeId};
use std::collections::{BTreeMap, HashMap, HashSet};

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
    pub default_ns: String,
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
    pub base_type: Option<Box<Struct>>,
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

impl ChoiceVariant {
    pub fn name_w_namespace(&self) -> String {
        match self.element_name.split_once(':') {
            None => format!("{}:{}", self.type_info.type_id().ns, self.element_name),
            Some(_) => self.element_name.to_owned(),
        }
    }
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

    pub fn replace_substitution_group(&mut self, field_name: &str, choice: &Choice) -> bool {
        match self {
            AnyType::Struct(element) => element.replace_substitution_group(field_name, choice),
            _ => false,
        }
    }

    pub fn type_id(&self) -> TypeId {
        match self {
            AnyType::Simple(_element) => TypeId {
                name: "".to_owned(),
                ns: "".to_owned(),
            },
            AnyType::Struct(element) => element.id.clone(),
            AnyType::Choice(element) => element.id.clone(),
        }
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
    pub namespaces: HashMap<String, String>,
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
            if child.id == self.id {
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

    pub fn replace_substitution_group(&mut self, field_name: &str, choice: &Choice) -> bool {
        Self::replace_substitution_group_fields(field_name, choice, &mut self.fields);

        if let Some(base) = &mut self.base_type {
            Self::replace_substitution_group_fields(field_name, choice, &mut base.fields);
        }

        true
    }

    pub fn replace_substitution_group_fields(
        field_name: &str,
        choice: &Choice,
        fields: &mut Vec<Field>,
    ) {
        for field in fields.iter_mut() {
            if field.name == field_name {
                match field.field_type {
                    FieldType::Element(multiplicity, _) => {
                        field.field_type = FieldType::Element(
                            multiplicity,
                            AnyType::Choice(choice.clone().into()),
                        );
                    }
                    _ => (),
                };
            }
        }
    }
}

impl Choice {
    pub fn replace_substitution_groups(
        &mut self,
        ids: &HashSet<TypeId>,
        groups: HashMap<TypeId, Choice>,
    ) {
        let substitutions = self
            .variants
            .iter()
            .filter(|variant| ids.contains(&variant.type_info.type_id()))
            .cloned()
            .map(|variant| variant.type_info.type_id().clone())
            .collect::<Vec<TypeId>>();

        if substitutions.is_empty() {
            return;
        }

        let mut variants = self
            .variants
            .iter()
            .filter(|variant| !ids.contains(&variant.type_info.type_id()))
            .cloned()
            .collect::<Vec<ChoiceVariant>>()
            .clone();

        let mut groups = groups
            .iter()
            .filter(|(_type_id, group)| substitutions.contains(&group.id))
            .collect::<HashMap<&TypeId, &Choice>>();

        tracing::info!("    Filtered out {} groups", groups.len());

        for group in groups.values() {
            variants.append(&mut group.variants.clone());
        }

        self.variants = variants;
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

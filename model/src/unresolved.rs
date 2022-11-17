use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;

use crate::config::{Config, FieldId};
use crate::map::Map;
use crate::resolved::*;
use crate::resolver::Resolver;
use crate::*;

#[derive(Clone, Debug)]
pub struct UnresolvedChoice {
    pub type_id: TypeId,
    pub comment: Option<String>,
    pub variants: Vec<UnresolvedChoiceVariant>,
}

#[derive(Clone, Debug)]
pub struct UnresolvedChoiceVariant {
    pub comment: Option<String>,
    // name of the element that indicates this variant
    pub element_name: String,
    // type for this variant
    pub type_id: TypeId,
}

impl UnresolvedChoice {
    fn analyze(&self, resolver: &Resolver) {
        for var in self.variants.iter() {
            if var.resolve(resolver).is_none() {
                tracing::warn!(
                    "cannot resolve choice variant {}::{} with type: {}",
                    self.type_id,
                    var.element_name,
                    var.type_id
                );
            }
        }
    }

    fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
        let mut variants: Vec<ChoiceVariant> = Vec::new();
        for var in self.variants.iter() {
            match var.resolve(resolver) {
                None => return None, // can't resolve variant yet
                Some(x) => variants.push(x),
            }
        }

        let choice = Choice {
            comment: self.comment.clone(),
            id: self.type_id.clone(),
            variants,
        };

        Some(AnyType::Choice(Rc::new(choice)))
    }
}

impl UnresolvedChoiceVariant {
    fn resolve(&self, resolver: &Resolver) -> Option<ChoiceVariant> {
        resolver.resolve(&self.type_id).map(|any| ChoiceVariant {
            comment: self.comment.clone(),
            element_name: self.element_name.clone(),
            type_info: any.clone(),
        })
    }
}

/// represent complex types whose sub-types must be resolved
#[derive(Debug, Clone)]
pub enum UnresolvedType {
    Struct(UnresolvedStruct),
    Choice(UnresolvedChoice),
}

impl UnresolvedType {
    fn get_struct(&self) -> Option<&UnresolvedStruct> {
        match self {
            UnresolvedType::Struct(x) => Some(x),
            UnresolvedType::Choice(_) => None,
        }
    }

    fn get_type_id(&self) -> &TypeId {
        match self {
            UnresolvedType::Struct(x) => &x.type_id,
            UnresolvedType::Choice(x) => &x.type_id,
        }
    }
}

/// Extended unresolved types provide additional metadata computed from the entire model
#[derive(Debug, Clone)]
pub enum UnresolvedTypeEx {
    Struct(UnresolvedStruct, StructMetadata),
    Choice(UnresolvedChoice),
}

impl UnresolvedTypeEx {
    fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.resolve(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.resolve(resolver),
        }
    }

    fn analyze(&self, resolver: &Resolver) {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.analyze(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.analyze(resolver),
        }
    }
}

#[derive(Debug, Default)]
pub struct UnresolvedModel {
    pub aliases: Map<TypeId, TypeId>,
    pub simple_types: Map<TypeId, SimpleType>,
    pub unresolved_types: Vec<UnresolvedType>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedField {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: TypeId,
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

#[derive(Copy, Clone, Debug)]
pub enum FieldTypeInfo {
    Attribute(AttributeType),
    Element(ElementType),
}

#[derive(Debug, Clone)]
pub struct UnresolvedStruct {
    pub comment: Option<String>,
    pub type_id: TypeId,
    /// single optional base struct
    pub base_type: Option<TypeId>,
    pub fields: Vec<UnresolvedField>,
}

fn get_field_type(info: FieldTypeInfo, t: AnyType) -> FieldType {
    match info {
        FieldTypeInfo::Attribute(attr_type) => match t {
            AnyType::Struct(_) => panic!("attributes may not reference struct types"),
            AnyType::Choice(_) => panic!("attributes may not reference choice types"),
            AnyType::Simple(x) => match attr_type {
                AttributeType::Single => FieldType::Attribute(AttrMultiplicity::Single, x.clone()),
                AttributeType::Option => {
                    FieldType::Attribute(AttrMultiplicity::Optional, x.clone())
                }
            },
        },
        FieldTypeInfo::Element(x) => match x {
            ElementType::Single => FieldType::Element(ElemMultiplicity::Single, t),
            ElementType::Array => FieldType::Element(ElemMultiplicity::Vec, t),
            ElementType::Option => FieldType::Element(ElemMultiplicity::Optional, t),
        },
    }
}

impl UnresolvedField {
    fn resolve(&self, parent_id: &TypeId, resolver: &Resolver) -> Option<Field> {
        // first try to resolve it using the field substitution map
        let field_id = FieldId {
            parent_id: parent_id.clone(),
            field_name: self.name.clone(),
        };

        tracing::debug!("resolving: {}", field_id);

        if let Some(x) = resolver.resolve_field(&field_id, &self.field_type) {
            let any_type: AnyType = x.clone().into();
            return Some(Field {
                comment: self.comment.clone(),
                name: self.name.clone(),
                field_type: get_field_type(self.info, any_type),
            });
        }

        None
    }
}

impl UnresolvedStruct {
    fn analyze(&self, _metadata: StructMetadata, resolver: &Resolver) {
        if let Some(base) = &self.base_type {
            if resolver.resolve(base).is_none() {
                tracing::warn!(
                    "Cannot resolve base type {} in struct {}",
                    base,
                    self.type_id
                );
            }
        }

        for field in self.fields.iter() {
            if field.resolve(&self.type_id, resolver).is_none() {
                tracing::warn!(
                    "Cannot resolve field ({}).{} with type {}",
                    self.type_id,
                    field.name,
                    field.field_type
                );
            }
        }
    }

    fn resolve(&self, metadata: StructMetadata, resolver: &Resolver) -> Option<AnyType> {
        tracing::debug!("resolving: {}", self.type_id);

        // resolve the base class
        let base_type = match &self.base_type {
            None => None,
            Some(base_id) => {
                match resolver.resolve(base_id) {
                    None => {
                        // base class isn't resolved yet, can't resolve this struct
                        return None;
                    }
                    Some(AnyType::Struct(x)) => Some(x.clone()),
                    Some(x) => {
                        panic!("Base type of {} is not a struct: {:?}", self.type_id, x)
                    }
                }
            }
        };

        // resolve the fields
        let mut fields: Vec<Field> = Vec::new();
        for field in self.fields.iter() {
            match field.resolve(&self.type_id, resolver) {
                None => {
                    // can't resolve field yet
                    return None;
                }
                Some(x) => fields.push(x),
            }
        }

        Some(AnyType::Struct(Rc::new(Struct {
            comment: self.comment.clone(),
            id: self.type_id.clone(),
            base_type,
            fields,
            metadata,
        })))
    }
}

impl UnresolvedModel {
    pub fn merge_xsd(&mut self, path: &Path) {
        let data = std::fs::read_to_string(path).unwrap();
        let rs_file = crate::parse::parser::parse(&data).unwrap();
        crate::parse::merge(rs_file, self)
    }

    fn is_base(&self, id: &TypeId) -> bool {
        for other in self.unresolved_types.iter().filter_map(|x| x.get_struct()) {
            if let Some(other) = &other.base_type {
                if other == id {
                    return true;
                }
            }
        }
        false
    }

    fn used_as_field(&self, id: &TypeId) -> bool {
        for other in self.unresolved_types.iter().filter_map(|x| x.get_struct()) {
            for field in other.fields.iter() {
                if &field.field_type == id {
                    return true;
                }
            }
        }
        false
    }

    fn extend(&self, unresolved: &UnresolvedType) -> UnresolvedTypeEx {
        match unresolved {
            UnresolvedType::Struct(x) => {
                let metadata = StructMetadata {
                    is_base: self.is_base(&x.type_id),
                    use_as_element: self.used_as_field(&x.type_id),
                };
                UnresolvedTypeEx::Struct(x.clone(), metadata)
            }
            UnresolvedType::Choice(x) => UnresolvedTypeEx::Choice(x.clone()),
        }
    }

    pub fn compute_metadata(&self) -> Map<TypeId, UnresolvedTypeEx> {
        let mut meta_map: Map<TypeId, UnresolvedTypeEx> = Default::default();

        for t in self.unresolved_types.iter() {
            meta_map.insert(t.get_type_id().clone(), self.extend(t));
        }

        meta_map
    }

    pub fn resolve(self, config: Config) -> crate::resolved::Model {
        // unresolved types with extended metadata
        let mut unresolved = self.compute_metadata().to_inner();

        //  type used to resolve them
        let mut resolver = Resolver::new(config, self.simple_types, self.aliases);

        let mut count: usize = 0;

        loop {
            let span = tracing::info_span!("resolve", i = count);
            let _entered = span.enter();

            if unresolved.is_empty() {
                tracing::info!("success in {} iterations", count);
                return resolver.model();
            }

            if let Some((any_type, id)) = unresolved
                .iter()
                .find_map(|(id, v)| v.resolve(&resolver).map(|x| (x, id.clone())))
            {
                tracing::info!("resolved type: {}", id);
                unresolved.remove(&id).expect("cannot be empty");
                resolver.insert(id, any_type);
            } else {
                tracing::error!("Cannot resolve remaining {} types", unresolved.len());

                for unresolved in unresolved.values() {
                    unresolved.analyze(&resolver);
                }

                panic!("resolution failed");
            }

            count += 1;
        }
    }
}

use crate::config::FieldId;
use crate::resolved::{
    AnyType, AttrMultiplicity, ElemMultiplicity, Field, FieldType, SimpleAttributeType, Struct,
    StructMetadata,
};
use crate::resolver::Resolver;
use crate::TypeId;
use std::rc::Rc;

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
pub(crate) struct UnresolvedStruct {
    pub(crate) comment: Option<String>,
    pub(crate) type_id: TypeId,
    /// single optional base struct
    pub(crate) base_type: Option<TypeId>,
    pub(crate) fields: Vec<UnresolvedField>,
}

#[derive(Debug, Clone)]
pub(crate) struct UnresolvedField {
    pub(crate) comment: Option<String>,
    pub(crate) name: String,
    pub(crate) field_type: TypeId,
    pub(crate) info: FieldTypeInfo,
}

impl UnresolvedField {
    fn resolve(&self, parent_id: &TypeId, resolver: &Resolver) -> Option<Field> {
        // first try to resolve it using the field substitution map
        let field_id = FieldId {
            parent_id: parent_id.clone(),
            field_name: self.name.clone(),
        };

        tracing::debug!("resolving: {}", field_id);

        if let Some(any_type) = resolver.resolve_field(&field_id, &self.field_type) {
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
    pub(crate) fn analyze(&self, _metadata: StructMetadata, resolver: &Resolver) {
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

    pub(crate) fn resolve(&self, metadata: StructMetadata, resolver: &Resolver) -> Option<AnyType> {
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
                    Some(AnyType::Struct(x)) => Some(x),
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

impl From<AttributeType> for AttrMultiplicity {
    fn from(x: AttributeType) -> Self {
        match x {
            AttributeType::Single => Self::Single,
            AttributeType::Option => Self::Optional,
        }
    }
}

fn get_field_type(info: FieldTypeInfo, t: AnyType) -> FieldType {
    match info {
        FieldTypeInfo::Attribute(attr_type) => match t {
            AnyType::Struct(_) => panic!("attributes may not reference struct types"),
            AnyType::Choice(_) => panic!("attributes may not reference choice types"),
            AnyType::Union(x) => {
                FieldType::Attribute(attr_type.into(), SimpleAttributeType::Union(x.clone()))
            }
            AnyType::Simple(x) => {
                FieldType::Attribute(attr_type.into(), SimpleAttributeType::Simple(x))
            }
        },
        FieldTypeInfo::Element(x) => match x {
            ElementType::Single => FieldType::Element(ElemMultiplicity::Single, t),
            ElementType::Array => FieldType::Element(ElemMultiplicity::Vec, t),
            ElementType::Option => FieldType::Element(ElemMultiplicity::Optional, t),
        },
    }
}

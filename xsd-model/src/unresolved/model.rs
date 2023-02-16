use heck::ToUpperCamelCase;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;

use crate::config::{BaseTypeConfig, Config};
use crate::map::Map;
use crate::parser::types::{
    Alias, Enum, EnumSource, RsEntity, Struct, StructFieldSource, TupleStruct, TypeModifier,
};
use crate::resolved::{AnyType, StructMetadata};
use crate::resolver::Resolver;
use crate::unresolved::choice::UnresolvedChoice;
use crate::unresolved::structs::UnresolvedStruct;
use crate::unresolved::{
    AttributeType, ElementType, FieldTypeInfo, UnresolvedChoiceVariant, UnresolvedField,
};

use crate::unresolved::union::{UnresolvedUnion, UnresolvedUnionVariant};
use crate::*;

#[derive(Debug, Default)]
pub struct UnresolvedModel {
    pub(crate) aliases: Map<TypeId, TypeId>,
    pub(crate) simple_types: Map<TypeId, SimpleType>,
    pub(crate) unresolved_types: Vec<UnresolvedType>,
}

pub(crate) struct Settings<'a> {
    pub(crate) namespace: &'a str,
}

impl UnresolvedModel {
    /// parser and XSD file and merge it into the unresolved xsd-model
    pub fn merge_xsd(&mut self, path: &Path) {
        let data = std::fs::read_to_string(path).unwrap();
        let xsd = parser::parse(&data).unwrap();

        let namespace = xsd
            .target_ns
            .clone()
            .expect("must contain a namespace entry");

        let ns_name = namespace.name().expect("must contain a namespace name");

        tracing::info!("target namespace: {}", ns_name);

        let settings = Settings { namespace: ns_name };

        for entity in xsd.types.iter() {
            self.merge_entity(entity, &settings);
        }
    }

    fn merge_entity(&mut self, entity: &RsEntity, settings: &Settings) {
        match entity {
            RsEntity::Struct(x) => self.merge_struct(x, settings),
            RsEntity::TupleStruct(x) => self.merge_tuple_struct(x, settings),
            RsEntity::Enum(x) => self.merge_enum(x, settings),
            RsEntity::Alias(x) => self.merge_alias(x, settings),
            RsEntity::Import(_) => {}
            _ => panic!("Unsupported entity type: {:?}", entity),
        }
    }

    fn merge_struct(&mut self, st: &Struct, settings: &Settings) {
        let type_id = TypeId::parse(st.name.as_str(), settings.namespace);
        let base_type = extract_base_type(st, settings);
        let fields = self.extract_fields(&st.fields.borrow(), settings);

        match &base_type {
            Some(base) => {
                tracing::info!("added Struct({}) (base_type == {})", type_id, base);
            }
            None => {
                tracing::info!("added Struct({})", type_id);
            }
        }

        let st = UnresolvedStruct {
            comment: st.comment.clone(),
            type_id,
            base_type,
            fields,
        };

        self.unresolved_types.push(UnresolvedType::Struct(st));
    }

    fn extract_fields(
        &mut self,
        fields: &[crate::parser::types::StructField],
        settings: &Settings,
    ) -> Vec<UnresolvedField> {
        let mut output: Vec<UnresolvedField> = Default::default();

        for field in fields {
            // add any subtypes recursively
            for en in field.subtypes.iter() {
                self.merge_entity(en, settings);
            }

            let info: Option<FieldTypeInfo> = match field.source {
                StructFieldSource::Attribute => Some(FieldTypeInfo::Attribute(get_attribute_type(
                    &field.type_modifiers,
                ))),
                StructFieldSource::Element | StructFieldSource::Choice => {
                    match get_element_type(&field.type_modifiers) {
                        None => Some(FieldTypeInfo::Element(ElementType::Single)),
                        Some(x) => Some(FieldTypeInfo::Element(x)),
                    }
                }
                StructFieldSource::Base => None,
                StructFieldSource::NA => unimplemented!(),
            };

            if let Some(info) = info {
                let field = UnresolvedField {
                    comment: field.comment.clone(),
                    name: field.name.clone(),
                    field_type: TypeId::parse(&field.type_name, settings.namespace),
                    info,
                };
                output.push(field);
            }
        }

        output
    }

    fn merge_enum(&mut self, en: &Enum, settings: &Settings) {
        for entity in en.subtypes.iter() {
            tracing::debug!("enum sub-type: {:#?}", entity);
            self.merge_entity(entity, settings);
        }

        match en.source {
            EnumSource::Restriction => {
                let en = convert_restricted_enum(en, settings);
                let type_id = en.type_id.clone();
                let en = SimpleType::Wrapper(WrapperType::Enum(Rc::new(en)));
                tracing::debug!("Adding {} as a restricted enum", type_id);
                self.simple_types.insert(type_id, en);
            }
            EnumSource::Choice => {
                let choice = convert_choice_enum(en, settings);
                tracing::debug!("Adding {} as an unresolved choice", choice.type_id);
                self.unresolved_types.push(UnresolvedType::Choice(choice));
            }
            EnumSource::Union => {
                let union = convert_union_enum(en, settings);
                tracing::debug!("Adding {} as an unresolved union", union.type_id);
                self.unresolved_types.push(UnresolvedType::Union(union));
            }
            _ => panic!("Unsupported enum source type: {:?} in {:#?}", en.source, en),
        }
    }

    fn merge_tuple_struct(&mut self, ts: &TupleStruct, settings: &Settings) {
        let type_id = TypeId::parse(&ts.name, settings.namespace);
        let base_type_id = TypeId::parse(&ts.type_name, settings.namespace);

        if !ts.subtypes.is_empty() {
            let span = tracing::info_span!("recurse", "TupleStruct({})", type_id);
            let _guard = span.enter();
            for entity in ts.subtypes.iter() {
                self.merge_entity(entity, settings);
            }
        }

        match PrimitiveType::try_resolve_xs_type(&base_type_id) {
            None => {
                panic!(
                    "Cannot resolve TupleStruct({}) with base type {}",
                    type_id, base_type_id
                );
                //self.unresolved_types.push(UnresolvedType::Tuple(UnresolvedTupleStruct::new(type_id, ts.clone())))
            }
            Some(pt) => {
                // apply all the facets to the type to constrain it
                let pt = pt.apply_facets(&ts.facets);
                tracing::debug!(
                    "TupleStruct({}) with base type {} is primitive type: {:?}",
                    type_id,
                    base_type_id,
                    pt
                );
                self.simple_types.insert(type_id, SimpleType::Primitive(pt));
            }
        }
    }

    fn merge_alias(&mut self, x: &Alias, settings: &Settings) {
        let target = TypeId::parse(&x.original, settings.namespace);
        let alias = TypeId::parse(&x.name, settings.namespace);
        tracing::debug!("{} is an alias for {}", alias, target);
        self.aliases.insert(alias, target);
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
            UnresolvedType::Union(x) => UnresolvedTypeEx::Union(x.clone()),
        }
    }

    // find all structs that inherit from this struct
    fn get_parents_of(&self, st: &UnresolvedStruct) -> Vec<UnresolvedStruct> {
        let mut parents: Vec<UnresolvedStruct> = Default::default();

        for t in self.unresolved_types.iter() {
            if let UnresolvedType::Struct(x) = t {
                if x.base_type.as_ref() == Some(&st.type_id) {
                    parents.push(x.clone());
                    parents.extend(self.get_parents_of(x));
                }
            }
        }

        parents
    }

    pub fn resolve(self, config: Config) -> crate::resolved::Model {
        // unresolved types with extended metadata
        let mut unresolved = self.resolve_base_types(&config.base_types);

        // type used to resolve them
        let mut resolver = Resolver::new(config.mappings, self.simple_types, self.aliases);

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

    fn resolve_base_types(&self, _config: &BaseTypeConfig) -> BTreeMap<TypeId, UnresolvedTypeEx> {
        let unresolved = self.compute_metadata().to_inner();

        // look for structs that are base types
        for unresolved in unresolved.values() {
            match unresolved {
                UnresolvedTypeEx::Struct(st, meta) => {
                    if meta.is_base {
                        let inherited_types = self.get_parents_of(st);
                        let names: Vec<String> = inherited_types
                            .iter()
                            .map(|x| format!("{}", x.type_id))
                            .collect();
                        tracing::warn!(
                            "base struct: {} with inherited types: {:?}",
                            st.type_id,
                            names
                        );
                    }
                }
                UnresolvedTypeEx::Choice(_) => {}
                UnresolvedTypeEx::Union(_) => {}
            }
        }

        unresolved
    }

    fn compute_metadata(&self) -> Map<TypeId, UnresolvedTypeEx> {
        let mut meta_map: Map<TypeId, UnresolvedTypeEx> = Default::default();

        for t in self.unresolved_types.iter() {
            meta_map.insert(t.get_type_id().clone(), self.extend(t));
        }

        meta_map
    }
}

/// represent complex types whose sub-types must be resolved
#[derive(Debug, Clone)]
pub(crate) enum UnresolvedType {
    Struct(UnresolvedStruct),
    Choice(UnresolvedChoice),
    Union(UnresolvedUnion),
}

impl UnresolvedType {
    fn get_struct(&self) -> Option<&UnresolvedStruct> {
        match self {
            Self::Struct(x) => Some(x),
            Self::Choice(_) => None,
            Self::Union(_) => None,
        }
    }

    fn get_type_id(&self) -> &TypeId {
        match self {
            Self::Struct(x) => &x.type_id,
            Self::Choice(x) => &x.type_id,
            Self::Union(x) => &x.type_id,
        }
    }
}

/// Extended unresolved types provide additional metadata computed from the entire xsd-model
#[derive(Debug, Clone)]
enum UnresolvedTypeEx {
    Struct(UnresolvedStruct, StructMetadata),
    Choice(UnresolvedChoice),
    Union(UnresolvedUnion),
}

impl UnresolvedTypeEx {
    fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.resolve(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.resolve(resolver),
            UnresolvedTypeEx::Union(x) => x.resolve(resolver),
        }
    }

    fn analyze(&self, resolver: &Resolver) {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.analyze(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.analyze(resolver),
            UnresolvedTypeEx::Union(_) => unimplemented!(),
        }
    }
}

fn convert_restricted_enum(en: &Enum, settings: &Settings) -> Enumeration {
    let mut variants: Vec<NamedEnumVariant> = Vec::new();
    let mut set: HashSet<String> = Default::default();
    for v in en.cases.iter() {
        let name = v.name.to_upper_camel_case();

        let mut suffix = 2;
        let mut current = v.name.to_upper_camel_case();
        let name = loop {
            if set.contains(&current) {
                let next = format!("{}{}", name, suffix);
                tracing::warn!(
                    "variant {}::{} conflicts, renaming to {}::{}",
                    en.name,
                    current,
                    en.name,
                    next
                );
                current = next;
            } else {
                break current;
            }
            suffix += 1;
        };

        set.insert(name.clone());

        let comment = {
            let docs = format!("xml value == '{}'", v.value);
            match &v.comment {
                None => docs,
                Some(x) => format!("{} ({})", x, docs),
            }
        };

        variants.push(NamedEnumVariant {
            name,
            value: v.value.clone(),
            comment: Some(comment),
        });
    }

    Enumeration {
        type_id: TypeId::parse(&en.name, settings.namespace),
        comment: en.comment.clone(),
        variants,
    }
}

fn convert_choice_enum(en: &Enum, settings: &Settings) -> UnresolvedChoice {
    let mut variants: Vec<UnresolvedChoiceVariant> = Vec::new();
    for v in en.cases.iter() {
        let type_name = v
            .type_name
            .as_ref()
            .expect("choice case must include a type name");

        let variant = UnresolvedChoiceVariant {
            comment: v.comment.clone(),
            element_name: v.name.clone(),
            type_id: TypeId::parse(type_name, settings.namespace),
        };

        variants.push(variant);
    }
    UnresolvedChoice {
        type_id: TypeId::parse(&en.name, settings.namespace),
        comment: en.comment.clone(),
        variants,
    }
}

fn convert_union_enum(en: &Enum, settings: &Settings) -> UnresolvedUnion {
    let mut variants: Vec<UnresolvedUnionVariant> = Vec::new();
    for v in en.cases.iter() {
        let type_name = v
            .type_name
            .as_ref()
            .expect("union case must include a type name");

        variants.push(UnresolvedUnionVariant {
            name: TypeId::parse(&v.name, settings.namespace),
            comment: v.comment.clone(),
            type_name: TypeId::parse(&type_name, settings.namespace),
        });
    }
    UnresolvedUnion {
        type_id: TypeId::parse(&en.name, settings.namespace),
        comment: en.comment.clone(),
        variants,
    }
}

fn extract_base_type(x: &Struct, settings: &Settings) -> Option<TypeId> {
    let base_types: Vec<String> = x
        .fields
        .borrow()
        .iter()
        .filter_map(|x| match x.source {
            StructFieldSource::Attribute => None,
            StructFieldSource::Element => None,
            StructFieldSource::Base => Some(x.type_name.clone()),
            StructFieldSource::Choice => None,
            StructFieldSource::NA => unimplemented!(),
        })
        .collect();

    match base_types.as_slice() {
        [] => None,
        [x] => Some(TypeId::parse(x.as_str(), settings.namespace)),
        _ => panic!("Unexpected number of base types: {:#?}", base_types),
    }
}

fn get_attribute_type(input: &[TypeModifier]) -> AttributeType {
    let modifiers: Vec<AttributeType> = input
        .iter()
        .filter_map(|x| match x {
            TypeModifier::None => None,
            TypeModifier::Array => unimplemented!(),
            TypeModifier::Option => Some(AttributeType::Option),
            TypeModifier::Empty => unimplemented!(),
        })
        .collect();

    match modifiers.as_slice() {
        [] => AttributeType::Single,
        [x] => *x,
        _ => panic!("Unexpected field modifier count: {:#?}", modifiers),
    }
}

fn get_element_type(input: &[TypeModifier]) -> Option<ElementType> {
    let modifiers: Vec<ElementType> = input
        .iter()
        .filter_map(|x| match x {
            TypeModifier::None => None,
            TypeModifier::Array => Some(ElementType::Array),
            TypeModifier::Option => Some(ElementType::Option),
            TypeModifier::Empty => None,
        })
        .collect();

    match modifiers.as_slice() {
        [] => None,
        [x] => Some(*x),
        _ => panic!("Unexpected field modifier count: {:#?}", modifiers),
    }
}
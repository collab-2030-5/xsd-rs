use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;

use crate::config::{Config, FieldId};
use crate::map::Map;
use crate::parse::parser::types::{Alias, Facet, RsEntity, RsFile, TupleStruct};
use crate::resolved::*;
use crate::resolver::Resolver;
use crate::unresolved::choice::UnresolvedChoice;
use crate::unresolved::structs::UnresolvedStruct;
use crate::*;

/// represent complex types whose sub-types must be resolved
#[derive(Debug, Clone)]
pub enum UnresolvedType {
    Struct(UnresolvedStruct),
    Choice(UnresolvedChoice),
    Tuple(UnresolvedTupleStruct),
}

impl UnresolvedType {
    fn get_struct(&self) -> Option<&UnresolvedStruct> {
        match self {
            UnresolvedType::Struct(x) => Some(x),
            UnresolvedType::Choice(_) => None,
            UnresolvedType::Tuple(_) => None,
        }
    }

    fn get_type_id(&self) -> &TypeId {
        match self {
            UnresolvedType::Struct(x) => &x.type_id,
            UnresolvedType::Choice(x) => &x.type_id,
            UnresolvedType::Tuple(x) => &x.type_id,
        }
    }
}

/// Extended unresolved types provide additional metadata computed from the entire model
#[derive(Debug, Clone)]
pub enum UnresolvedTypeEx {
    Struct(UnresolvedStruct, StructMetadata),
    Choice(UnresolvedChoice),
    Tuple(UnresolvedTupleStruct),
}

impl UnresolvedTypeEx {
    fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.resolve(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.resolve(resolver),
            UnresolvedTypeEx::Tuple(_) => unimplemented!(),
        }
    }

    fn analyze(&self, resolver: &Resolver) {
        match self {
            UnresolvedTypeEx::Struct(x, metadata) => x.analyze(*metadata, resolver),
            UnresolvedTypeEx::Choice(x) => x.analyze(resolver),
            UnresolvedTypeEx::Tuple(_) => unimplemented!(),
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
pub struct UnresolvedTupleStruct {
    pub name: String,
    pub comment: Option<String>,
    pub type_id: TypeId,
    pub facets: Vec<Facet>,
}

impl From<UnresolvedTupleStruct> for UnresolvedType {
    fn from(x: UnresolvedTupleStruct) -> Self {
        Self::Tuple(x)
    }
}

impl UnresolvedTupleStruct {
    pub(crate) fn new(ts: TupleStruct, settings: &Settings) -> Self {
        if !ts.type_modifiers.is_empty() {
            panic!(
                "TupleStruct {} contains types modifiers: {:?}",
                ts.name, ts.type_modifiers
            );
        }

        if !ts.subtypes.is_empty() {
            panic!(
                "TupleStruct {} contains subtypes: {:?}",
                ts.name, ts.subtypes
            );
        }

        Self {
            name: ts.name,
            comment: ts.comment,
            type_id: TypeId::parse(&ts.type_name, settings.namespace),
            facets: ts.facets,
        }
    }
}

pub(crate) struct Settings<'a> {
    pub(crate) namespace: &'a str,
}

impl UnresolvedModel {
    pub fn merge_xsd(&mut self, path: &Path) {
        let data = std::fs::read_to_string(path).unwrap();
        let xsd = crate::parse::parser::parse(&data).unwrap();
        self.merge(xsd)
    }

    /// merge the parsed XSD into the existing unresolved model
    fn merge(&mut self, xsd: RsFile) {
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

    fn merge_entity(&mut self, en: &RsEntity, settings: &Settings) {}

    fn merge_alias(&mut self, x: Alias, settings: &Settings) {
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
            UnresolvedType::Tuple(x) => UnresolvedTypeEx::Tuple(x.clone()),
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

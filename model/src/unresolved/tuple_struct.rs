use crate::parse::parser::types::{Facet, TupleStruct};
use crate::unresolved::model::{Settings, UnresolvedType};
use crate::TypeId;

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

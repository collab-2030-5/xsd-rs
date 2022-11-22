use crate::parser::types::{Facet, TupleStruct};
use crate::unresolved::model::UnresolvedType;
use crate::TypeId;

#[derive(Debug, Clone)]
pub(crate) struct UnresolvedTupleStruct {
    pub(crate) name: String,
    pub(crate) comment: Option<String>,
    pub(crate) type_id: TypeId,
    pub(crate) facets: Vec<Facet>,
}

impl From<UnresolvedTupleStruct> for UnresolvedType {
    fn from(x: UnresolvedTupleStruct) -> Self {
        Self::Tuple(x)
    }
}

impl UnresolvedTupleStruct {
    pub(crate) fn new(type_id: TypeId, ts: TupleStruct) -> Self {
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
            type_id,
            facets: ts.facets,
        }
    }
}

use crate::resolved::{AnyType, Choice, ChoiceVariant};
use crate::resolver::Resolver;
use crate::TypeId;
use std::rc::Rc;

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
    pub(crate) fn analyze(&self, resolver: &Resolver) {
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

    pub(crate) fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
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

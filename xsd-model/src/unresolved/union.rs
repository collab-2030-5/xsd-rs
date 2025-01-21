use crate::resolved::AnyType;
use crate::resolver::Resolver;
use crate::{PrimitiveType, SimpleType, StringConstraints, TypeId};
use std::rc::Rc;

/// One of multiple possible simple types
#[derive(Clone, Debug)]
pub struct UnresolvedUnion {
    pub name: TypeId,
    pub type_id: TypeId,
    pub comment: Option<String>,
    pub variants: Vec<UnresolvedUnionVariant>,
}

#[derive(Clone, Debug)]
pub struct UnresolvedUnionVariant {
    pub name: TypeId,
    pub comment: Option<String>,
    pub type_name: TypeId,
}

impl UnresolvedUnion {
    pub(crate) fn resolve(&self, resolver: &Resolver) -> Option<AnyType> {
        let mut variants: Vec<crate::resolved::ChoiceVariant> = Vec::new();

        tracing::info!("Resolving union variants: {:#?}", self.variants);

        for variant in self.variants.iter() {
            let any = match resolver.resolve(&variant.type_name) {
                Some(x) => x,
                None => return None,
            };

            match &any {
                AnyType::Simple(x) => {
                    let choice = crate::resolved::ChoiceVariant {
                        comment: variant.comment.clone(),
                        element_name: variant.name.name.clone(),
                        type_info: any.clone(),
                    };

                    variants.push(choice);
                }
                _ => panic!(
                    "Union variant {} resolved to complex type: {:#?}",
                    variant.name, any
                ),
            }
        }

        let choice = crate::resolved::Choice {
            comment: self.comment.clone(),
            id: self.type_id.clone(),
            name: Some(self.name.clone()),
            variants,
            is_from_union: true,
        };

        Some(AnyType::Choice(Rc::new(choice)))
    }
}

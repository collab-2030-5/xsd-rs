use crate::resolved::AnyType;
use crate::resolver::Resolver;
use crate::{PrimitiveType, SimpleType, StringConstraints, TypeId};

/// One of multiple possible simple types
#[derive(Clone, Debug)]
pub struct UnresolvedUnion {
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
    pub(crate) fn resolve(&self, _resolver: &Resolver) -> Option<AnyType> {
        // just punt on this for now and make all unions strings
        Some(SimpleType::Primitive(PrimitiveType::String(StringConstraints::default())).into())

        /*
        let mut variants: Vec<UnionVariant> = Default::default();

        for v in self.variants.iter() {
            let any = match resolver.resolve(&v.type_name) {
                Some(x) => x,
                None => return None,
            };

            match any {
                AnyType::Simple(x) => {
                    let variant = UnionVariant {
                        comment: v.comment.clone(),
                        name: v.name.name.clone(),
                        type_info: x,
                    };
                    variants.push(variant);
                }
                _ => panic!(
                    "Union variant {} resolved to complex type: {:#?}",
                    v.name, any
                ),
            }
        }

        let resolved = Union {
            comment: self.comment.clone(),
            id: self.type_id.clone(),
            variants,
        };

        Some(AnyType::Union(Rc::new(resolved)))
             */
    }
}

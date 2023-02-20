use crate::config::{FieldId, MappingConfig, ResolvedConfig};
use crate::map::Map;
use crate::resolved::{AnyType, Model};
use crate::{
    HexByteConstraints, NumericConstraint, NumericType, PrimitiveType, SimpleType,
    StringConstraints, TypeId,
};

#[derive(Debug)]
struct AliasMap {
    inner: Map<TypeId, TypeId>,
}

impl AliasMap {
    fn new(inner: Map<TypeId, TypeId>) -> Self {
        Self { inner }
    }

    fn get(&self, alias: &TypeId) -> Option<&TypeId> {
        let ret = self.get_recursive(0, alias);
        if let Some((depth, target)) = ret {
            tracing::debug!("{} is an alias for {} (depth == {})", alias, target, depth);
        }
        ret.map(|x| x.1)
    }

    fn get_recursive(&self, depth: usize, id: &TypeId) -> Option<(usize, &TypeId)> {
        let first = self.inner.get(id);
        match first {
            None => None,
            Some(target) => {
                // we can always return this, but can we resolve deeper?
                match self.get_recursive(depth + 1, target) {
                    Some(deeper) => Some(deeper),
                    None => Some((depth, target)),
                }
            }
        }
    }
}

/// type that holds all of the information needed to resolve types and tracks
/// the master map of resolved types
#[derive(Debug)]
pub(crate) struct Resolver {
    /// resolved version of the configuration
    config: ResolvedConfig,
    /// running map of resolved types
    resolved: Map<TypeId, AnyType>,
    /// store simple types separate to avoid name collisions
    simple: Map<TypeId, AnyType>,
    /// map of aliases we use to lookup types
    aliases: AliasMap,
}

impl Resolver {
    pub(crate) fn new(
        config: MappingConfig,
        simple_types: Map<TypeId, SimpleType>,
        aliases: Map<TypeId, TypeId>,
    ) -> Self {
        // make sure the config is valid (panics)
        let config = config.resolve();

        let mut resolved: Map<TypeId, AnyType> = Default::default();
        let mut simple: Map<TypeId, AnyType> = Default::default();

        // add the simple types
        for (id, t) in simple_types.into_iter() {
            tracing::debug!("{} -> {:?}", id, t);
            simple.insert(id, t.into());
        }

        Self {
            config,
            resolved,
            simple,
            aliases: AliasMap::new(aliases),
        }
    }

    pub(crate) fn resolve(&self, id: &TypeId) -> Option<AnyType> {
        // apply type substitutions in config
        if let Some(x) = self.config.type_mappings.get(id) {
            tracing::debug!("substituting {} with {:?}", id, x);
            return Some(x.clone().into());
        }

        // then do a type lookup using aliases
        let target = self.aliases.get(id).unwrap_or(id);

        // then resolve "xs" types
        if let Some(t) = Self::resolve_xs_type(target) {
            return Some(t.into());
        }

        if self.resolved.get(target).is_none() {
            return self.simple.get(target).cloned();
        }

        // finally look at all the types that have already been fully resolved
        self.resolved.get(target).cloned()
    }

    // just like resolve(), but checks for type substitution on fields first
    pub(crate) fn resolve_field(&self, id: &FieldId, type_id: &TypeId) -> Option<AnyType> {
        // see if there's a substitution for this field
        if let Some(x) = self.config.field_mappings.get(id) {
            return Some(x.clone().into());
        }

        // otherwise, do resolution solely based on the field's type id
        self.resolve(type_id)
    }

    pub(crate) fn insert(&mut self, id: TypeId, any: AnyType) {
        self.resolved.insert(id, any)
    }

    pub(crate) fn model(self) -> Model {
        Model {
            types: self.resolved.to_inner(),
            simple_types: self.simple.to_inner(),
        }
    }

    fn resolve_xs_type(type_id: &TypeId) -> Option<SimpleType> {
        if type_id.ns.as_str() != "xs" {
            return None;
        }

        match type_id.name.as_str() {
            "boolean" => Some(PrimitiveType::Boolean.into()),
            "anyURI" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "hexBinary" => Some(PrimitiveType::HexBytes(HexByteConstraints::default()).into()),
            "base64Binary" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "string" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "ID" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "NMTOKEN" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "byte" => Some(NumericType::I8(NumericConstraint::default()).into()),
            "unsignedByte" => Some(NumericType::U8(NumericConstraint::default()).into()),
            "short" => Some(NumericType::I16(NumericConstraint::default()).into()),
            "unsignedShort" => Some(NumericType::U16(NumericConstraint::default()).into()),
            "int" => Some(NumericType::I32(NumericConstraint::default()).into()),
            "integer" => Some(PrimitiveType::String(StringConstraints::default()).into()),
            "unsignedInt" => Some(NumericType::U32(NumericConstraint::default()).into()),
            "long" => Some(NumericType::I64(NumericConstraint::default()).into()),
            "unsignedLong" => Some(NumericType::U64(NumericConstraint::default()).into()),
            "positiveInteger" => Some(NumericType::U64(NumericConstraint::default()).into()),
            "float" => Some(NumericType::F32(NumericConstraint::default()).into()),
            // TODO - default formatting for double doesn't use scientific notation so this is OK for decimal type too
            // we could provide a more optimized representation for double/float that uses scientific notation to remove zeroes
            "double" | "decimal" => Some(NumericType::F64(NumericConstraint::default()).into()),
            _ => {
                panic!("unhandled xs type: {}", type_id);
            }
        }
    }
}

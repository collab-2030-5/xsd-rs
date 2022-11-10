pub(crate) mod parser;

use heck::ToUpperCamelCase;
use parser::types::{RsEntity, RsFile, StructFieldSource, TupleStruct, TypeModifier};
use parser::xsd_elements::FacetType;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use crate::map::Map;
use crate::parse::parser::types::{Enum, EnumSource, Struct};
use crate::unresolved::*;
use crate::*;

pub fn parse_xsd(xsd: &str) -> impl Debug + '_ {
    crate::parse::parser::parse(xsd).unwrap()
}

struct Settings<'a> {
    namespace: &'a str,
}

/// merge the parsed XSD into the existing unresolved model
pub(crate) fn merge(xsd: RsFile, model: &mut UnresolvedModel) {
    let namespace = xsd
        .target_ns
        .clone()
        .expect("must contain a namespace entry");

    let ns_name = namespace.name().expect("must contain a namespace name");

    tracing::info!("target namespace: {}", ns_name);

    let settings = Settings { namespace: ns_name };

    let mut simple_types = resolve_simple_types(&xsd, &settings);
    let mut structs = extract_structs_from_root(&xsd, &settings);
    let choices = extract_choice_types(&xsd, &settings);

    // find struct that are mis-classified - some are just inherited from basic types
    let base_structs: HashMap<TypeId, SimpleType> = structs
        .iter()
        .filter_map(|x| match &x.base_type {
            Some(bt) => simple_types
                .get(bt)
                .map(|st| (x.type_id.clone(), st.clone())),
            None => None,
        })
        .collect();

    // remove these structs from the struct list
    structs.retain(|f| !base_structs.contains_key(&f.type_id));

    // add these aliases to the simple types list
    for (k, v) in base_structs.iter() {
        simple_types.insert(k.clone(), v.clone());
    }

    // add everything to the model

    for (k, v) in extract_aliases(&xsd, &settings).to_inner() {
        model.aliases.insert(k, v);
    }

    for (k, v) in simple_types.to_inner() {
        model.simple_types.insert(k, v);
    }

    for s in structs {
        model.unresolved_types.push(UnresolvedType::Struct(s));
    }

    for c in choices {
        model.unresolved_types.push(UnresolvedType::Choice(c))
    }
}

fn extract_aliases(model: &RsFile, settings: &Settings) -> Map<TypeId, TypeId> {
    let mut map: Map<TypeId, TypeId> = Default::default();

    for entity in model.types.iter() {
        if let RsEntity::Alias(x) = entity {
            let target = TypeId::parse(&x.original, settings.namespace);
            let alias = TypeId::parse(&x.name, settings.namespace);
            tracing::debug!("{} is an alias for {}", alias, target);
            map.insert(alias, target);
        }
    }

    map
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
            StructFieldSource::Choice => unimplemented!(),
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

fn extract_fields(
    st: &Struct,
    settings: &Settings,
) -> (Vec<UnresolvedField>, Vec<UnresolvedStruct>) {
    let mut fields: Vec<UnresolvedField> = Default::default();
    let mut structs: Vec<UnresolvedStruct> = Default::default();

    for field in st.fields.borrow().iter() {
        for en in field.subtypes.iter() {
            match en {
                RsEntity::Struct(st) => {
                    structs.append(&mut extract_structs(st, settings, Some(st)));
                }
                _ => panic!("Unsupported field subtype: {:#?}", en),
            }
        }

        let field = match field.source {
            StructFieldSource::Attribute => Some(UnresolvedField {
                comment: field.comment.clone(),
                name: field.name.clone(),
                field_type: TypeId::parse(&field.type_name, settings.namespace),
                info: FieldTypeInfo::Attribute(get_attribute_type(&field.type_modifiers)),
            }),
            StructFieldSource::Element => {
                let element_type = match get_element_type(&field.type_modifiers) {
                    None => ElementType::Single,
                    Some(x) => x,
                };

                Some(UnresolvedField {
                    comment: field.comment.clone(),
                    name: field.name.clone(),
                    field_type: TypeId::parse(&field.type_name, settings.namespace),
                    info: FieldTypeInfo::Element(element_type),
                })
            }
            StructFieldSource::Base => None,
            StructFieldSource::Choice => unimplemented!(),
            StructFieldSource::NA => unimplemented!(),
        };

        if let Some(field) = field {
            fields.push(field);
        }
    }

    (fields, structs)
}

fn extract_structs(
    st: &Struct,
    settings: &Settings,
    parent: Option<&Struct>,
) -> Vec<UnresolvedStruct> {
    let mut structs: Vec<UnresolvedStruct> = Default::default();

    let base_type = extract_base_type(st, settings);

    let (fields, mut sub_structs) = { extract_fields(st, settings) };

    structs.append(&mut sub_structs);

    tracing::info!(
        "added {} (parent == {:?})",
        st.name,
        parent.map(|x| x.name.as_str())
    );

    structs.push(UnresolvedStruct {
        comment: st.comment.clone(),
        type_id: TypeId::parse(st.name.as_str(), settings.namespace),
        base_type,
        fields,
    });

    for entity in st.subtypes.iter() {
        panic!("Unhandled subtype: {:?}", entity)
    }

    structs
}

fn extract_structs_from_root(entity: &RsFile, settings: &Settings) -> Vec<UnresolvedStruct> {
    let span = tracing::info_span!("extract");
    let _guard = span.enter();
    let mut structs = Vec::new();
    for st in entity.types.iter() {
        if let RsEntity::Struct(x) = st {
            structs.append(&mut extract_structs(x, settings, None));
        }
    }
    structs
}

fn extract_enum(entity: &RsEntity) -> Option<&Enum> {
    match entity {
        RsEntity::Enum(x) => Some(x),
        _ => None,
    }
}

fn extract_tuple_struct(entity: &RsEntity) -> Option<&TupleStruct> {
    match entity {
        RsEntity::TupleStruct(x) => Some(x),
        _ => None,
    }
}

fn convert_restricted_enum(en: &Enum, settings: &Settings) -> Enumeration {
    if !en.subtypes.is_empty() {
        panic!(
            "Restriction-based enum {} has {} subtypes",
            en.name,
            en.subtypes.len()
        )
    }

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
    if !en.subtypes.is_empty() {
        tracing::warn!(
            "Choice-based enum {} has {} ignored subtypes",
            en.name,
            en.subtypes.len()
        )
    }

    let mut variants: Vec<UnresolvedChoiceVariant> = Vec::new();
    for v in en.cases.iter() {
        let name = v
            .type_name
            .as_ref()
            .expect("choice case must include a type name");
        variants.push(UnresolvedChoiceVariant {
            comment: v.comment.clone(),
            element_name: name.clone(),
            type_id: TypeId::parse(&name, settings.namespace),
        });
    }
    UnresolvedChoice {
        type_id: TypeId::parse(&en.name, settings.namespace),
        comment: en.comment.clone(),
        variants,
    }
}

fn extract_enum_types(model: &RsFile, settings: &Settings) -> Vec<Enumeration> {
    let mut ret: Vec<Enumeration> = Default::default();
    for en in model
        .types
        .iter()
        .filter_map(extract_enum)
        .filter(|p| p.source == EnumSource::Restriction)
    {
        ret.push(convert_restricted_enum(en, settings));
    }
    ret
}

fn extract_choice_types(model: &RsFile, settings: &Settings) -> Vec<UnresolvedChoice> {
    let mut ret: Vec<UnresolvedChoice> = Default::default();
    for en in model
        .types
        .iter()
        .filter_map(extract_enum)
        .filter(|p| p.source == EnumSource::Choice)
    {
        ret.push(convert_choice_enum(en, settings));
    }
    ret
}

fn extract_simple_types_from_tuple_structs(
    model: &RsFile,
    settings: &Settings,
) -> Vec<(TypeId, SimpleType)> {
    let mut output: Vec<(TypeId, SimpleType)> = Vec::new();
    let tuple_structs: Vec<TupleStruct> = model
        .types
        .iter()
        .filter_map(extract_tuple_struct)
        .cloned()
        .collect();
    for ts in tuple_structs.iter() {
        let (type_id, st) = match try_resolve_basic(ts) {
            None => {
                match tuple_structs.iter().find(|s| ts.type_name == s.name) {
                    None => {
                        panic!("Unknown simple type: {} in {}", ts.type_name, ts.name)
                    }
                    Some(base) => {
                        // try to resolve by going 1 level down
                        let resolved = try_resolve_basic(base).unwrap();
                        (TypeId::parse(&ts.name, settings.namespace), resolved)
                    }
                }
            }
            Some(st) => (TypeId::parse(&ts.name, settings.namespace), st),
        };
        output.push((type_id, st));
    }
    output
}

// simple types can only reference each other
fn resolve_simple_types(model: &RsFile, settings: &Settings) -> Map<TypeId, SimpleType> {
    let span = tracing::info_span!("resolve-simple-types");
    let _guard = span.enter();

    let mut output: Map<TypeId, SimpleType> = Map::new();

    for en in extract_enum_types(model, settings) {
        tracing::info!("enum: {}", en.type_id);
        output.insert(
            en.type_id.clone(),
            SimpleType::Wrapper(WrapperType::Enum(Rc::new(en))),
        );
    }

    for (name, st) in extract_simple_types_from_tuple_structs(model, settings) {
        tracing::info!("tuple struct '{}' is {:?}", name, st);
        output.insert(name, st);
    }

    output
}

fn parse_numeric_type<T>(ts: &TupleStruct) -> NumericConstraint<T>
where
    T: FromStr + Default,
    T::Err: Debug,
{
    let mut constraint = NumericConstraint::default();
    for facet in ts.facets.iter() {
        match &facet.facet_type {
            FacetType::MaxInclusive(s) => {
                constraint.max = Some(s.parse::<T>().unwrap());
            }
            FacetType::MinInclusive(s) => {
                constraint.min = Some(s.parse::<T>().unwrap());
            }
            x => panic!("Unsupported {} facet: {:?}", std::any::type_name::<T>(), x),
        }
    }
    constraint
}

fn parse_string_constraints(ts: &TupleStruct) -> StringConstraints {
    let mut constraint = StringConstraints::default();
    for facet in ts.facets.iter() {
        match &facet.facet_type {
            FacetType::MaxLength(x) => constraint.max_length = Some(x.parse::<usize>().unwrap()),
            _ => tracing::warn!(
                "ignoring unsupported string facet: {:?} in  {}",
                facet.facet_type,
                ts.name
            ),
        }
    }
    constraint
}

fn parse_hex_binary(ts: &TupleStruct) -> PrimitiveType {
    // 2030.5 only has one of these with a max length == 1
    if ts.facets.len() != 1 {
        panic!(
            "Unexpected # of facets for xs:hexBinary: {}",
            ts.facets.len()
        );
    }
    match &ts.facets[0].facet_type {
        FacetType::MaxLength(x) => match x.parse::<usize>().unwrap() {
            1 => PrimitiveType::HexByte,
            len => PrimitiveType::HexBytes(Some(len)).into(),
        },
        ft => panic!("Unexpected Facet type for xs:hexBinary: {:?}", ft),
    }
}

fn try_resolve_basic(ts: &TupleStruct) -> Option<SimpleType> {
    let type_name = match ts.type_name.split_once(':') {
        Some(("xs", name)) => name,
        _ => return None,
    };

    match type_name {
        "token" | "string" | "normalizedString" | "anyURI" => {
            Some(PrimitiveType::String(parse_string_constraints(ts)).into())
        }
        "dateTime" => Some(PrimitiveType::String(parse_string_constraints(ts)).into()), // TODO - use chrono
        "hexBinary" => Some(parse_hex_binary(ts).into()),
        "byte" => Some(NumericType::I8(parse_numeric_type::<i8>(ts)).into()),
        "unsignedByte" => Some(NumericType::U8(parse_numeric_type::<u8>(ts)).into()),
        "short" => Some(NumericType::I16(parse_numeric_type::<i16>(ts)).into()),
        "unsignedShort" => Some(NumericType::U16(parse_numeric_type::<u16>(ts)).into()),
        "int" => Some(NumericType::I32(parse_numeric_type::<i32>(ts)).into()),
        "unsignedInt" => Some(NumericType::U32(parse_numeric_type::<u32>(ts)).into()),
        "long" => Some(NumericType::I64(parse_numeric_type::<i64>(ts)).into()),
        "unsignedLong" => Some(NumericType::U64(parse_numeric_type::<u64>(ts)).into()),
        "float" => Some(NumericType::F32(parse_numeric_type::<f32>(ts)).into()),
        "double" => Some(NumericType::F64(parse_numeric_type::<f64>(ts)).into()),
        _ => None,
    }
}

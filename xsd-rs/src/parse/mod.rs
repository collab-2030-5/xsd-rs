mod parser;

use parser::types::{RsEntity, RsFile, StructFieldSource, TupleStruct, TypeModifier};
use parser::xsd_elements::FacetType;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::str::FromStr;

use xml_model::unresolved::*;
use xml_model::{Namespace, NumericConstraint, SimpleType, StringConstraint};

/// Parse the XML and transform it into an UnresolvedModel
pub fn transform(xml: &str) -> UnresolvedModel {
    //  parse using the underlying library
    let entity = crate::parse::parser::parse(xml).unwrap();

    let mut simple_types = resolve_simple_types(&entity);
    let mut structs = extract_structs(&entity);

    // find struct that are mis-classified - some are just inherited from basic types
    let base_structs: HashMap<String, SimpleType> = structs
        .iter()
        .filter_map(|x| match &x.base_type {
            Some(bt) => simple_types.get(bt).map(|st| (x.name.clone(), st.clone())),
            None => None,
        })
        .collect();

    // remove these structs from the struct list
    structs.retain(|f| !base_structs.contains_key(&f.name));

    // add these aliases to the simple types list
    for (k, v) in base_structs.iter() {
        simple_types.insert(k.clone(), v.clone());
    }

    UnresolvedModel {
        xsd_ns: entity.xsd_ns.map(|x| Namespace {
            name: x.name().map(|x| x.to_string()),
            uri: x.uri().to_string(),
        }),
        target_ns: entity.target_ns.map(|x| Namespace {
            name: x.name().map(|x| x.to_string()),
            uri: x.uri().to_string(),
        }),
        simple_types,
        structs,
    }
}

fn extract_base_type(x: &super::parse::parser::types::Struct) -> Option<String> {
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
        [x] => Some(x.clone()),
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
            TypeModifier::Empty => {
                unimplemented!()
            }
        })
        .collect();

    match modifiers.as_slice() {
        [] => None,
        [x] => Some(*x),
        _ => panic!("Unexpected field modifier count: {:#?}", modifiers),
    }
}

fn extract_fields(st: &super::parse::parser::types::Struct) -> Vec<UnresolvedField> {
    st.fields
        .borrow()
        .iter()
        .filter_map(|x| match x.source {
            StructFieldSource::Attribute => Some(UnresolvedField {
                comment: x.comment.clone(),
                name: x.name.clone(),
                field_type: x.type_name.clone(),
                info: FieldTypeInfo::Attribute(get_attribute_type(&x.type_modifiers)),
            }),
            StructFieldSource::Element => {
                let element_type = match get_element_type(&x.type_modifiers) {
                    None => {
                        //println!("In {} element {} has no modifiers, assuming single", st.name, x.name);
                        //println!("info : {:#?}", x);
                        ElementType::Single
                    }
                    Some(x) => x,
                };

                Some(UnresolvedField {
                    comment: x.comment.clone(),
                    name: x.name.clone(),
                    field_type: x.type_name.clone(),
                    info: FieldTypeInfo::Element(element_type),
                })
            }
            StructFieldSource::Base => None,
            StructFieldSource::Choice => unimplemented!(),
            StructFieldSource::NA => unimplemented!(),
        })
        .collect()
}

fn extract_structs(entity: &RsFile) -> Vec<UnresolvedStruct> {
    let mut structs = Vec::new();
    for st in entity.types.iter() {
        if let RsEntity::Struct(x) = st {
            let base_type = extract_base_type(x);
            let fields = extract_fields(x);
            structs.push(UnresolvedStruct {
                comment: x.comment.clone(),
                name: x.name.clone(),
                base_type,
                fields,
            })
        }
    }
    structs
}

// simple types can only reference each other
fn resolve_simple_types(model: &RsFile) -> BTreeMap<String, SimpleType> {
    let input: Vec<TupleStruct> = model
        .types
        .iter()
        .filter_map(|x| match x {
            RsEntity::TupleStruct(ts) => Some(ts),
            _ => None,
        })
        .cloned()
        .collect();

    let mut output = BTreeMap::new();

    for ts in input.iter() {
        match try_resolve_basic(ts) {
            None => {
                match input.iter().find(|s| ts.type_name == s.name) {
                    None => {
                        panic!("Unknown simple type: {} in {}", ts.type_name, ts.name)
                    }
                    Some(base) => {
                        // try to resolve by going 1 level down
                        let resolved = try_resolve_basic(base).unwrap();
                        output.insert(ts.name.clone(), resolved);
                    }
                }
            }
            Some(st) => {
                output.insert(ts.name.clone(), st);
            }
        }
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

fn parse_string_type(ts: &TupleStruct) -> StringConstraint {
    let mut constraint = StringConstraint::default();
    for facet in ts.facets.iter() {
        match &facet.facet_type {
            FacetType::MaxLength(x) => constraint.max_length = Some(x.parse::<usize>().unwrap()),
            _ => panic!("Unsupported string facet: {:?}", facet),
        }
    }
    constraint
}

fn try_resolve_basic(ts: &TupleStruct) -> Option<SimpleType> {
    match ts.type_name.as_str() {
        "xs:hexBinary" => {
            // 2030.5 only has one of these with a max length == 1
            if ts.facets.len() != 1 {
                panic!(
                    "Unexpected # of facets for xs:hexBinary: {}",
                    ts.facets.len()
                );
            }
            match &ts.facets[0].facet_type {
                FacetType::MaxLength(x) => match x.parse::<usize>().unwrap() {
                    1 => Some(SimpleType::HexByte),
                    len => Some(SimpleType::HexBytes(Some(len))),
                },
                ft => panic!("Unexpected Facet type for xs:hexBinary: {:?}", ft),
            }
        }
        "xs:string" => Some(SimpleType::String(parse_string_type(ts))),
        "xs:byte" => Some(SimpleType::I8(parse_numeric_type::<i8>(ts))),
        "xs:unsignedByte" => Some(SimpleType::U8(parse_numeric_type::<u8>(ts))),
        "xs:short" => Some(SimpleType::I16(parse_numeric_type::<i16>(ts))),
        "xs:unsignedShort" => Some(SimpleType::U16(parse_numeric_type::<u16>(ts))),
        "xs:int" => Some(SimpleType::I32(parse_numeric_type::<i32>(ts))),
        "xs:unsignedInt" => Some(SimpleType::U32(parse_numeric_type::<u32>(ts))),
        "xs:long" => Some(SimpleType::I64(parse_numeric_type::<i64>(ts))),
        "xs:unsignedLong" => Some(SimpleType::U64(parse_numeric_type::<u64>(ts))),
        _ => None,
    }
}

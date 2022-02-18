use crate::parser::types::{RsEntity, TupleStruct};
use crate::parser::xsd_elements::FacetType;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Model {
    pub simple_types: HashMap<String, SimpleType>,
    pub structs: Vec<Struct>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct StringConstraint {
    pub max_length: Option<usize>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct NumericConstraint<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

// maps to simple types with possible constraints
#[derive(Copy, Clone, Debug)]
pub enum SimpleType {
    // a single byte encoded as a hex (2 characters e.g. "FF")
    HexByte,
    // multiple bytes with a maximum length
    HexBytes(usize),
    String(StringConstraint),
    I8(NumericConstraint<i8>),
    U8(NumericConstraint<u8>),
    I16(NumericConstraint<i16>),
    U16(NumericConstraint<u16>),
    I32(NumericConstraint<i32>),
    U32(NumericConstraint<u32>),
    I64(NumericConstraint<i64>),
    U64(NumericConstraint<u64>),
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug)]
pub struct Struct {
    /// single optional base struct
    pub base: Option<String>,
    pub fields: Vec<StructField>,
}

pub(crate) mod parser;

pub fn parse(path: &str) -> Model {
    //  parse using the underlying library
    let entity = parser::parse(path).unwrap();

    let mut simple_types: Vec<TupleStruct> = Vec::new();

    // transform into the model types
    for t in entity.types.iter() {
        match t {
            RsEntity::Struct(_) => {}
            RsEntity::StructField(_) => {
                unimplemented!()
            }
            RsEntity::TupleStruct(x) => {
                simple_types.push(x.clone());
            }
            RsEntity::Enum(_) => {
                unimplemented!()
            }
            RsEntity::EnumCase(_) => {
                unimplemented!()
            }
            RsEntity::Alias(_) => {}
            RsEntity::Import(_) => {
                unimplemented!()
            }
        }
    }

    let simple_types = resolve_simple_types(simple_types);

    Model {
        simple_types,
        structs: Vec::new(),
    }
}

// simple types can only reference each other
fn resolve_simple_types(input: Vec<TupleStruct>) -> HashMap<String, SimpleType> {
    let mut output = HashMap::new();

    for ts in input.iter() {
        match try_resolve_basic(ts) {
            None => {
                match input.iter().find(|s| s.name == ts.name) {
                    None => {
                        panic!("Unknown simple type: {} in {}", ts.type_name, ts.name)
                    }
                    Some(_base) => {
                        //println!("{} resolved to {:?}", ts.name, base)
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
                    len => Some(SimpleType::HexBytes(len)),
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

use std::collections::HashMap;

use crate::config::{Config, NumericEnum};
use crate::resolved::{AttrMultiplicity, ElemMultiplicity, Field, FieldType, Metadata, Struct};
use crate::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedModel {
    pub xsd_ns: Option<Namespace>,
    pub target_ns: Option<Namespace>,
    pub simple_types: HashMap<String, SimpleType>,
    pub structs: Vec<UnresolvedStruct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedField {
    pub comment: Option<String>,
    pub name: String,
    pub field_type: String,
    pub info: FieldTypeInfo,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ElementType {
    Single,
    Array,
    Option,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum AttributeType {
    Single,
    Option,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FieldTypeInfo {
    Attribute(AttributeType),
    Element(ElementType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedStruct {
    pub comment: Option<String>,
    pub name: String,
    /// single optional base struct
    pub base_type: Option<String>,
    pub fields: Vec<UnresolvedField>,
}

fn resolve_basic(name: &str) -> Option<SimpleType> {
    match name {
        "xs:boolean" => Some(SimpleType::Boolean),
        "xs:anyURI" => Some(SimpleType::String(StringConstraint::default())),
        "xs:hexBinary" => Some(SimpleType::HexBytes(None)),
        "xs:string" => Some(SimpleType::String(StringConstraint::default())),
        "xs:byte" => Some(SimpleType::I8(NumericConstraint::default())),
        "xs:unsignedByte" => Some(SimpleType::U8(NumericConstraint::default())),
        "xs:short" => Some(SimpleType::I16(NumericConstraint::default())),
        "xs:unsignedShort" => Some(SimpleType::U16(NumericConstraint::default())),
        "xs:int" => Some(SimpleType::I32(NumericConstraint::default())),
        "xs:unsignedInt" => Some(SimpleType::U32(NumericConstraint::default())),
        "xs:long" => Some(SimpleType::I64(NumericConstraint::default())),
        "xs:unsignedLong" => Some(SimpleType::U64(NumericConstraint::default())),
        _ => {
            if name.starts_with("xs:") {
                panic!("unhandled primitive: {}", name);
            }
            None
        }
    }
}

fn get_simple_field_type(info: FieldTypeInfo, st: SimpleType) -> FieldType {
    match info {
        FieldTypeInfo::Attribute(x) => match x {
            AttributeType::Single => FieldType::Attribute(AttrMultiplicity::Single, st),
            AttributeType::Option => FieldType::Attribute(AttrMultiplicity::Optional, st),
        },
        FieldTypeInfo::Element(x) => match x {
            ElementType::Single => FieldType::Element(ElemMultiplicity::Single, st.into()),
            ElementType::Array => FieldType::Element(ElemMultiplicity::Vec, st.into()),
            ElementType::Option => FieldType::Element(ElemMultiplicity::Optional, st.into()),
        },
    }
}

fn get_struct_field_type(info: FieldTypeInfo, st: Rc<crate::resolved::Struct>) -> FieldType {
    match info {
        FieldTypeInfo::Attribute(_) => panic!("attributes cannot use struct type"),
        FieldTypeInfo::Element(x) => match x {
            ElementType::Single => FieldType::Element(ElemMultiplicity::Single, st.into()),
            ElementType::Array => FieldType::Element(ElemMultiplicity::Vec, st.into()),
            ElementType::Option => FieldType::Element(ElemMultiplicity::Optional, st.into()),
        },
    }
}

impl UnresolvedField {
    fn resolve(
        &self,
        structs: &HashMap<String, Rc<Struct>>,
        simple_types: &HashMap<String, SimpleType>,
    ) -> Option<Field> {
        // first try to resolve as a simple type
        if let Some(x) = resolve_basic(&self.field_type) {
            return Some(Field {
                comment: self.comment.clone(),
                name: self.name.clone(),
                field_type: get_simple_field_type(self.info, x),
            });
        }

        // then try to resolve as an alias for a simple type
        if let Some(x) = simple_types.get(&self.field_type) {
            return Some(Field {
                comment: self.comment.clone(),
                name: self.name.clone(),
                field_type: get_simple_field_type(self.info, x.clone()),
            });
        }

        // finally try to resolve as a struct!
        if let Some(x) = structs.get(&self.field_type) {
            return Some(Field {
                comment: self.comment.clone(),
                name: self.name.clone(),
                field_type: get_struct_field_type(self.info, x.clone()),
            });
        }

        None
    }
}

impl UnresolvedStruct {
    fn resolve(
        &self,
        metadata: Metadata,
        structs: &HashMap<String, Rc<Struct>>,
        simple_types: &HashMap<String, SimpleType>,
    ) -> Option<std::rc::Rc<Struct>> {
        // resolve the base class
        let base_type = if let Some(base) = &self.base_type {
            match structs.get(base) {
                None => {
                    // base class isn't resolved yet, can't resolve this class
                    return None;
                }
                Some(x) => Some(x.clone()),
            }
        } else {
            None
        };

        // resolve the fields
        let mut fields: Vec<Field> = Vec::new();
        for field in self.fields.iter() {
            match field.resolve(structs, simple_types) {
                None => {
                    // can't resolve field yet
                    return None;
                }
                Some(x) => fields.push(x),
            }
        }

        Some(Rc::new(Struct {
            comment: self.comment.clone(),
            name: self.name.clone(),
            base_type,
            fields,
            metadata,
        }))
    }
}

impl UnresolvedModel {
    fn is_base(&self, name: &str) -> bool {
        for other in self.structs.iter() {
            if let Some(other) = &other.base_type {
                if other.as_str() == name {
                    return true;
                }
            }
        }
        false
    }

    fn used_as_field(&self, name: &str) -> bool {
        for other in self.structs.iter() {
            for field in other.fields.iter() {
                if field.field_type.as_str() == name {
                    return true;
                }
            }
        }
        false
    }

    pub fn compute_metadata(&self) -> HashMap<String, Metadata> {
        let mut meta_map: HashMap<String, Metadata> = HashMap::new();

        for st in self.structs.iter() {
            let metadata = Metadata {
                is_base: self.is_base(&st.name),
                use_as_element: self.used_as_field(&st.name),
            };
            meta_map.insert(st.name.clone(), metadata);
        }

        meta_map
    }

    pub fn resolve(mut self, config: Config) -> crate::resolved::Model {
        let enums: Vec<Rc<NumericEnum<u8>>> = config.mappings.iter().map(|x| x.1.clone()).collect();

        // first do type substitution
        for (type_name, substitute) in config.mappings {
            match self.simple_types.get_mut(&type_name) {
                None => {
                    panic!("No substitute found for type: {}", type_name);
                }
                Some(x) => {
                    *x = SimpleType::EnumU8(substitute);
                }
            }
        }

        let mut input: HashMap<String, UnresolvedStruct> = self
            .structs
            .iter()
            .map(|x| (x.name.clone(), x.clone()))
            .collect();

        // compute some metadata about the structs
        let meta_map = self.compute_metadata();

        let mut output: HashMap<String, Rc<Struct>> = HashMap::new();

        loop {
            if input.is_empty() {
                return crate::resolved::Model {
                    enums,
                    target_ns: self.target_ns.clone(),
                    structs: output.values().cloned().collect(),
                };
            }

            if let Some(x) = input.iter().find_map(|(_, v)| {
                // lookup the metadata
                let metadata = *meta_map.get(&v.name).unwrap();

                v.resolve(metadata, &output, &self.simple_types)
            }) {
                input.remove(&x.name);
                output.insert(x.name.clone(), x);
            } else {
                panic!("cannot resolve anything else");
            }
        }
    }
}

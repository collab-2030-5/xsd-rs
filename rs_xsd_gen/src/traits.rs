use heck::ToUpperCamelCase;
use xml_model::resolved::{AttrMultiplicity, ElemMultiplicity, ElementType, FieldType};
use xml_model::SimpleType;

pub(crate) trait RustType {
    fn rust_struct_type(&self) -> String;
}

impl RustType for SimpleType {
    fn rust_struct_type(&self) -> String {
        match self {
            Self::Boolean => "bool".to_string(),
            Self::HexByte => "u8".to_string(),
            Self::HexBytes(_) => "Vec<u8>".to_string(),
            Self::String(_) => "String".to_string(),
            Self::I8(_) => "i8".to_string(),
            Self::U8(_) => "u8".to_string(),
            Self::I16(_) => "i16".to_string(),
            Self::U16(_) => "u16".to_string(),
            Self::I32(_) => "i32".to_string(),
            Self::U32(_) => "u32".to_string(),
            Self::I64(_) => "i64".to_string(),
            Self::U64(_) => "u64".to_string(),
            Self::EnumU8(x) => format!("structs::{}", x.name.to_upper_camel_case()),
        }
    }
}

impl RustType for ElementType {
    fn rust_struct_type(&self) -> String {
        match self {
            ElementType::Simple(x) => x.rust_struct_type(),
            ElementType::Struct(x) => {
                if x.metadata.is_base {
                    format!("base::{}", x.name.to_upper_camel_case())
                } else {
                    format!("structs::{}", x.name.to_upper_camel_case())
                }
            }
        }
    }
}

impl RustType for FieldType {
    fn rust_struct_type(&self) -> String {
        match self {
            FieldType::Element(m, t) => match m {
                ElemMultiplicity::Optional => format!("Option<{}>", t.rust_struct_type()),
                ElemMultiplicity::Single => t.rust_struct_type(),
                ElemMultiplicity::Vec => format!("Vec<{}>", t.rust_struct_type()),
            },
            FieldType::Attribute(m, t) => match m {
                AttrMultiplicity::Single => t.rust_struct_type(),
                AttrMultiplicity::Optional => format!("Option<{}>", t.rust_struct_type()),
            },
        }
    }
}

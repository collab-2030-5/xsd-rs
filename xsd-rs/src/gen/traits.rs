use heck::ToUpperCamelCase;
use std::borrow::Cow;
use xml_model::resolved::{AnyType, AttrMultiplicity, ElemMultiplicity, FieldType};
use xml_model::{NumericType, PrimitiveType, SimpleType, TypeId, WrapperType};

use heck::ToSnakeCase;

pub(crate) trait RustType {
    fn rust_struct_type(&self) -> Cow<'static, str>;
}

pub(crate) trait RustFieldName {
    fn rust_field_name(&self) -> String;
}

impl<T> RustFieldName for T
where
    T: AsRef<str>,
{
    fn rust_field_name(&self) -> String {
        let snake = self.as_ref().to_snake_case();
        match snake.as_str() {
            // have to rename reserved identifiers
            "type" => "typ".to_string(),
            _ => snake,
        }
    }
}

impl RustType for NumericType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            Self::I8(_) => "i8".into(),
            Self::U8(_) => "u8".into(),
            Self::I16(_) => "i16".into(),
            Self::U16(_) => "u16".into(),
            Self::I32(_) => "i32".into(),
            Self::U32(_) => "u32".into(),
            Self::I64(_) => "i64".into(),
            Self::U64(_) => "u64".into(),
            Self::F32(_) => "f32".into(),
            Self::F64(_) => "f64".into(),
        }
    }
}

impl RustType for PrimitiveType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            Self::Boolean => "bool".into(),
            Self::HexByte => "u8".into(),
            Self::HexBytes(_) => "Vec<u8>".into(),
            Self::String(_) => "String".into(),
            Self::Number(x) => x.rust_struct_type(),
            Self::NumericDuration(_) => "std::time::Duration".into(),
        }
    }
}

impl RustType for WrapperType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            Self::EnumU8(x) => format!("structs::{}", x.name.to_upper_camel_case()).into(),
            Self::Enum(x) => format!("enums::{}", x.type_id.name.to_upper_camel_case()).into(),
            Self::NamedArray(x) => format!("structs::{}", x.name).into(),
            Self::HexBitField(x) => format!("structs::{}", x.name).into(),
        }
    }
}

impl RustType for SimpleType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            Self::Primitive(x) => x.rust_struct_type(),
            Self::Wrapper(x) => x.rust_struct_type(),
        }
    }
}

fn fully_qualified_name(id: &TypeId) -> String {
    format!(
        "crate::types::{}::{}",
        id.ns.to_snake_case(),
        id.name.to_upper_camel_case()
    )
}

impl RustType for AnyType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            AnyType::Simple(x) => x.rust_struct_type(),
            AnyType::Struct(x) => {
                if x.metadata.is_base {
                    format!("base::{}", x.id.name.to_upper_camel_case()).into()
                } else {
                    fully_qualified_name(&x.id).into()
                }
            }
            AnyType::Choice(x) => fully_qualified_name(&x.id).into(),
        }
    }
}

impl RustType for FieldType {
    fn rust_struct_type(&self) -> Cow<'static, str> {
        match self {
            FieldType::Element(m, t) => match m {
                ElemMultiplicity::Optional => format!("Option<{}>", t.rust_struct_type()).into(),
                ElemMultiplicity::Single => t.rust_struct_type(),
                ElemMultiplicity::Vec => format!("Vec<{}>", t.rust_struct_type()).into(),
            },
            FieldType::Attribute(m, t) => match m {
                AttrMultiplicity::Single => t.rust_struct_type(),
                AttrMultiplicity::Optional => format!("Option<{}>", t.rust_struct_type()).into(),
            },
        }
    }
}
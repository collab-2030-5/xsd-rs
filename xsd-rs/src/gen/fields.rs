use crate::gen::traits::fully_qualified_name;
use heck::ToUpperCamelCase;
use xsd_model::config::{DurationEncoding, NumericDuration};
use xsd_model::resolved::{AnyType, Struct};
use xsd_model::{PrimitiveType, SimpleType, WrapperType};

pub(crate) trait ElementTransforms {
    fn read_transform(&self, elem_name: &str) -> String;
}

impl ElementTransforms for AnyType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            AnyType::Simple(x) => x.read_transform(elem_name),
            AnyType::Struct(x) => x.read_transform(elem_name),
            AnyType::Choice(_) => unimplemented!(),
        }
    }
}

impl ElementTransforms for Struct {
    fn read_transform(&self, elem_name: &str) -> String {
        if self.metadata.is_base {
            format!(
                "base::{}::read(reader, &attributes, \"{}\")?",
                self.id.name.to_upper_camel_case(),
                elem_name
            )
        } else {
            let name = fully_qualified_name(&self.id);
            format!("{}::read(reader, &attributes, \"{}\")?", name, elem_name)
        }
    }
}

impl ElementTransforms for SimpleType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            SimpleType::Primitive(x) => x.read_transform(elem_name),
            SimpleType::Wrapper(x) => x.read_transform(elem_name),
        }
    }
}

impl ElementTransforms for PrimitiveType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            PrimitiveType::Boolean | PrimitiveType::Number(_) => {
                format!(
                    "xsd_util::read_type_from_string(reader, \"{}\")?",
                    elem_name
                )
            }
            PrimitiveType::HexBytes(_) => {
                format!("xsd_util::read_hex_bytes(reader, \"{}\")?", elem_name)
            }
            PrimitiveType::String(_) => {
                format!("xsd_util::read_string(reader, \"{}\")?", elem_name)
            }
            PrimitiveType::NumericDuration(x) => match x {
                NumericDuration::Seconds(x) => match x {
                    DurationEncoding::UInt32 => {
                        format!(
                            "xsd_util::read_duration_secs_u32(reader, \"{}\")?)",
                            elem_name
                        )
                    }
                },
            },
        }
    }
}

impl ElementTransforms for WrapperType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            WrapperType::Enum(_) => {
                format!("xsd_util::read_string_enum(reader, \"{}\")?", elem_name)
            }
            WrapperType::EnumU8(_, _) => unimplemented!(),
            WrapperType::NamedArray(_, _) => unimplemented!(),
            WrapperType::HexBitField(_, _) => unimplemented!(),
        }
    }
}

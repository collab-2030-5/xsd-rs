use crate::RustType;
use xsd_model::config::{DurationEncoding, NumericDuration};
use xsd_model::resolved::{AnyType, Choice, Struct};
use xsd_model::{PrimitiveType, SimpleType, WrapperType};

pub(crate) trait ElementTransforms {
    fn read_transform(&self, elem_name: &str) -> String;
    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String;
}

impl ElementTransforms for AnyType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            AnyType::Simple(x) => x.read_transform(elem_name),
            AnyType::Struct(x) => x.read_transform(elem_name),
            AnyType::Choice(x) => x.read_transform(elem_name),
        }
    }

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        match self {
            AnyType::Simple(x) => x.write_transform(rust_field_name, xsd_field_name),
            AnyType::Struct(x) => x.write_transform(rust_field_name, xsd_field_name),
            AnyType::Choice(x) => x.write_transform(rust_field_name, xsd_field_name),
        }
    }
}

impl ElementTransforms for Struct {
    fn read_transform(&self, elem_name: &str) -> String {
        format!(
            "{}::read(reader, &attributes, \"{}\")?",
            self.rust_struct_type(),
            elem_name
        )
    }

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        format!(
            "{}.write_with_name(writer, \"{}\", false, false)?;",
            rust_field_name, xsd_field_name
        )
    }
}

impl ElementTransforms for Choice {
    fn read_transform(&self, elem_name: &str) -> String {
        format!(
            "{}::read(reader, &attributes, \"{}\")?",
            self.rust_struct_type(),
            elem_name
        )
    }

    fn write_transform(&self, rust_field_name: &str, _xsd_field_name: &str) -> String {
        format!("{}.write(writer)?;", rust_field_name)
    }
}

impl ElementTransforms for SimpleType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            SimpleType::Primitive(x) => x.read_transform(elem_name),
            SimpleType::Wrapper(x) => x.read_transform(elem_name),
        }
    }

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        match self {
            SimpleType::Primitive(x) => x.write_transform(rust_field_name, xsd_field_name),
            SimpleType::Wrapper(x) => x.write_transform(rust_field_name, xsd_field_name),
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

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        match self {
            PrimitiveType::Boolean | PrimitiveType::Number(_) => {
                format!(
                    "xsd_util::write_element_using_to_string(writer, \"{}\", {})?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::HexBytes(_) => {
                format!(
                    "xsd_util::write_hex_tag(writer, \"{}\", &{})?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::String(_) => {
                format!(
                    "xsd_util::write_simple_element(writer, \"{}\", {}.as_str())?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::NumericDuration(x) => match x {
                NumericDuration::Seconds(_) => {
                    format!(
                        "xsd_util::write_duration_as_seconds(writer, \"{}\", {})?;",
                        xsd_field_name, rust_field_name
                    )
                }
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

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        match self {
            WrapperType::Enum(_) => {
                format!(
                    "xsd_util::write_string_enumeration(writer, \"{}\", {})?;",
                    xsd_field_name, rust_field_name
                )
            }
            WrapperType::EnumU8(_, _) => unimplemented!(),
            WrapperType::NamedArray(_, _) => unimplemented!(),
            WrapperType::HexBitField(_, _) => unimplemented!(),
        }
    }
}

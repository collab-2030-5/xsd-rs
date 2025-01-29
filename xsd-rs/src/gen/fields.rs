use crate::RustType;
use xsd_model::config::{DurationEncoding, NumericDuration};
use xsd_model::resolved::{AnyType, Choice, Struct};
use xsd_model::{PrimitiveType, SimpleType, WrapperType};

pub(crate) trait ElementTransforms {
    fn read_transform(&self, elem_name: &str) -> String;
    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String;
    fn str_transform(&self, var_name: &str) -> String;
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

    fn str_transform(&self, var_name: &str) -> String {
        match self {
            AnyType::Simple(x) => x.str_transform(var_name),
            _ => unimplemented!(),
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

    fn str_transform(&self, var_name: &str) -> String {
        unimplemented!("Structs should not be converted to strings")
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

    fn str_transform(&self, _var_name: &str) -> String {
        unimplemented!("Choices should not be converted to strings")
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

    fn str_transform(&self, var_name: &str) -> String {
        match self {
            SimpleType::Primitive(x) => x.str_transform(var_name),
            SimpleType::Wrapper(x) => x.str_transform(var_name),
        }
    }
}

impl ElementTransforms for PrimitiveType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            PrimitiveType::Boolean | PrimitiveType::Number(_) => {
                format!(
                    "crate::xsd_util::read_type_from_string(reader, \"{}\")?",
                    elem_name
                )
            }
            PrimitiveType::HexBytes(_) => {
                format!(
                    "crate::xsd_util::read_hex_bytes(reader, \"{}\")?",
                    elem_name
                )
            }
            PrimitiveType::String(_) => {
                format!("crate::xsd_util::read_string(reader, \"{}\")?", elem_name)
            }
            PrimitiveType::NumericDuration(x) => match x {
                NumericDuration::Seconds(x) => match x {
                    DurationEncoding::UInt32 => {
                        format!(
                            "crate::xsd_util::read_duration_secs_u32(reader, \"{}\")?)",
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
                    "crate::xsd_util::write_element_using_to_string(writer, \"{}\", {})?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::HexBytes(_) => {
                format!(
                    "crate::xsd_util::write_hex_tag(writer, \"{}\", &{})?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::String(_) => {
                format!(
                    "crate::xsd_util::write_simple_element(writer, \"{}\", {}.as_str())?;",
                    xsd_field_name, rust_field_name
                )
            }
            PrimitiveType::NumericDuration(x) => match x {
                NumericDuration::Seconds(_) => {
                    format!(
                        "crate::xsd_util::write_duration_as_seconds(writer, \"{}\", {})?;",
                        xsd_field_name, rust_field_name
                    )
                }
            },
        }
    }

    fn str_transform(&self, var_name: &str) -> String {
        match self {
            PrimitiveType::String(_) => {
                format!("{}.as_str()", var_name)
            }
            _ => unimplemented!(),
        }
    }
}

impl ElementTransforms for WrapperType {
    fn read_transform(&self, elem_name: &str) -> String {
        match self {
            WrapperType::Enum(_) | WrapperType::UnionChoice(_, _) => {
                format!(
                    "crate::xsd_util::read_string_enum(reader, \"{}\")?",
                    elem_name
                )
            }
            WrapperType::EnumU8(_, _) => unimplemented!(),
            WrapperType::NamedArray(_, _) => unimplemented!(),
            WrapperType::HexBitField(_, _) => unimplemented!(),
        }
    }

    fn write_transform(&self, rust_field_name: &str, xsd_field_name: &str) -> String {
        match self {
            WrapperType::Enum(_) | WrapperType::UnionChoice(_, _) => {
                let rust_field_name = match rust_field_name.starts_with("self.") {
                    true => format!("&{}", rust_field_name),
                    false => rust_field_name.to_string(),
                };

                format!(
                    "crate::xsd_util::write_string_enumeration(writer, \"{}\", {})?;",
                    xsd_field_name, rust_field_name
                )
            }
            WrapperType::EnumU8(_, _) => unimplemented!(),
            WrapperType::NamedArray(_, _) => unimplemented!(),
            WrapperType::HexBitField(_, _) => unimplemented!(),
        }
    }

    fn str_transform(&self, var_name: &str) -> String {
        match self {
            WrapperType::UnionChoice(_, _) => format!("{}.to_str()", var_name),
            WrapperType::Enum(_) => format!("{}.to_str()", var_name),
            _ => unimplemented!("str_transform {:#?}", self),
        }
    }
}

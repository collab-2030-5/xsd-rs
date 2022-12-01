use heck::ToUpperCamelCase;
use std::io::Write;
use xsd_model::resolved::Choice;

use crate::gen::fields::ElementTransforms;
use crate::gen::{indent, write_comment};
use crate::{FatalError, RustType};

pub(crate) fn write(w: &mut dyn Write, choice: &Choice) -> Result<(), FatalError> {
    writeln!(w, "use xml::writer::*;")?;
    writeln!(w)?;
    write_definition(w, choice)?;
    writeln!(w)?;
    write_impl(w, choice)?;
    Ok(())
}

fn write_definition(w: &mut dyn Write, choice: &Choice) -> Result<(), std::io::Error> {
    write_comment(w, &choice.comment)?;
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub enum {} {{", choice.id.name.to_upper_camel_case())?;
    indent(w, |w| {
        for var in choice.variants.iter() {
            write_comment(w, &var.comment)?;
            writeln!(
                w,
                "{}({}),",
                var.element_name.to_upper_camel_case(),
                var.type_info.rust_struct_type()
            )?;
        }
        Ok(())
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_impl(w: &mut dyn Write, choice: &Choice) -> Result<(), std::io::Error> {
    writeln!(w, "impl {} {{", choice.id.name.to_upper_camel_case())?;
    indent(w, |w| {
        write_serializer(w, choice)?;
        writeln!(w)?;
        write_deserializer(w, choice)?;
        Ok(())
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_serializer(w: &mut dyn Write, choice: &Choice) -> Result<(), std::io::Error> {
    writeln!(w, "pub(crate) fn write<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
    indent(w, |w| {
        writeln!(w, "match self {{")?;
        indent(w, |w| {
            for var in choice.variants.iter() {
                writeln!(
                    w,
                    "{}::{}(x) => {{",
                    choice.id.name.to_upper_camel_case(),
                    var.element_name.to_upper_camel_case()
                )?;
                indent(w, |w| {
                    let tx = var.type_info.write_transform("x", &var.element_name);
                    writeln!(w, "{}", tx)?;
                    Ok(())
                })?;
                writeln!(w, "}}")?;
            }
            Ok(())
        })?;
        writeln!(w, "}}")?;
        writeln!(w, "Ok(())")?;
        Ok(())
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_deserializer(w: &mut dyn Write, _choice: &Choice) -> Result<(), std::io::Error> {
    writeln!(w, "pub(crate) fn read<R>(_reader: &mut xml::reader::EventReader<R>) -> core::result::Result<Self, xsd_api::ReadError> where R: std::io::Read {{")?;
    indent(w, |w| {
        writeln!(w, "unimplemented!()")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

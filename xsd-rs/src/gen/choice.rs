use heck::ToUpperCamelCase;
use std::io::Write;
use xsd_model::resolved::Choice;

use crate::gen::{indent, write_comment};
use crate::{FatalError, RustType};

pub(crate) fn write(w: &mut dyn Write, choice: &Choice) -> Result<(), FatalError> {
    writeln!(w, "use xml::writer::*;")?;
    writeln!(w, "use xml::common::Position;")?;
    writeln!(w)?;
    write_definition(w, choice)?;
    Ok(())
}

fn write_definition(w: &mut dyn Write, choice: &Choice) -> Result<(), FatalError> {
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

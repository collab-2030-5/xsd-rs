use heck::ToUpperCamelCase;
use std::io::Write;
use xsd_model::resolved::Choice;

use crate::gen::indent;
use crate::{FatalError, RustType};

pub(crate) fn write(w: &mut dyn Write, choice: &Choice) -> Result<(), FatalError> {
    writeln!(w, "use xml::writer::*;")?;
    writeln!(w, "use xml::common::Position;")?;
    writeln!(w)?;
    write_definition(w, choice)?;
    Ok(())
}

fn write_definition(w: &mut dyn Write, choice: &Choice) -> Result<(), FatalError> {
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub enum {} {{", choice.id.name.to_upper_camel_case())?;
    indent(w, |w| writeln!(w, "// variants"))?;
    writeln!(w, "}}")?;
    Ok(())
}

use std::io::Write;

use crate::FatalError;
use heck::ToUpperCamelCase;
use xsd_model::Enumeration;

use crate::gen::{indent, write_comment};

pub(crate) fn write(w: &mut dyn Write, en: &Enumeration) -> Result<(), FatalError> {
    write_definition(w, en)?;
    writeln!(w)?;
    write_trait_impl(w, en)?;
    Ok(())
}

fn write_definition(w: &mut dyn Write, en: &Enumeration) -> std::io::Result<()> {
    writeln!(w, "#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]")?;
    write_comment(w, &en.comment)?;
    writeln!(w, "pub enum {} {{", en.type_id.name.to_upper_camel_case())?;
    indent(w, |w| {
        for var in en.variants.iter() {
            write_comment(w, &var.comment)?;
            writeln!(w, "{},", var.name)?;
        }
        Ok(())
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_from_str(w: &mut dyn Write, en: &Enumeration) -> std::io::Result<()> {
    writeln!(w, "fn find(s: &str) -> Option<Self>{{")?;
    indent(w, |w| {
        writeln!(w, "match s {{")?;
        indent(w, |w| {
            for var in en.variants.iter() {
                writeln!(w, "\"{}\" => Some(Self::{}),", var.value, var.name)?;
            }
            writeln!(w, "_ => None")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_to_str(w: &mut dyn Write, en: &Enumeration) -> std::io::Result<()> {
    writeln!(w, "fn to_str(self) -> &'static str {{")?;
    indent(w, |w| {
        writeln!(w, "match self {{")?;
        indent(w, |w| {
            for var in en.variants.iter() {
                writeln!(w, "Self::{} => \"{}\",", var.name, var.value)?;
            }
            Ok(())
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_trait_impl(w: &mut dyn Write, en: &Enumeration) -> std::io::Result<()> {
    writeln!(
        w,
        "impl xsd_util::StringEnumeration for {} {{",
        en.type_id.name.to_upper_camel_case()
    )?;
    indent(w, |w| {
        write_from_str(w, en)?;
        writeln!(w)?;
        write_to_str(w, en)?;
        Ok(())
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

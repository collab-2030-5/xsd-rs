use std::io::Write;
use xsd_model::config::NumericEnum;

use crate::gen::*;
use crate::FatalError;

pub(crate) fn write(w: &mut dyn Write, e: &NumericEnum<u8>) -> Result<(), FatalError> {
    writeln!(w, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]")?;
    writeln!(w, "pub enum {} {{", e.name)?;
    indent(w, |w| {
        for value in e.variants.values() {
            write_comment(w, &value.comment)?;
            writeln!(w, "{},", value.name)?;
        }
        writeln!(w, "/// Value not defined in the XSD")?;
        writeln!(w, "Other(u8)")
    })?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl {} {{", e.name)?;
    indent(w, |w| {
        writeln!(w, "pub fn value(&self) -> u8 {{")?;
        indent(w, |w| {
            writeln!(w, "match self {{")?;
            indent(w, |w| {
                for (value, var) in e.variants.iter() {
                    writeln!(w, "Self::{} => {},", var.name, value)?;
                }
                writeln!(w, "Self::Other(x) => *x,")?;
                Ok(())
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;

        writeln!(w)?;

        writeln!(w, "pub fn from_value(value: u8) -> Self {{")?;
        indent(w, |w| {
            writeln!(w, "match value {{")?;
            indent(w, |w| {
                for (value, var) in e.variants.iter() {
                    writeln!(w, "{} => Self::{},", value, var.name)?;
                }
                writeln!(w, "_ => Self::Other(value),")?;
                Ok(())
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

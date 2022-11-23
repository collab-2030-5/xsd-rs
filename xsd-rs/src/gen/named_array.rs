use std::io::Write;
use xsd_model::config::NamedArray;

use crate::gen::indent;
use crate::FatalError;

pub(crate) fn write(w: &mut dyn Write, na: &NamedArray) -> Result<(), FatalError> {
    writeln!(w, "#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]")?;
    writeln!(w, "pub struct {} {{", na.name)?;
    indent(w, |w| writeln!(w, "pub(crate) inner: [u8; Self::SIZE],"))?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl {} {{", na.name)?;
    indent(w, |w| writeln!(w, "pub const SIZE: usize = {};", na.size))?;
    writeln!(w, "}}")?;
    Ok(())
}

use std::io::Write;
use xml_model::config::BitField;

use crate::gen::*;
use crate::FatalError;

pub(crate) fn write(w: &mut dyn Write, bf: &BitField) -> Result<(), FatalError> {
    write_comment(w, &bf.comment)?;
    writeln!(
        w,
        "#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]"
    )?;
    writeln!(w, "pub struct {} {{", bf.name)?;
    indent(w, |w| {
        for (num, byte) in bf.bytes.iter().enumerate() {
            writeln!(w, "// --- Byte #{} ---", num)?;
            writeln!(w)?;
            for (x, bit) in byte.iter() {
                writeln!(w, "// --- mask 0b{0:08b} ---", x)?;
                writeln!(w, "/// {}", bit.comment)?;
                writeln!(w, "pub {}: bool,", bit.name)?;
            }
        }
        Ok(())
    })?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl {} {{", bf.name)?;
    indent(w, |w| {
        writeln!(
            w,
            "pub(crate) fn from_hex(hex: &str) -> Result<Self, crate::ReadError> {{"
        )?;
        indent(w, |w| {
            writeln!(
                w,
                "let bytes = crate::parse_fixed_hex_bytes::<{}>(hex)?;",
                bf.bytes.len()
            )?;
            writeln!(w)?;
            writeln!(w, "let mut value: Self = Default::default();")?;
            writeln!(w)?;
            for (num, byte) in bf.bytes.iter().enumerate() {
                for (mask, bit) in byte.iter() {
                    writeln!(w, "if bytes[{}] & 0b{:08b} != 0 {{", num, mask)?;
                    indent(w, |w| writeln!(w, "value.{} = true;", bit.name))?;
                    writeln!(w, "}}")?;
                }
            }
            writeln!(w)?;
            writeln!(w, "Ok(value)")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        writeln!(w, "pub(crate) fn to_hex(&self) -> String {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let mut bytes : [u8; {}] = [0; {}];",
                bf.bytes.len(),
                bf.bytes.len()
            )?;
            writeln!(w)?;
            for (num, byte) in bf.bytes.iter().enumerate() {
                for (mask, bit) in byte.iter() {
                    writeln!(w, "if self.{} {{", bit.name)?;
                    indent(w, |w| writeln!(w, "bytes[{}] |= 0b{:08b};", num, mask))?;
                    writeln!(w, "}}")?;
                }
            }
            writeln!(w)?;
            writeln!(w, "crate::to_hex(&bytes)")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

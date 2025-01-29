use crate::xsd_util::StringEnumeration;
use xml::writer::*;

/// Signal name.
#[derive(Debug, Clone, PartialEq)]
pub enum SignalNameType {
    SignalNameEnumeratedType(crate::oadr20b::ei::SignalNameEnumeratedType),
    EiExtensionTokenType(String),
}

impl SignalNameType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            SignalNameType::SignalNameEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:signalName", x)?;
            }
            SignalNameType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:signalName", x.as_str())?;
            }
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        match self {
            SignalNameType::SignalNameEnumeratedType(x) => x.to_str(),
            SignalNameType::EiExtensionTokenType(x) => x.as_str(),
        }
    }
}

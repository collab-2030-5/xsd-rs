use xml::writer::*;

/// Reason for opting.
#[derive(Debug, Clone, PartialEq)]
pub enum OptReasonType {
    OptReasonEnumeratedType(crate::oadr20b::ei::OptReasonEnumeratedType),
    EiExtensionTokenType(String),
}

impl OptReasonType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            OptReasonType::OptReasonEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:optReason", x)?;
            }
            OptReasonType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:optReason", x.as_str())?;
            }
        }
        Ok(())
    }
}

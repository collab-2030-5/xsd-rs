use xml::writer::*;

/// An enumerated value that gives the type of report being provided.
#[derive(Debug, Clone, PartialEq)]
pub enum ReportTypeType {
    ReportEnumeratedType(crate::oadr20b::ei::ReportEnumeratedType),
    EiExtensionTokenType(String),
}

impl ReportTypeType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            ReportTypeType::ReportEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:reportType", x)?;
            }
            ReportTypeType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:reportType", x.as_str())?;
            }
        }
        Ok(())
    }
}

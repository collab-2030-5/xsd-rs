use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ReportNameType {
    ReportNameEnumeratedType(crate::oadr20b::ei::ReportNameEnumeratedType),
    EiExtensionTokenType(String),
}

impl ReportNameType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            ReportNameType::ReportNameEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:reportName", x)?;
            }
            ReportNameType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:reportName", x.as_str())?;
            }
        }
        Ok(())
    }
}

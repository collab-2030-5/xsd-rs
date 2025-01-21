use xml::writer::*;

/// Type of Reading.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadingTypeType {
    ReadingTypeEnumeratedType(crate::oadr20b::ei::ReadingTypeEnumeratedType),
    EiExtensionTokenType(String),
}

impl ReadingTypeType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            ReadingTypeType::ReadingTypeEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:readingType", x)?;
            }
            ReadingTypeType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:readingType", x.as_str())?;
            }
        }
        Ok(())
    }
}

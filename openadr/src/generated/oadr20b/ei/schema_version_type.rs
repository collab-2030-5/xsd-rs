use crate::xsd_util::StringEnumeration;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaVersionType {
    SchemaVersionEnumeratedType(crate::oadr20b::ei::SchemaVersionEnumeratedType),
    EiExtensionTokenType(String),
}

impl SchemaVersionType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            SchemaVersionType::SchemaVersionEnumeratedType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "ei:schemaVersion", x)?;
            }
            SchemaVersionType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "ei:schemaVersion", x.as_str())?;
            }
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        match self {
            SchemaVersionType::SchemaVersionEnumeratedType(x) => x.to_str(),
            SchemaVersionType::EiExtensionTokenType(x) => x.as_str(),
        }
    }
}

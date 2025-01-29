use crate::xsd_util::StringEnumeration;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum OadrDataQualityTypeType {
    OadrDataQualityType(crate::oadr20b::oadr::OadrDataQualityType),
    EiExtensionTokenType(String),
}

impl OadrDataQualityTypeType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            OadrDataQualityTypeType::OadrDataQualityType(x) => {
                crate::xsd_util::write_string_enumeration(writer, "oadr:oadrDataQuality", x)?;
            }
            OadrDataQualityTypeType::EiExtensionTokenType(x) => {
                crate::xsd_util::write_simple_element(writer, "oadr:oadrDataQuality", x.as_str())?;
            }
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        match self {
            OadrDataQualityTypeType::OadrDataQualityType(x) => x.to_str(),
            OadrDataQualityTypeType::EiExtensionTokenType(x) => x.as_str(),
        }
    }
}

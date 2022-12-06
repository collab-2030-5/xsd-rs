use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum X509DataTypeChoice {
    DsX509IssuerSerialType(crate::ds::X509IssuerSerialType),
    XsBase64Binary(String),
    XsString(String),
    XsBase64Binary(String),
    XsBase64Binary(String),
}

impl X509DataTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            X509DataTypeChoice::DsX509IssuerSerialType(x) => {
                x.write_with_name(writer, "ds:X509IssuerSerialType", false, false)?;
            }
            X509DataTypeChoice::XsBase64Binary(x) => {
                xsd_util::write_simple_element(writer, "xs:base64Binary", x.as_str())?;
            }
            X509DataTypeChoice::XsString(x) => {
                xsd_util::write_simple_element(writer, "xs:string", x.as_str())?;
            }
            X509DataTypeChoice::XsBase64Binary(x) => {
                xsd_util::write_simple_element(writer, "xs:base64Binary", x.as_str())?;
            }
            X509DataTypeChoice::XsBase64Binary(x) => {
                xsd_util::write_simple_element(writer, "xs:base64Binary", x.as_str())?;
            }
        }
        Ok(())
    }

    pub(crate) fn read<R>(
        _reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        unimplemented!()
    }
}

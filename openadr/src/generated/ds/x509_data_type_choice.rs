use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum X509DataTypeChoice {
    X509IssuerSerial(crate::ds::X509IssuerSerialType),
    X509ski(String),
    X509SubjectName(String),
    X509Certificate(String),
    X509crl(String),
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
            X509DataTypeChoice::X509IssuerSerial(x) => {
                x.write_with_name(writer, "X509IssuerSerial", false, false)?;
            }
            X509DataTypeChoice::X509ski(x) => {
                xsd_util::write_simple_element(writer, "X509SKI", x.as_str())?;
            }
            X509DataTypeChoice::X509SubjectName(x) => {
                xsd_util::write_simple_element(writer, "X509SubjectName", x.as_str())?;
            }
            X509DataTypeChoice::X509Certificate(x) => {
                xsd_util::write_simple_element(writer, "X509Certificate", x.as_str())?;
            }
            X509DataTypeChoice::X509crl(x) => {
                xsd_util::write_simple_element(writer, "X509CRL", x.as_str())?;
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

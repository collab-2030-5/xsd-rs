use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyInfoTypeChoice {
    DsKeyName(String),
    DsKeyValue(crate::ds::KeyValueType),
    DsRetrievalMethod(crate::ds::RetrievalMethodType),
    DsX509Data(crate::ds::X509DataType),
    DsPgpData(crate::ds::PgpDataType),
    DsSpkiData(crate::ds::SpkiDataType),
    DsMgmtData(String),
}

impl KeyInfoTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            KeyInfoTypeChoice::DsKeyName(x) => {
                xsd_util::write_simple_element(writer, "ds:KeyName", x.as_str())?;
            }
            KeyInfoTypeChoice::DsKeyValue(x) => {
                x.write_with_name(writer, "ds:KeyValue", false, false)?;
            }
            KeyInfoTypeChoice::DsRetrievalMethod(x) => {
                x.write_with_name(writer, "ds:RetrievalMethod", false, false)?;
            }
            KeyInfoTypeChoice::DsX509Data(x) => {
                x.write_with_name(writer, "ds:X509Data", false, false)?;
            }
            KeyInfoTypeChoice::DsPgpData(x) => {
                x.write_with_name(writer, "ds:PGPData", false, false)?;
            }
            KeyInfoTypeChoice::DsSpkiData(x) => {
                x.write_with_name(writer, "ds:SPKIData", false, false)?;
            }
            KeyInfoTypeChoice::DsMgmtData(x) => {
                xsd_util::write_simple_element(writer, "ds:MgmtData", x.as_str())?;
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

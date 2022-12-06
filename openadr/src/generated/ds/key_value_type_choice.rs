use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyValueTypeChoice {
    DsDsaKeyValue(crate::ds::DsaKeyValueType),
    DsRsaKeyValue(crate::ds::RsaKeyValueType),
}

impl KeyValueTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            KeyValueTypeChoice::DsDsaKeyValue(x) => {
                x.write_with_name(writer, "ds:DSAKeyValue", false, false)?;
            }
            KeyValueTypeChoice::DsRsaKeyValue(x) => {
                x.write_with_name(writer, "ds:RSAKeyValue", false, false)?;
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

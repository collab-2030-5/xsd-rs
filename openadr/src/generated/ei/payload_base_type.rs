use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadBaseType {
    OadrPayloadResourceStatus(crate::oadr::OadrPayloadResourceStatusType),
    PayloadFloat(crate::ei::PayloadFloatType),
}

impl PayloadBaseType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            PayloadBaseType::OadrPayloadResourceStatus(x) => {
                x.write_with_name(writer, "oadrPayloadResourceStatus", false, false)?;
            }
            PayloadBaseType::PayloadFloat(x) => {
                x.write_with_name(writer, "payloadFloat", false, false)?;
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

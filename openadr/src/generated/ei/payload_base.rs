use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadBase {
    OadrPayloadResourceStatusType(crate::oadr::OadrPayloadResourceStatusType),
    PayloadFloatType(crate::ei::PayloadFloatType),
}

impl PayloadBase {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            PayloadBase::OadrPayloadResourceStatusType(x) => {
                x.write_with_name(writer, "oadrPayloadResourceStatusType", false, false)?;
            }
            PayloadBase::PayloadFloatType(x) => {
                x.write_with_name(writer, "PayloadFloatType", false, false)?;
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

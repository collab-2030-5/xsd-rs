use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StreamPayloadBaseType {
    OadrReportPayloadType(crate::oadr::OadrReportPayloadType),
    SignalPayloadType(crate::ei::SignalPayloadType),
}

impl StreamPayloadBaseType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            StreamPayloadBaseType::OadrReportPayloadType(x) => {
                x.write_with_name(writer, "oadrReportPayloadType", false, false)?;
            }
            StreamPayloadBaseType::SignalPayloadType(x) => {
                x.write_with_name(writer, "signalPayloadType", false, false)?;
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

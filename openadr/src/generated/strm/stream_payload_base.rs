use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StreamPayloadBase {
    SignalPayloadType(crate::ei::SignalPayloadType),
    OadrReportPayloadType(crate::oadr::OadrReportPayloadType),
}

impl StreamPayloadBase {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            StreamPayloadBase::SignalPayloadType(x) => {
                x.write_with_name(writer, "signalPayloadType", false, false)?;
            }
            StreamPayloadBase::OadrReportPayloadType(x) => {
                x.write_with_name(writer, "oadrReportPayloadType", false, false)?;
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

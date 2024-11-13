use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StreamPayloadBaseType {
    OadrReportPayload(crate::oadr20b::oadr::OadrReportPayloadType),
    SignalPayload(crate::oadr20b::ei::SignalPayloadType),
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
            StreamPayloadBaseType::OadrReportPayload(x) => {
                x.write_with_name(writer, "oadr:oadrReportPayload", false, false)?;
            }
            StreamPayloadBaseType::SignalPayload(x) => {
                x.write_with_name(writer, "ei:signalPayload", false, false)?;
            }
        }
        Ok(())
    }
}

use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StreamPayloadBaseType {
    SignalPayload(crate::oadr20a::ei::SignalPayloadType),
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
            StreamPayloadBaseType::SignalPayload(x) => {
                x.write_with_name(writer, "ei:signalPayload", false, false)?;
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

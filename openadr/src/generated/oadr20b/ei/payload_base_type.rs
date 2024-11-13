use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadBaseType {
    OadrPayloadResourceStatus(crate::oadr20b::oadr::OadrPayloadResourceStatusType),
    PayloadFloat(crate::oadr20b::ei::PayloadFloatType),
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
                x.write_with_name(writer, "oadr:oadrPayloadResourceStatus", false, false)?;
            }
            PayloadBaseType::PayloadFloat(x) => {
                x.write_with_name(writer, "ei:payloadFloat", false, false)?;
            }
        }
        Ok(())
    }
}

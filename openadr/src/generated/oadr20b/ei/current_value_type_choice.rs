use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentValueTypeChoice {
    EiPayloadFloat(crate::oadr20b::ei::PayloadFloatType),
}

impl CurrentValueTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            CurrentValueTypeChoice::EiPayloadFloat(x) => {
                x.write_with_name(writer, "ei:payloadFloat", false, false)?;
            }
        }
        Ok(())
    }
}

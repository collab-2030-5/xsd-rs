use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentValueTypeChoice {
    EiPayloadFloat(crate::ei::PayloadFloatType),
}

impl CurrentValueTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
        name: &str,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        writer.write(events::XmlEvent::start_element(name))?;
        match self {
            CurrentValueTypeChoice::EiPayloadFloat(x) => {
                x.write_with_name(writer, "ei:payloadFloat", false, false)?;
            }
        }
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

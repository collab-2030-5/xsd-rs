use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiEventSignalType {
    pub strm_intervals: crate::oadr20a::strm::Intervals,
    pub signal_name: String,
    pub ei_signal_type: crate::oadr20a::ei::SignalTypeEnumeratedType,
    pub signal_id: String,
    pub ei_current_value: crate::oadr20a::ei::CurrentValueType,
}

impl EiEventSignalType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.strm_intervals
            .write_with_name(writer, "strm:intervals", false, false)?;
        xsd_util::write_simple_element(writer, "ei:signalName", self.signal_name.as_str())?;
        xsd_util::write_string_enumeration(writer, "ei:signalType", self.ei_signal_type)?;
        xsd_util::write_simple_element(writer, "ei:signalID", self.signal_id.as_str())?;
        self.ei_current_value
            .write_with_name(writer, "ei:currentValue", false, false)?;
        Ok(())
    }

    pub(crate) fn write_with_name<W>(
        &self,
        writer: &mut EventWriter<W>,
        name: &str,
        top_level: bool,
        write_type: bool,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        let start = if top_level {
            super::add_schema_attr(events::XmlEvent::start_element(name))
        } else {
            events::XmlEvent::start_element(name)
        };
        let start = if write_type {
            start.attr("xsi:type", "eiEventSignalType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EiEventSignalType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eiEventSignal", true, false)?;
        Ok(())
    }
}

impl EiEventSignalType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut strm_intervals: xsd_util::SetOnce<crate::oadr20a::strm::Intervals> =
            Default::default();
        let mut signal_name: xsd_util::SetOnce<String> = Default::default();
        let mut ei_signal_type: xsd_util::SetOnce<crate::oadr20a::ei::SignalTypeEnumeratedType> =
            Default::default();
        let mut signal_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_current_value: xsd_util::SetOnce<crate::oadr20a::ei::CurrentValueType> =
            Default::default();

        loop {
            match reader.next()? {
                xml::reader::XmlEvent::EndElement { name } => {
                    if name.local_name.as_str() == parent_tag {
                        // try to construct struct
                        break;
                    } else {
                        // TODO - make this more specific
                        return Err(xsd_api::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "intervals" => strm_intervals.set(crate::oadr20a::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
                    "signalName" => {
                        signal_name.set(xsd_util::read_string(reader, "signalName")?)?
                    }
                    "signalType" => {
                        ei_signal_type.set(xsd_util::read_string_enum(reader, "signalType")?)?
                    }
                    "signalID" => signal_id.set(xsd_util::read_string(reader, "signalID")?)?,
                    "currentValue" => {
                        ei_current_value.set(crate::oadr20a::ei::CurrentValueType::read(
                            reader,
                            &attributes,
                            "currentValue",
                        )?)?
                    }
                    name => {
                        return Err(xsd_api::ReadError::UnexpectedToken(
                            xsd_api::ParentToken(parent_tag.to_owned()),
                            xsd_api::ChildToken(name.to_owned()),
                        ))
                    }
                },
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::EndDocument => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::Characters(_) => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::ProcessingInstruction { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(EiEventSignalType {
            strm_intervals: strm_intervals.require()?,
            signal_name: signal_name.require()?,
            ei_signal_type: ei_signal_type.require()?,
            signal_id: signal_id.require()?,
            ei_current_value: ei_current_value.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "eiEventSignal")?;
        EiEventSignalType::read(reader, &attr, "eiEventSignal")
    }
}

impl xsd_api::ReadXml for EiEventSignalType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiEventSignalType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(xsd_api::ErrorWithLocation {
                    err,
                    line: pos.row,
                    col: pos.column,
                })
            }
        }
    }
}

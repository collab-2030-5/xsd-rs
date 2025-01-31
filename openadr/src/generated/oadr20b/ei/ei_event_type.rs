use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EiEventType {
    pub ei_event_descriptor: crate::oadr20b::ei::EventDescriptorType,
    pub ei_ei_active_period: crate::oadr20b::ei::EiActivePeriodType,
    pub ei_ei_event_signals: crate::oadr20b::ei::EiEventSignalsType,
    pub ei_ei_target: crate::oadr20b::ei::EiTargetType,
}

impl EiEventType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.ei_event_descriptor
            .write_with_name(writer, "ei:eventDescriptor", false, false)?;
        self.ei_ei_active_period
            .write_with_name(writer, "ei:eiActivePeriod", false, false)?;
        self.ei_ei_event_signals
            .write_with_name(writer, "ei:eiEventSignals", false, false)?;
        self.ei_ei_target
            .write_with_name(writer, "ei:eiTarget", false, false)?;
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
            start.attr("xsi:type", "eiEventType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for EiEventType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eiEvent", true, false)?;
        Ok(())
    }
}

impl EiEventType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_event_descriptor: crate::xsd_util::SetOnce<
            crate::oadr20b::ei::EventDescriptorType,
        > = Default::default();
        let mut ei_ei_active_period: crate::xsd_util::SetOnce<
            crate::oadr20b::ei::EiActivePeriodType,
        > = Default::default();
        let mut ei_ei_event_signals: crate::xsd_util::SetOnce<
            crate::oadr20b::ei::EiEventSignalsType,
        > = Default::default();
        let mut ei_ei_target: crate::xsd_util::SetOnce<crate::oadr20b::ei::EiTargetType> =
            Default::default();

        loop {
            match reader.next()? {
                xml::reader::XmlEvent::EndElement { name } => {
                    if name.local_name.as_str() == parent_tag {
                        // try to construct struct
                        break;
                    } else {
                        // TODO - make this more specific
                        return Err(crate::xsd_util::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "eventDescriptor" => {
                        ei_event_descriptor.set(crate::oadr20b::ei::EventDescriptorType::read(
                            reader,
                            &attributes,
                            "eventDescriptor",
                        )?)?
                    }
                    "eiActivePeriod" => {
                        ei_ei_active_period.set(crate::oadr20b::ei::EiActivePeriodType::read(
                            reader,
                            &attributes,
                            "eiActivePeriod",
                        )?)?
                    }
                    "eiEventSignals" => {
                        ei_ei_event_signals.set(crate::oadr20b::ei::EiEventSignalsType::read(
                            reader,
                            &attributes,
                            "eiEventSignals",
                        )?)?
                    }
                    "eiTarget" => ei_ei_target.set(crate::oadr20b::ei::EiTargetType::read(
                        reader,
                        &attributes,
                        "eiTarget",
                    )?)?,
                    name => {
                        return Err(crate::xsd_util::ReadError::UnexpectedToken(
                            crate::xsd_util::ParentToken(parent_tag.to_owned()),
                            crate::xsd_util::ChildToken(name.to_owned()),
                        ))
                    }
                },
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => {
                    return Err(crate::xsd_util::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::EndDocument => {
                    return Err(crate::xsd_util::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::Characters(_) => {
                    return Err(crate::xsd_util::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::ProcessingInstruction { .. } => {
                    return Err(crate::xsd_util::ReadError::UnexpectedEvent)
                }
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(EiEventType {
            ei_event_descriptor: ei_event_descriptor.require()?,
            ei_ei_active_period: ei_ei_active_period.require()?,
            ei_ei_event_signals: ei_ei_event_signals.require()?,
            ei_ei_target: ei_ei_target.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "eiEvent")?;
        EiEventType::read(reader, &attr, "eiEvent")
    }
}

impl crate::xsd_util::ReadXml for EiEventType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiEventType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(crate::xsd_util::ErrorWithLocation {
                    err,
                    line: pos.row,
                    col: pos.column,
                })
            }
        }
    }
}

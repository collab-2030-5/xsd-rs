use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EventDescriptorType {
    pub ei_event_id: String,
    pub ei_modification_number: u32,
    pub priority: Option<u32>,
    pub ei_market_context: crate::oadr20a::ei::EiMarketContext,
    pub created_date_time: String,
    pub ei_event_status: crate::oadr20a::ei::EventStatusEnumeratedType,
    pub test_event: Option<String>,
    pub vtn_comment: Option<String>,
}

impl EventDescriptorType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:eventID", self.ei_event_id.as_str())?;
        xsd_util::write_element_using_to_string(
            writer,
            "ei:modificationNumber",
            self.ei_modification_number,
        )?;
        if let Some(elem) = &self.priority {
            xsd_util::write_element_using_to_string(writer, "ei:priority", elem)?;
        }
        self.ei_market_context
            .write_with_name(writer, "ei:eiMarketContext", false, false)?;
        xsd_util::write_simple_element(
            writer,
            "ei:createdDateTime",
            self.created_date_time.as_str(),
        )?;
        xsd_util::write_string_enumeration(writer, "ei:eventStatus", self.ei_event_status)?;
        if let Some(elem) = &self.test_event {
            xsd_util::write_simple_element(writer, "ei:testEvent", elem.as_str())?;
        }
        if let Some(elem) = &self.vtn_comment {
            xsd_util::write_simple_element(writer, "ei:vtnComment", elem.as_str())?;
        }
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
            start.attr("xsi:type", "eventDescriptorType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EventDescriptorType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eventDescriptor", true, false)?;
        Ok(())
    }
}

impl EventDescriptorType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_event_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_modification_number: xsd_util::SetOnce<u32> = Default::default();
        let mut priority: xsd_util::SetOnce<u32> = Default::default();
        let mut ei_market_context: xsd_util::SetOnce<crate::oadr20a::ei::EiMarketContext> =
            Default::default();
        let mut created_date_time: xsd_util::SetOnce<String> = Default::default();
        let mut ei_event_status: xsd_util::SetOnce<crate::oadr20a::ei::EventStatusEnumeratedType> =
            Default::default();
        let mut test_event: xsd_util::SetOnce<String> = Default::default();
        let mut vtn_comment: xsd_util::SetOnce<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                _ => {} // ignore unknown attributes
            };
        }

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
                    "eventID" => ei_event_id.set(xsd_util::read_string(reader, "eventID")?)?,
                    "modificationNumber" => ei_modification_number.set(
                        xsd_util::read_type_from_string(reader, "modificationNumber")?,
                    )?,
                    "priority" => {
                        priority.set(xsd_util::read_type_from_string(reader, "priority")?)?
                    }
                    "eiMarketContext" => {
                        ei_market_context.set(crate::oadr20a::ei::EiMarketContext::read(
                            reader,
                            &attributes,
                            "eiMarketContext",
                        )?)?
                    }
                    "createdDateTime" => {
                        created_date_time.set(xsd_util::read_string(reader, "createdDateTime")?)?
                    }
                    "eventStatus" => {
                        ei_event_status.set(xsd_util::read_string_enum(reader, "eventStatus")?)?
                    }
                    "testEvent" => test_event.set(xsd_util::read_string(reader, "testEvent")?)?,
                    "vtnComment" => {
                        vtn_comment.set(xsd_util::read_string(reader, "vtnComment")?)?
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
        Ok(EventDescriptorType {
            ei_event_id: ei_event_id.require()?,
            ei_modification_number: ei_modification_number.require()?,
            priority: priority.get(),
            ei_market_context: ei_market_context.require()?,
            created_date_time: created_date_time.require()?,
            ei_event_status: ei_event_status.require()?,
            test_event: test_event.get(),
            vtn_comment: vtn_comment.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "eventDescriptor")?;
        EventDescriptorType::read(reader, &attr, "eventDescriptor")
    }
}

impl xsd_api::ReadXml for EventDescriptorType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EventDescriptorType::read_top_level(&mut reader) {
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

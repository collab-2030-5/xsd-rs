use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiCreatedEvent {
    pub ei_ei_response: crate::oadr20a::ei::EiResponse,
    pub ei_event_responses: Option<crate::oadr20a::ei::EventResponses>,
    pub ei_ven_id: String,
}

impl EiCreatedEvent {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.ei_ei_response
            .write_with_name(writer, "ei:eiResponse", false, false)?;
        if let Some(elem) = &self.ei_event_responses {
            elem.write_with_name(writer, "ei:eventResponses", false, false)?;
        }
        crate::xsd_util::write_simple_element(writer, "ei:venID", self.ei_ven_id.as_str())?;
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
            start.attr("xsi:type", "eiCreatedEvent")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for EiCreatedEvent {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "pyld:eiCreatedEvent", true, false)?;
        Ok(())
    }
}

impl EiCreatedEvent {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_ei_response: crate::xsd_util::SetOnce<crate::oadr20a::ei::EiResponse> =
            Default::default();
        let mut ei_event_responses: crate::xsd_util::SetOnce<crate::oadr20a::ei::EventResponses> =
            Default::default();
        let mut ei_ven_id: crate::xsd_util::SetOnce<String> = Default::default();

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
                    "eiResponse" => ei_ei_response.set(crate::oadr20a::ei::EiResponse::read(
                        reader,
                        &attributes,
                        "eiResponse",
                    )?)?,
                    "eventResponses" => {
                        ei_event_responses.set(crate::oadr20a::ei::EventResponses::read(
                            reader,
                            &attributes,
                            "eventResponses",
                        )?)?
                    }
                    "venID" => ei_ven_id.set(crate::xsd_util::read_string(reader, "venID")?)?,
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
        Ok(EiCreatedEvent {
            ei_ei_response: ei_ei_response.require()?,
            ei_event_responses: ei_event_responses.get(),
            ei_ven_id: ei_ven_id.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "eiCreatedEvent")?;
        EiCreatedEvent::read(reader, &attr, "eiCreatedEvent")
    }
}

impl crate::xsd_util::ReadXml for EiCreatedEvent {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiCreatedEvent::read_top_level(&mut reader) {
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

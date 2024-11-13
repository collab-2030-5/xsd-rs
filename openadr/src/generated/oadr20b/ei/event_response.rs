use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EventResponse {
    pub ei_response_code: String,
    pub ei_response_description: Option<String>,
    pub pyld_request_id: String,
    pub ei_qualified_event_id: crate::ei::QualifiedEventIdType,
    pub ei_opt_type: crate::ei::OptTypeType,
}

impl EventResponse {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:responseCode", self.ei_response_code.as_str())?;
        if let Some(elem) = &self.ei_response_description {
            xsd_util::write_simple_element(writer, "ei:responseDescription", elem.as_str())?;
        }
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        self.ei_qualified_event_id
            .write_with_name(writer, "ei:qualifiedEventID", false, false)?;
        xsd_util::write_string_enumeration(writer, "ei:optType", self.ei_opt_type)?;
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
            start.attr("xsi:type", "eventResponse")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EventResponse {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eventResponse", true, false)?;
        Ok(())
    }
}

impl EventResponse {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_response_code: xsd_util::SetOnce<String> = Default::default();
        let mut ei_response_description: xsd_util::SetOnce<String> = Default::default();
        let mut pyld_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_qualified_event_id: xsd_util::SetOnce<crate::ei::QualifiedEventIdType> =
            Default::default();
        let mut ei_opt_type: xsd_util::SetOnce<crate::ei::OptTypeType> = Default::default();

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
                    "responseCode" => {
                        ei_response_code.set(xsd_util::read_string(reader, "responseCode")?)?
                    }
                    "responseDescription" => ei_response_description
                        .set(xsd_util::read_string(reader, "responseDescription")?)?,
                    "requestID" => {
                        pyld_request_id.set(xsd_util::read_string(reader, "requestID")?)?
                    }
                    "qualifiedEventID" => {
                        ei_qualified_event_id.set(crate::ei::QualifiedEventIdType::read(
                            reader,
                            &attributes,
                            "qualifiedEventID",
                        )?)?
                    }
                    "optType" => ei_opt_type.set(xsd_util::read_string_enum(reader, "optType")?)?,
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
        Ok(EventResponse {
            ei_response_code: ei_response_code.require()?,
            ei_response_description: ei_response_description.get(),
            pyld_request_id: pyld_request_id.require()?,
            ei_qualified_event_id: ei_qualified_event_id.require()?,
            ei_opt_type: ei_opt_type.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "eventResponse")?;
        EventResponse::read(reader, &attr, "eventResponse")
    }
}

impl xsd_api::ReadXml for EventResponse {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EventResponse::read_top_level(&mut reader) {
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

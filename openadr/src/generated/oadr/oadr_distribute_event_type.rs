use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrDistributeEventType {
    pub ei_ei_response: Option<crate::ei::EiResponseType>,
    pub pyld_request_id: String,
    pub ei_vtn_id: String,
    /// An object containing a demand response event
    pub oadr_event: Vec<crate::oadr::OadrEvent>,
    pub ei_schema_version: Option<String>,
}

impl OadrDistributeEventType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        if let Some(elem) = &self.ei_ei_response {
            elem.write_with_name(writer, "ei:eiResponse", false, false)?;
        }
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        xsd_util::write_simple_element(writer, "ei:vtnID", self.ei_vtn_id.as_str())?;
        for item in &self.oadr_event {
            item.write_with_name(writer, "oadrEvent", false, false)?;
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
        // ---- start attributes ----
        let start = match &self.ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadrDistributeEventType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrDistributeEventType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrDistributeEventType", true, false)?;
        Ok(())
    }
}

impl OadrDistributeEventType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_ei_response: xsd_util::SetOnce<crate::ei::EiResponseType> = Default::default();
        let mut pyld_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_vtn_id: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_event: Vec<crate::oadr::OadrEvent> = Default::default();
        let mut ei_schema_version: xsd_util::SetOnce<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "ei:schemaVersion" => ei_schema_version.set(attr.value.clone())?,
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
                    "eiResponse" => ei_ei_response.set(crate::ei::EiResponseType::read(
                        reader,
                        &attributes,
                        "eiResponse",
                    )?)?,
                    "requestID" => {
                        pyld_request_id.set(xsd_util::read_string(reader, "requestID")?)?
                    }
                    "vtnID" => ei_vtn_id.set(xsd_util::read_string(reader, "vtnID")?)?,
                    "oadrEvent" => oadr_event.push(crate::oadr::OadrEvent::read(
                        reader,
                        &attributes,
                        "oadrEvent",
                    )?),
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
        Ok(OadrDistributeEventType {
            ei_ei_response: ei_ei_response.get(),
            pyld_request_id: pyld_request_id.require()?,
            ei_vtn_id: ei_vtn_id.require()?,
            oadr_event,
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrDistributeEventType")?;
        OadrDistributeEventType::read(reader, &attr, "oadrDistributeEventType")
    }
}

impl xsd_api::ReadXml for OadrDistributeEventType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrDistributeEventType::read_top_level(&mut reader) {
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

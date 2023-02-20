use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrCancelReportType {
    pub pyld_request_id: String,
    pub ei_report_request_id: Vec<String>,
    pub pyld_report_to_follow: bool,
    pub ei_ven_id: Option<String>,
    pub ei_schema_version: Option<String>,
}

impl OadrCancelReportType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        for item in &self.ei_report_request_id {
            xsd_util::write_simple_element(writer, "ei:reportRequestID", item.as_str())?;
        }
        xsd_util::write_element_using_to_string(
            writer,
            "pyld:reportToFollow",
            self.pyld_report_to_follow,
        )?;
        if let Some(elem) = &self.ei_ven_id {
            xsd_util::write_simple_element(writer, "ei:venID", elem.as_str())?;
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
            start.attr("xsi:type", "oadr:oadrCancelReportType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrCancelReportType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrCancelReportType", true, false)?;
        Ok(())
    }
}

impl OadrCancelReportType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut pyld_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_request_id: Vec<String> = Default::default();
        let mut pyld_report_to_follow: xsd_util::SetOnce<bool> = Default::default();
        let mut ei_ven_id: xsd_util::SetOnce<String> = Default::default();
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
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "pyld:requestID" => {
                            pyld_request_id.set(xsd_util::read_string(reader, "pyld:requestID")?)?
                        }
                        "ei:reportRequestID" => ei_report_request_id
                            .push(xsd_util::read_string(reader, "ei:reportRequestID")?),
                        "pyld:reportToFollow" => pyld_report_to_follow.set(
                            xsd_util::read_type_from_string(reader, "pyld:reportToFollow")?,
                        )?,
                        "ei:venID" => ei_ven_id.set(xsd_util::read_string(reader, "ei:venID")?)?,
                        _ => return Err(xsd_api::ReadError::UnexpectedEvent),
                    }
                }
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
        Ok(OadrCancelReportType {
            pyld_request_id: pyld_request_id.require()?,
            ei_report_request_id,
            pyld_report_to_follow: pyld_report_to_follow.require()?,
            ei_ven_id: ei_ven_id.get(),
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrCancelReportType")?;
        OadrCancelReportType::read(reader, &attr, "oadr:oadrCancelReportType")
    }
}

impl xsd_api::ReadXml for OadrCancelReportType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCancelReportType::read_top_level(&mut reader) {
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
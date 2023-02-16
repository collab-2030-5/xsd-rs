use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrUpdateReportType {
    pub pyld_request_id: String,
    pub oadr_oadr_report: Vec<crate::oadr::OadrReportType>,
    pub ei_ven_id: Option<String>,
    pub ei_schema_version: Option<String>,
}

impl OadrUpdateReportType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        for item in &self.oadr_oadr_report {
            item.write_with_name(writer, "oadr:oadrReport", false, false)?;
        }
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
            start.attr("xsi:type", "oadr:oadrUpdateReportType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrUpdateReportType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrUpdateReportType", true, false)?;
        Ok(())
    }
}

impl OadrUpdateReportType {
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
        let mut oadr_oadr_report: Vec<crate::oadr::OadrReportType> = Default::default();
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
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "pyld:requestID" => {
                        pyld_request_id.set(xsd_util::read_string(reader, "pyld:requestID")?)?
                    }
                    "oadr:oadrReport" => oadr_oadr_report.push(crate::oadr::OadrReportType::read(
                        reader,
                        &attributes,
                        "oadr:oadrReport",
                    )?),
                    "ei:venID" => ei_ven_id.set(xsd_util::read_string(reader, "ei:venID")?)?,
                    _ => return Err(xsd_api::ReadError::UnexpectedEvent),
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
        Ok(OadrUpdateReportType {
            pyld_request_id: pyld_request_id.require()?,
            oadr_oadr_report,
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
        let attr = xsd_util::read_start_tag(reader, "oadrUpdateReportType")?;
        OadrUpdateReportType::read(reader, &attr, "oadr:oadrUpdateReportType")
    }
}

impl xsd_api::ReadXml for OadrUpdateReportType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrUpdateReportType::read_top_level(&mut reader) {
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

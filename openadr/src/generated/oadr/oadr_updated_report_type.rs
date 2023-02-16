use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrUpdatedReportType {
    pub ei_ei_response: crate::ei::EiResponseType,
    pub oadr_oadr_cancel_report: Option<crate::oadr::OadrCancelReportType>,
    pub ei_ven_id: Option<String>,
    pub ei_schema_version: Option<String>,
}

impl OadrUpdatedReportType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.ei_ei_response
            .write_with_name(writer, "ei:eiResponse", false, false)?;
        if let Some(elem) = &self.oadr_oadr_cancel_report {
            elem.write_with_name(writer, "oadr:oadrCancelReport", false, false)?;
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
            start.attr("xsi:type", "oadr:oadrUpdatedReportType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrUpdatedReportType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrUpdatedReportType", true, false)?;
        Ok(())
    }
}

impl OadrUpdatedReportType {
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
        let mut oadr_oadr_cancel_report: xsd_util::SetOnce<crate::oadr::OadrCancelReportType> =
            Default::default();
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
                    "ei:eiResponse" => ei_ei_response.set(crate::ei::EiResponseType::read(
                        reader,
                        &attributes,
                        "ei:eiResponse",
                    )?)?,
                    "oadr:oadrCancelReport" => {
                        oadr_oadr_cancel_report.set(crate::oadr::OadrCancelReportType::read(
                            reader,
                            &attributes,
                            "oadr:oadrCancelReport",
                        )?)?
                    }
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
        Ok(OadrUpdatedReportType {
            ei_ei_response: ei_ei_response.require()?,
            oadr_oadr_cancel_report: oadr_oadr_cancel_report.get(),
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
        let attr = xsd_util::read_start_tag(reader, "oadrUpdatedReportType")?;
        OadrUpdatedReportType::read(reader, &attr, "oadr:oadrUpdatedReportType")
    }
}

impl xsd_api::ReadXml for OadrUpdatedReportType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrUpdatedReportType::read_top_level(&mut reader) {
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

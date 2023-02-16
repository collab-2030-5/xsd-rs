use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrReportType {
    pub xcal_dtstart: Option<crate::xcal::Dtstart>,
    pub xcal_duration: Option<crate::xcal::DurationPropType>,
    pub strm_intervals: Option<crate::strm::Intervals>,
    /// reference ID to this report.
    pub ei_ei_report_id: Option<String>,
    /// Define data points the implementation is capable of reporting on. Only used in Metadata report
    pub oadr_oadr_report_description: Vec<crate::oadr::OadrReportDescriptionType>,
    /// Reference to the oadrCreateReport request that defined this report.
    pub ei_report_request_id: String,
    /// Reference to Metadata report from which this report was derived.
    pub ei_report_specifier_id: String,
    /// Name possibly for use in a user interface.
    pub ei_report_name: Option<String>,
    pub ei_created_date_time: String,
}

impl OadrReportType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        if let Some(elem) = &self.xcal_dtstart {
            elem.write_with_name(writer, "xcal:dtstart", false, false)?;
        }
        if let Some(elem) = &self.xcal_duration {
            elem.write_with_name(writer, "xcal:duration", false, false)?;
        }
        if let Some(elem) = &self.strm_intervals {
            elem.write_with_name(writer, "strm:intervals", false, false)?;
        }
        if let Some(elem) = &self.ei_ei_report_id {
            xsd_util::write_simple_element(writer, "ei:eiReportID", elem.as_str())?;
        }
        for item in &self.oadr_oadr_report_description {
            item.write_with_name(writer, "oadr:oadrReportDescription", false, false)?;
        }
        xsd_util::write_simple_element(
            writer,
            "ei:reportRequestID",
            self.ei_report_request_id.as_str(),
        )?;
        xsd_util::write_simple_element(
            writer,
            "ei:reportSpecifierID",
            self.ei_report_specifier_id.as_str(),
        )?;
        if let Some(elem) = &self.ei_report_name {
            xsd_util::write_simple_element(writer, "ei:reportName", elem.as_str())?;
        }
        xsd_util::write_simple_element(
            writer,
            "ei:createdDateTime",
            self.ei_created_date_time.as_str(),
        )?;
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
            start.attr("xsi:type", "oadr:oadrReportType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrReportType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrReportType", true, false)?;
        Ok(())
    }
}

impl OadrReportType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut xcal_dtstart: xsd_util::SetOnce<crate::xcal::Dtstart> = Default::default();
        let mut xcal_duration: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();
        let mut strm_intervals: xsd_util::SetOnce<crate::strm::Intervals> = Default::default();
        let mut ei_ei_report_id: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_report_description: Vec<crate::oadr::OadrReportDescriptionType> =
            Default::default();
        let mut ei_report_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_specifier_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_name: xsd_util::SetOnce<String> = Default::default();
        let mut ei_created_date_time: xsd_util::SetOnce<String> = Default::default();

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
                    "xcal:dtstart" => xcal_dtstart.set(crate::xcal::Dtstart::read(
                        reader,
                        &attributes,
                        "xcal:dtstart",
                    )?)?,
                    "xcal:duration" => xcal_duration.set(crate::xcal::DurationPropType::read(
                        reader,
                        &attributes,
                        "xcal:duration",
                    )?)?,
                    "strm:intervals" => strm_intervals.set(crate::strm::Intervals::read(
                        reader,
                        &attributes,
                        "strm:intervals",
                    )?)?,
                    "ei:eiReportID" => {
                        ei_ei_report_id.set(xsd_util::read_string(reader, "ei:eiReportID")?)?
                    }
                    "oadr:oadrReportDescription" => oadr_oadr_report_description.push(
                        crate::oadr::OadrReportDescriptionType::read(
                            reader,
                            &attributes,
                            "oadr:oadrReportDescription",
                        )?,
                    ),
                    "ei:reportRequestID" => ei_report_request_id
                        .set(xsd_util::read_string(reader, "ei:reportRequestID")?)?,
                    "ei:reportSpecifierID" => ei_report_specifier_id
                        .set(xsd_util::read_string(reader, "ei:reportSpecifierID")?)?,
                    "ei:reportName" => {
                        ei_report_name.set(xsd_util::read_string(reader, "ei:reportName")?)?
                    }
                    "ei:createdDateTime" => ei_created_date_time
                        .set(xsd_util::read_string(reader, "ei:createdDateTime")?)?,
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
        Ok(OadrReportType {
            xcal_dtstart: xcal_dtstart.get(),
            xcal_duration: xcal_duration.get(),
            strm_intervals: strm_intervals.get(),
            ei_ei_report_id: ei_ei_report_id.get(),
            oadr_oadr_report_description,
            ei_report_request_id: ei_report_request_id.require()?,
            ei_report_specifier_id: ei_report_specifier_id.require()?,
            ei_report_name: ei_report_name.get(),
            ei_created_date_time: ei_created_date_time.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrReportType")?;
        OadrReportType::read(reader, &attr, "oadr:oadrReportType")
    }
}

impl xsd_api::ReadXml for OadrReportType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrReportType::read_top_level(&mut reader) {
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

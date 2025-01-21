use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrReportType {
    pub xcal_dtstart: Option<crate::oadr20b::xcal::Dtstart>,
    pub xcal_duration: Option<crate::oadr20b::xcal::DurationPropType>,
    pub strm_intervals: Option<crate::oadr20b::strm::Intervals>,
    /// reference ID to this report.
    pub ei_ei_report_id: Option<String>,
    /// Define data points the implementation is capable of reporting on. Only used in Metadata report
    pub oadr_oadr_report_description: Vec<crate::oadr20b::oadr::OadrReportDescriptionType>,
    /// Reference to the oadrCreateReport request that defined this report.
    pub ei_report_request_id: String,
    /// Reference to Metadata report from which this report was derived.
    pub ei_report_specifier_id: String,
    /// Name possibly for use in a user interface.
    pub ei_report_name: Option<crate::oadr20b::ei::ReportNameType>,
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
            crate::xsd_util::write_simple_element(writer, "ei:eiReportID", elem.as_str())?;
        }
        for item in &self.oadr_oadr_report_description {
            item.write_with_name(writer, "oadr:oadrReportDescription", false, false)?;
        }
        crate::xsd_util::write_simple_element(
            writer,
            "ei:reportRequestID",
            self.ei_report_request_id.as_str(),
        )?;
        crate::xsd_util::write_simple_element(
            writer,
            "ei:reportSpecifierID",
            self.ei_report_specifier_id.as_str(),
        )?;
        if let Some(elem) = &self.ei_report_name {
            elem.write(writer)?;
        }
        crate::xsd_util::write_simple_element(
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
            start.attr("xsi:type", "oadrReportType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrReportType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrReport", true, false)?;
        Ok(())
    }
}

impl OadrReportType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut xcal_dtstart: crate::xsd_util::SetOnce<crate::oadr20b::xcal::Dtstart> =
            Default::default();
        let mut xcal_duration: crate::xsd_util::SetOnce<crate::oadr20b::xcal::DurationPropType> =
            Default::default();
        let mut strm_intervals: crate::xsd_util::SetOnce<crate::oadr20b::strm::Intervals> =
            Default::default();
        let mut ei_ei_report_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_report_description: Vec<crate::oadr20b::oadr::OadrReportDescriptionType> =
            Default::default();
        let mut ei_report_request_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_specifier_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_name: crate::xsd_util::SetOnce<crate::oadr20b::ei::ReportNameType> =
            Default::default();
        let mut ei_created_date_time: crate::xsd_util::SetOnce<String> = Default::default();

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
                    "dtstart" => xcal_dtstart.set(crate::oadr20b::xcal::Dtstart::read(
                        reader,
                        &attributes,
                        "dtstart",
                    )?)?,
                    "duration" => {
                        xcal_duration.set(crate::oadr20b::xcal::DurationPropType::read(
                            reader,
                            &attributes,
                            "duration",
                        )?)?
                    }
                    "intervals" => strm_intervals.set(crate::oadr20b::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
                    "eiReportID" => {
                        ei_ei_report_id.set(crate::xsd_util::read_string(reader, "eiReportID")?)?
                    }
                    "oadrReportDescription" => oadr_oadr_report_description.push(
                        crate::oadr20b::oadr::OadrReportDescriptionType::read(
                            reader,
                            &attributes,
                            "oadrReportDescription",
                        )?,
                    ),
                    "reportRequestID" => ei_report_request_id
                        .set(crate::xsd_util::read_string(reader, "reportRequestID")?)?,
                    "reportSpecifierID" => ei_report_specifier_id
                        .set(crate::xsd_util::read_string(reader, "reportSpecifierID")?)?,
                    "reportName" => ei_report_name.set(
                        crate::report_name_type::read_choice_enum(reader, "reportName")?,
                    )?,
                    "createdDateTime" => ei_created_date_time
                        .set(crate::xsd_util::read_string(reader, "createdDateTime")?)?,
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
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrReport")?;
        OadrReportType::read(reader, &attr, "oadrReport")
    }
}

impl crate::xsd_util::ReadXml for OadrReportType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrReportType::read_top_level(&mut reader) {
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

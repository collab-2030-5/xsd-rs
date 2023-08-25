use xml::common::Position;
use xml::writer::*;

/// Parameters that define the content of a Report Stream
#[derive(Debug, Clone, PartialEq)]
pub struct ReportSpecifierType {
    pub ei_report_specifier_id: String,
    /// How frequently the [measurement] is to be recorded.
    pub xcal_granularity: crate::xcal::DurationPropType,
    /// Report back with the Report-To-Date for each passing of this Duration.
    pub report_back_duration: crate::xcal::DurationPropType,
    /// This is the overall period of reporting.
    pub report_interval: Option<crate::xcal::WsCalendarIntervalType>,
    pub ei_specifier_payload: Vec<crate::ei::SpecifierPayloadType>,
}

impl ReportSpecifierType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(
            writer,
            "ei:reportSpecifierID",
            self.ei_report_specifier_id.as_str(),
        )?;
        self.xcal_granularity
            .write_with_name(writer, "xcal:granularity", false, false)?;
        self.report_back_duration
            .write_with_name(writer, "ei:reportBackDuration", false, false)?;
        if let Some(elem) = &self.report_interval {
            elem.write_with_name(writer, "ei:reportInterval", false, false)?;
        }
        for item in &self.ei_specifier_payload {
            item.write_with_name(writer, "ei:specifierPayload", false, false)?;
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
            start.attr("xsi:type", "ReportSpecifierType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for ReportSpecifierType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:reportSpecifier", true, false)?;
        Ok(())
    }
}

impl ReportSpecifierType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_report_specifier_id: xsd_util::SetOnce<String> = Default::default();
        let mut xcal_granularity: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();
        let mut report_back_duration: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();
        let mut report_interval: xsd_util::SetOnce<crate::xcal::WsCalendarIntervalType> =
            Default::default();
        let mut ei_specifier_payload: Vec<crate::ei::SpecifierPayloadType> = Default::default();

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
                    "reportSpecifierID" => ei_report_specifier_id
                        .set(xsd_util::read_string(reader, "reportSpecifierID")?)?,
                    "granularity" => xcal_granularity.set(crate::xcal::DurationPropType::read(
                        reader,
                        &attributes,
                        "granularity",
                    )?)?,
                    "reportBackDuration" => {
                        report_back_duration.set(crate::xcal::DurationPropType::read(
                            reader,
                            &attributes,
                            "reportBackDuration",
                        )?)?
                    }
                    "reportInterval" => {
                        report_interval.set(crate::xcal::WsCalendarIntervalType::read(
                            reader,
                            &attributes,
                            "reportInterval",
                        )?)?
                    }
                    "specifierPayload" => {
                        ei_specifier_payload.push(crate::ei::SpecifierPayloadType::read(
                            reader,
                            &attributes,
                            "specifierPayload",
                        )?)
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
        Ok(ReportSpecifierType {
            ei_report_specifier_id: ei_report_specifier_id.require()?,
            xcal_granularity: xcal_granularity.require()?,
            report_back_duration: report_back_duration.require()?,
            report_interval: report_interval.get(),
            ei_specifier_payload,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "reportSpecifier")?;
        ReportSpecifierType::read(reader, &attr, "reportSpecifier")
    }
}

impl xsd_api::ReadXml for ReportSpecifierType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match ReportSpecifierType::read_top_level(&mut reader) {
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

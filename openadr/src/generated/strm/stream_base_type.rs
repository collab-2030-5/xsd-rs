use xml::common::Position;
use xml::writer::*;

/// abstract base for communication of schedules for signals and observations
#[derive(Debug, Clone, PartialEq)]
pub struct StreamBaseType {
    pub xcal_dtstart: Option<crate::xcal::Dtstart>,
    pub xcal_duration: Option<crate::xcal::DurationPropType>,
    pub strm_intervals: Option<crate::strm::Intervals>,
}

impl StreamBaseType {
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
            start.attr("xsi:type", "strm:StreamBaseType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for StreamBaseType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "strm:StreamBaseType", true, false)?;
        Ok(())
    }
}

impl StreamBaseType {
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
        Ok(StreamBaseType {
            xcal_dtstart: xcal_dtstart.get(),
            xcal_duration: xcal_duration.get(),
            strm_intervals: strm_intervals.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "StreamBaseType")?;
        StreamBaseType::read(reader, &attr, "strm:StreamBaseType")
    }
}

impl xsd_api::ReadXml for StreamBaseType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match StreamBaseType::read_top_level(&mut reader) {
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

use xml::common::Position;
use xml::writer::*;

/// abstract base for communication of schedules for signals and observations
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StreamBaseType {
    /// Indicates when the Designated Interval of the Stream begins. May be inherited from containing artifact.
    pub xcal_dtstart: Option<crate::oadr20a::xcal::Dtstart>,
    /// Indicates the duration inherited by the intervals. May be inherited from containing artifact.
    pub xcal_duration: Option<crate::oadr20a::xcal::DurationPropType>,
    pub strm_intervals: Option<crate::oadr20a::strm::Intervals>,
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
            start.attr("xsi:type", "StreamBaseType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for StreamBaseType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "strm:streamBase", true, false)?;
        Ok(())
    }
}

impl StreamBaseType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut xcal_dtstart: crate::xsd_util::SetOnce<crate::oadr20a::xcal::Dtstart> =
            Default::default();
        let mut xcal_duration: crate::xsd_util::SetOnce<crate::oadr20a::xcal::DurationPropType> =
            Default::default();
        let mut strm_intervals: crate::xsd_util::SetOnce<crate::oadr20a::strm::Intervals> =
            Default::default();

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
                    "dtstart" => xcal_dtstart.set(crate::oadr20a::xcal::Dtstart::read(
                        reader,
                        &attributes,
                        "dtstart",
                    )?)?,
                    "duration" => {
                        xcal_duration.set(crate::oadr20a::xcal::DurationPropType::read(
                            reader,
                            &attributes,
                            "duration",
                        )?)?
                    }
                    "intervals" => strm_intervals.set(crate::oadr20a::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
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
        Ok(StreamBaseType {
            xcal_dtstart: xcal_dtstart.get(),
            xcal_duration: xcal_duration.get(),
            strm_intervals: strm_intervals.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "streamBase")?;
        StreamBaseType::read(reader, &attr, "streamBase")
    }
}

impl crate::xsd_util::ReadXml for StreamBaseType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match StreamBaseType::read_top_level(&mut reader) {
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

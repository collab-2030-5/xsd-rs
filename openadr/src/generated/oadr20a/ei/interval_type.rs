use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IntervalType {
    pub xcal_duration: crate::oadr20a::xcal::DurationPropType,
    pub xcal_uid: crate::oadr20a::xcal::Uid,
    pub strm_stream_payload_base: crate::oadr20a::strm::StreamPayloadBaseType,
}

impl IntervalType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.xcal_duration
            .write_with_name(writer, "xcal:duration", false, false)?;
        self.xcal_uid
            .write_with_name(writer, "xcal:uid", false, false)?;
        self.strm_stream_payload_base.write(writer)?;
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
            start.attr("xsi:type", "IntervalType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for IntervalType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:interval", true, false)?;
        Ok(())
    }
}

impl IntervalType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut xcal_duration: crate::xsd_util::SetOnce<crate::oadr20a::xcal::DurationPropType> =
            Default::default();
        let mut xcal_uid: crate::xsd_util::SetOnce<crate::oadr20a::xcal::Uid> = Default::default();
        let mut strm_stream_payload_base: crate::xsd_util::SetOnce<
            crate::oadr20a::strm::StreamPayloadBaseType,
        > = Default::default();

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
                    "duration" => {
                        xcal_duration.set(crate::oadr20a::xcal::DurationPropType::read(
                            reader,
                            &attributes,
                            "duration",
                        )?)?
                    }
                    "uid" => xcal_uid.set(crate::oadr20a::xcal::Uid::read(
                        reader,
                        &attributes,
                        "uid",
                    )?)?,
                    "signalPayload" => strm_stream_payload_base.set(
                        crate::oadr20a::strm::StreamPayloadBaseType::SignalPayload(
                            crate::oadr20a::ei::SignalPayloadType::read(
                                reader,
                                &attributes,
                                "signalPayload",
                            )?,
                        ),
                    )?,
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
        Ok(IntervalType {
            xcal_duration: xcal_duration.require()?,
            xcal_uid: xcal_uid.require()?,
            strm_stream_payload_base: strm_stream_payload_base.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "interval")?;
        IntervalType::read(reader, &attr, "interval")
    }
}

impl crate::xsd_util::ReadXml for IntervalType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match IntervalType::read_top_level(&mut reader) {
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

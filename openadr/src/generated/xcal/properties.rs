use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
    pub xcal_dtstart: crate::xcal::Dtstart,
    pub xcal_duration: crate::xcal::DurationPropType,
    /// Set randomization period for start of event
    pub tolerance: Option<crate::xcal::Tolerance>,
    pub ei_x_ei_notification: Option<crate::xcal::DurationPropType>,
    pub ei_x_ei_ramp_up: Option<crate::xcal::DurationPropType>,
    pub ei_x_ei_recovery: Option<crate::xcal::DurationPropType>,
}

impl Properties {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.xcal_dtstart
            .write_with_name(writer, "xcal:dtstart", false, false)?;
        self.xcal_duration
            .write_with_name(writer, "xcal:duration", false, false)?;
        if let Some(elem) = &self.tolerance {
            elem.write_with_name(writer, "tolerance", false, false)?;
        }
        if let Some(elem) = &self.ei_x_ei_notification {
            elem.write_with_name(writer, "ei:x-eiNotification", false, false)?;
        }
        if let Some(elem) = &self.ei_x_ei_ramp_up {
            elem.write_with_name(writer, "ei:x-eiRampUp", false, false)?;
        }
        if let Some(elem) = &self.ei_x_ei_recovery {
            elem.write_with_name(writer, "ei:x-eiRecovery", false, false)?;
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
            start.attr("xsi:type", "properties")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for Properties {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "xcal:properties", true, false)?;
        Ok(())
    }
}

impl Properties {
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
        let mut tolerance: xsd_util::SetOnce<crate::xcal::Tolerance> = Default::default();
        let mut ei_x_ei_notification: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();
        let mut ei_x_ei_ramp_up: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();
        let mut ei_x_ei_recovery: xsd_util::SetOnce<crate::xcal::DurationPropType> =
            Default::default();

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
                    "dtstart" => xcal_dtstart.set(crate::xcal::Dtstart::read(
                        reader,
                        &attributes,
                        "dtstart",
                    )?)?,
                    "duration" => xcal_duration.set(crate::xcal::DurationPropType::read(
                        reader,
                        &attributes,
                        "duration",
                    )?)?,
                    "tolerance" => tolerance.set(crate::xcal::Tolerance::read(
                        reader,
                        &attributes,
                        "tolerance",
                    )?)?,
                    "x-eiNotification" => {
                        ei_x_ei_notification.set(crate::xcal::DurationPropType::read(
                            reader,
                            &attributes,
                            "x-eiNotification",
                        )?)?
                    }
                    "x-eiRampUp" => ei_x_ei_ramp_up.set(crate::xcal::DurationPropType::read(
                        reader,
                        &attributes,
                        "x-eiRampUp",
                    )?)?,
                    "x-eiRecovery" => ei_x_ei_recovery.set(crate::xcal::DurationPropType::read(
                        reader,
                        &attributes,
                        "x-eiRecovery",
                    )?)?,
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
        Ok(Properties {
            xcal_dtstart: xcal_dtstart.require()?,
            xcal_duration: xcal_duration.require()?,
            tolerance: tolerance.get(),
            ei_x_ei_notification: ei_x_ei_notification.get(),
            ei_x_ei_ramp_up: ei_x_ei_ramp_up.get(),
            ei_x_ei_recovery: ei_x_ei_recovery.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "properties")?;
        Properties::read(reader, &attr, "properties")
    }
}

impl xsd_api::ReadXml for Properties {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match Properties::read_top_level(&mut reader) {
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

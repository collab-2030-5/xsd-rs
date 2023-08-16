use xml::common::Position;
use xml::writer::*;

/// Payload for use in Report Specifiers.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecifierPayloadType {
    pub ei_r_id: String,
    /// What is measured or tracked in this report (Units).
    pub emix_item_base: Option<crate::emix::ItemBaseType>,
    pub ei_reading_type: String,
}

impl SpecifierPayloadType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:rID", self.ei_r_id.as_str())?;
        if let Some(elem) = &self.emix_item_base {
            elem.write(writer)?;
        }
        xsd_util::write_simple_element(writer, "ei:readingType", self.ei_reading_type.as_str())?;
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
            start.attr("xsi:type", "SpecifierPayloadType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for SpecifierPayloadType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:SpecifierPayloadType", true, false)?;
        Ok(())
    }
}

impl SpecifierPayloadType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_r_id: xsd_util::SetOnce<String> = Default::default();
        let mut emix_item_base: xsd_util::SetOnce<crate::emix::ItemBaseType> = Default::default();
        let mut ei_reading_type: xsd_util::SetOnce<String> = Default::default();

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
                    "rID" => ei_r_id.set(xsd_util::read_string(reader, "rID")?)?,
                    "VoltageType" => emix_item_base.set(crate::emix::ItemBaseType::VoltageType(
                        crate::power::VoltageType::read(reader, &attributes, "VoltageType")?,
                    ))?,
                    "ThermType" => emix_item_base.set(crate::emix::ItemBaseType::ThermType(
                        crate::oadr::ThermType::read(reader, &attributes, "ThermType")?,
                    ))?,
                    "EnergyItemType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::EnergyItemType(
                            crate::power::EnergyItemType::read(
                                reader,
                                &attributes,
                                "EnergyItemType",
                            )?,
                        ))?
                    }
                    "temperatureType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::TemperatureType(
                            crate::oadr::TemperatureType::read(
                                reader,
                                &attributes,
                                "temperatureType",
                            )?,
                        ))?
                    }
                    "PowerItemType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::PowerItemType(
                            crate::power::PowerItemType::read(
                                reader,
                                &attributes,
                                "PowerItemType",
                            )?,
                        ))?
                    }
                    "pulseCountType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::PulseCountType(
                            crate::oadr::PulseCountType::read(
                                reader,
                                &attributes,
                                "pulseCountType",
                            )?,
                        ))?
                    }
                    "CurrentType" => emix_item_base.set(crate::emix::ItemBaseType::CurrentType(
                        crate::oadr::CurrentType::read(reader, &attributes, "CurrentType")?,
                    ))?,
                    "currencyType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::CurrencyType(
                            crate::oadr::CurrencyType::read(reader, &attributes, "currencyType")?,
                        ))?
                    }
                    "FrequencyType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::FrequencyType(
                            crate::oadr::FrequencyType::read(reader, &attributes, "FrequencyType")?,
                        ))?
                    }
                    "BaseUnitType" => {
                        emix_item_base.set(crate::emix::ItemBaseType::BaseUnitType(
                            crate::oadr::BaseUnitType::read(reader, &attributes, "BaseUnitType")?,
                        ))?
                    }
                    "readingType" => {
                        ei_reading_type.set(xsd_util::read_string(reader, "readingType")?)?
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
        Ok(SpecifierPayloadType {
            ei_r_id: ei_r_id.require()?,
            emix_item_base: emix_item_base.get(),
            ei_reading_type: ei_reading_type.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "SpecifierPayloadType")?;
        SpecifierPayloadType::read(reader, &attr, "SpecifierPayloadType")
    }
}

impl xsd_api::ReadXml for SpecifierPayloadType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match SpecifierPayloadType::read_top_level(&mut reader) {
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

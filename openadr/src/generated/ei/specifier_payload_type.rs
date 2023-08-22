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
                    "Therm" => emix_item_base.set(crate::emix::ItemBaseType::Therm(
                        crate::oadr::ThermType::read(reader, &attributes, "Therm")?,
                    ))?,
                    "currency" => emix_item_base.set(crate::emix::ItemBaseType::Currency(
                        crate::oadr::CurrencyType::read(reader, &attributes, "currency")?,
                    ))?,
                    "current" => emix_item_base.set(crate::emix::ItemBaseType::Current(
                        crate::oadr::CurrentType::read(reader, &attributes, "current")?,
                    ))?,
                    "customUnit" => emix_item_base.set(crate::emix::ItemBaseType::CustomUnit(
                        crate::oadr::BaseUnitType::read(reader, &attributes, "customUnit")?,
                    ))?,
                    "frequency" => emix_item_base.set(crate::emix::ItemBaseType::Frequency(
                        crate::oadr::FrequencyType::read(reader, &attributes, "frequency")?,
                    ))?,
                    "pulseCount" => emix_item_base.set(crate::emix::ItemBaseType::PulseCount(
                        crate::oadr::PulseCountType::read(reader, &attributes, "pulseCount")?,
                    ))?,
                    "temperature" => emix_item_base.set(crate::emix::ItemBaseType::Temperature(
                        crate::oadr::TemperatureType::read(reader, &attributes, "temperature")?,
                    ))?,
                    "voltage" => emix_item_base.set(crate::emix::ItemBaseType::Voltage(
                        crate::power::VoltageType::read(reader, &attributes, "voltage")?,
                    ))?,
                    "energyApparent" => {
                        emix_item_base.set(crate::emix::ItemBaseType::EnergyApparent(
                            crate::power::EnergyApparentType::read(
                                reader,
                                &attributes,
                                "energyApparent",
                            )?,
                        ))?
                    }
                    "energyReactive" => {
                        emix_item_base.set(crate::emix::ItemBaseType::EnergyReactive(
                            crate::power::EnergyReactiveType::read(
                                reader,
                                &attributes,
                                "energyReactive",
                            )?,
                        ))?
                    }
                    "energyReal" => emix_item_base.set(crate::emix::ItemBaseType::EnergyReal(
                        crate::power::EnergyRealType::read(reader, &attributes, "energyReal")?,
                    ))?,
                    "powerApparent" => {
                        emix_item_base.set(crate::emix::ItemBaseType::PowerApparent(
                            crate::power::PowerApparentType::read(
                                reader,
                                &attributes,
                                "powerApparent",
                            )?,
                        ))?
                    }
                    "powerReactive" => {
                        emix_item_base.set(crate::emix::ItemBaseType::PowerReactive(
                            crate::power::PowerReactiveType::read(
                                reader,
                                &attributes,
                                "powerReactive",
                            )?,
                        ))?
                    }
                    "powerReal" => emix_item_base.set(crate::emix::ItemBaseType::PowerReal(
                        crate::power::PowerRealType::read(reader, &attributes, "powerReal")?,
                    ))?,
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

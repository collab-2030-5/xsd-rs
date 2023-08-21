use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiEventSignalType {
    pub strm_intervals: crate::strm::Intervals,
    /// Optionally identifies the device class associated with the signal. Only the endDeviceAsset subelement is used
    pub ei_ei_target: Option<crate::ei::EiTargetType>,
    /// Descriptive name for signal.
    pub ei_signal_name: String,
    pub ei_signal_type: crate::ei::SignalTypeEnumeratedType,
    /// unique Identifier for a specific event signal
    pub signal_id: String,
    /// This is the unit of the signal.
    pub emix_item_base: Option<crate::emix::ItemBaseType>,
    pub ei_current_value: Option<crate::ei::CurrentValueType>,
}

impl EiEventSignalType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.strm_intervals
            .write_with_name(writer, "strm:intervals", false, false)?;
        if let Some(elem) = &self.ei_ei_target {
            elem.write_with_name(writer, "ei:eiTarget", false, false)?;
        }
        xsd_util::write_simple_element(writer, "ei:signalName", self.ei_signal_name.as_str())?;
        xsd_util::write_string_enumeration(writer, "ei:signalType", self.ei_signal_type)?;
        xsd_util::write_simple_element(writer, "ei:signalID", self.signal_id.as_str())?;
        if let Some(elem) = &self.emix_item_base {
            elem.write(writer)?;
        }
        if let Some(elem) = &self.ei_current_value {
            elem.write_with_name(writer, "ei:currentValue", false, false)?;
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
            start.attr("xsi:type", "eiEventSignalType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EiEventSignalType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eiEventSignalType", true, false)?;
        Ok(())
    }
}

impl EiEventSignalType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut strm_intervals: xsd_util::SetOnce<crate::strm::Intervals> = Default::default();
        let mut ei_ei_target: xsd_util::SetOnce<crate::ei::EiTargetType> = Default::default();
        let mut ei_signal_name: xsd_util::SetOnce<String> = Default::default();
        let mut ei_signal_type: xsd_util::SetOnce<crate::ei::SignalTypeEnumeratedType> =
            Default::default();
        let mut signal_id: xsd_util::SetOnce<String> = Default::default();
        let mut emix_item_base: xsd_util::SetOnce<crate::emix::ItemBaseType> = Default::default();
        let mut ei_current_value: xsd_util::SetOnce<crate::ei::CurrentValueType> =
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
                    "intervals" => strm_intervals.set(crate::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
                    "eiTarget" => ei_ei_target.set(crate::ei::EiTargetType::read(
                        reader,
                        &attributes,
                        "eiTarget",
                    )?)?,
                    "signalName" => {
                        ei_signal_name.set(xsd_util::read_string(reader, "signalName")?)?
                    }
                    "signalType" => {
                        ei_signal_type.set(xsd_util::read_string_enum(reader, "signalType")?)?
                    }
                    "signalID" => signal_id.set(xsd_util::read_string(reader, "signalID")?)?,
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
                    "currentValue" => ei_current_value.set(crate::ei::CurrentValueType::read(
                        reader,
                        &attributes,
                        "currentValue",
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
        Ok(EiEventSignalType {
            strm_intervals: strm_intervals.require()?,
            ei_ei_target: ei_ei_target.get(),
            ei_signal_name: ei_signal_name.require()?,
            ei_signal_type: ei_signal_type.require()?,
            signal_id: signal_id.require()?,
            emix_item_base: emix_item_base.get(),
            ei_current_value: ei_current_value.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "eiEventSignalType")?;
        EiEventSignalType::read(reader, &attr, "eiEventSignalType")
    }
}

impl xsd_api::ReadXml for EiEventSignalType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiEventSignalType::read_top_level(&mut reader) {
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

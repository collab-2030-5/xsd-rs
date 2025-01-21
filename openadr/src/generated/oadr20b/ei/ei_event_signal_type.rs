use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiEventSignalType {
    pub strm_intervals: crate::oadr20b::strm::Intervals,
    /// Optionally identifies the device class associated with the signal. Only the endDeviceAsset subelement is used
    pub ei_ei_target: Option<crate::oadr20b::ei::EiTargetType>,
    /// Descriptive name for signal.
    pub ei_signal_name: crate::oadr20b::ei::SignalNameType,
    pub ei_signal_type: crate::oadr20b::ei::SignalTypeEnumeratedType,
    /// unique Identifier for a specific event signal
    pub signal_id: String,
    /// This is the unit of the signal.
    pub emix_item_base: Option<crate::oadr20b::emix::ItemBaseType>,
    pub ei_current_value: Option<crate::oadr20b::ei::CurrentValueType>,
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
        self.ei_signal_name.write(writer)?;
        crate::xsd_util::write_string_enumeration(writer, "ei:signalType", &self.ei_signal_type)?;
        crate::xsd_util::write_simple_element(writer, "ei:signalID", self.signal_id.as_str())?;
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

impl crate::xsd_util::WriteXml for EiEventSignalType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eiEventSignal", true, false)?;
        Ok(())
    }
}

impl EiEventSignalType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut strm_intervals: crate::xsd_util::SetOnce<crate::oadr20b::strm::Intervals> =
            Default::default();
        let mut ei_ei_target: crate::xsd_util::SetOnce<crate::oadr20b::ei::EiTargetType> =
            Default::default();
        let mut ei_signal_name: crate::xsd_util::SetOnce<crate::oadr20b::ei::SignalNameType> =
            Default::default();
        let mut ei_signal_type: crate::xsd_util::SetOnce<
            crate::oadr20b::ei::SignalTypeEnumeratedType,
        > = Default::default();
        let mut signal_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut emix_item_base: crate::xsd_util::SetOnce<crate::oadr20b::emix::ItemBaseType> =
            Default::default();
        let mut ei_current_value: crate::xsd_util::SetOnce<crate::oadr20b::ei::CurrentValueType> =
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
                    "intervals" => strm_intervals.set(crate::oadr20b::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
                    "eiTarget" => ei_ei_target.set(crate::oadr20b::ei::EiTargetType::read(
                        reader,
                        &attributes,
                        "eiTarget",
                    )?)?,
                    "signalName" => ei_signal_name.set(
                        crate::signal_name_type::read_choice_enum(reader, "signalName")?,
                    )?,
                    "signalType" => ei_signal_type
                        .set(crate::xsd_util::read_string_enum(reader, "signalType")?)?,
                    "signalID" => {
                        signal_id.set(crate::xsd_util::read_string(reader, "signalID")?)?
                    }
                    "Therm" => emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Therm(
                        crate::oadr20b::oadr::ThermType::read(reader, &attributes, "Therm")?,
                    ))?,
                    "currency" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Currency(
                            crate::oadr20b::oadr::CurrencyType::read(
                                reader,
                                &attributes,
                                "currency",
                            )?,
                        ))?
                    }
                    "currencyPerKW" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::CurrencyPerKw(
                            crate::oadr20b::oadr::CurrencyType::read(
                                reader,
                                &attributes,
                                "currencyPerKW",
                            )?,
                        ))?
                    }
                    "currencyPerKWh" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::CurrencyPerKWh(
                            crate::oadr20b::oadr::CurrencyType::read(
                                reader,
                                &attributes,
                                "currencyPerKWh",
                            )?,
                        ))?
                    }
                    "currencyPerThm" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::CurrencyPerThm(
                            crate::oadr20b::oadr::CurrencyType::read(
                                reader,
                                &attributes,
                                "currencyPerThm",
                            )?,
                        ))?
                    }
                    "current" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Current(
                            crate::oadr20b::oadr::CurrentType::read(
                                reader,
                                &attributes,
                                "current",
                            )?,
                        ))?
                    }
                    "customUnit" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::CustomUnit(
                            crate::oadr20b::oadr::BaseUnitType::read(
                                reader,
                                &attributes,
                                "customUnit",
                            )?,
                        ))?
                    }
                    "frequency" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Frequency(
                            crate::oadr20b::oadr::FrequencyType::read(
                                reader,
                                &attributes,
                                "frequency",
                            )?,
                        ))?
                    }
                    "pulseCount" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::PulseCount(
                            crate::oadr20b::oadr::PulseCountType::read(
                                reader,
                                &attributes,
                                "pulseCount",
                            )?,
                        ))?
                    }
                    "temperature" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Temperature(
                            crate::oadr20b::oadr::TemperatureType::read(
                                reader,
                                &attributes,
                                "temperature",
                            )?,
                        ))?
                    }
                    "voltage" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::Voltage(
                            crate::oadr20b::power::VoltageType::read(
                                reader,
                                &attributes,
                                "voltage",
                            )?,
                        ))?
                    }
                    "energyApparent" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::EnergyApparent(
                            crate::oadr20b::power::EnergyApparentType::read(
                                reader,
                                &attributes,
                                "energyApparent",
                            )?,
                        ))?
                    }
                    "energyReactive" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::EnergyReactive(
                            crate::oadr20b::power::EnergyReactiveType::read(
                                reader,
                                &attributes,
                                "energyReactive",
                            )?,
                        ))?
                    }
                    "energyReal" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::EnergyReal(
                            crate::oadr20b::power::EnergyRealType::read(
                                reader,
                                &attributes,
                                "energyReal",
                            )?,
                        ))?
                    }
                    "powerApparent" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::PowerApparent(
                            crate::oadr20b::power::PowerApparentType::read(
                                reader,
                                &attributes,
                                "powerApparent",
                            )?,
                        ))?
                    }
                    "powerReactive" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::PowerReactive(
                            crate::oadr20b::power::PowerReactiveType::read(
                                reader,
                                &attributes,
                                "powerReactive",
                            )?,
                        ))?
                    }
                    "powerReal" => {
                        emix_item_base.set(crate::oadr20b::emix::ItemBaseType::PowerReal(
                            crate::oadr20b::power::PowerRealType::read(
                                reader,
                                &attributes,
                                "powerReal",
                            )?,
                        ))?
                    }
                    "currentValue" => {
                        ei_current_value.set(crate::oadr20b::ei::CurrentValueType::read(
                            reader,
                            &attributes,
                            "currentValue",
                        )?)?
                    }
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
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "eiEventSignal")?;
        EiEventSignalType::read(reader, &attr, "eiEventSignal")
    }
}

impl crate::xsd_util::ReadXml for EiEventSignalType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiEventSignalType::read_top_level(&mut reader) {
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

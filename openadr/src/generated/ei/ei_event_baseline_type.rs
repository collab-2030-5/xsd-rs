use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiEventBaselineType {
    pub xcal_dtstart: crate::xcal::Dtstart,
    pub xcal_duration: crate::xcal::DurationPropType,
    pub strm_intervals: crate::strm::Intervals,
    /// Unique ID for a specific baseline
    pub baseline_id: String,
    pub ei_resource_id: Vec<String>,
    /// Descriptive name for baseline
    pub baseline_name: String,
    /// This is the unit of the signal.
    pub emix_item_base: Option<crate::emix::ItemBaseType>,
}

impl EiEventBaselineType {
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
        self.strm_intervals
            .write_with_name(writer, "strm:intervals", false, false)?;
        xsd_util::write_simple_element(writer, "ei:baselineID", self.baseline_id.as_str())?;
        for item in &self.ei_resource_id {
            xsd_util::write_simple_element(writer, "ei:resourceID", item.as_str())?;
        }
        xsd_util::write_simple_element(writer, "ei:baselineName", self.baseline_name.as_str())?;
        if let Some(elem) = &self.emix_item_base {
            elem.write(writer)?;
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
            start.attr("xsi:type", "eiEventBaselineType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EiEventBaselineType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:eiEventBaselineType", true, false)?;
        Ok(())
    }
}

impl EiEventBaselineType {
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
        let mut baseline_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_resource_id: Vec<String> = Default::default();
        let mut baseline_name: xsd_util::SetOnce<String> = Default::default();
        let mut emix_item_base: xsd_util::SetOnce<crate::emix::ItemBaseType> = Default::default();

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
                    "intervals" => strm_intervals.set(crate::strm::Intervals::read(
                        reader,
                        &attributes,
                        "intervals",
                    )?)?,
                    "baselineID" => {
                        baseline_id.set(xsd_util::read_string(reader, "baselineID")?)?
                    }
                    "resourceID" => {
                        ei_resource_id.push(xsd_util::read_string(reader, "resourceID")?)
                    }
                    "baselineName" => {
                        baseline_name.set(xsd_util::read_string(reader, "baselineName")?)?
                    }
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
        Ok(EiEventBaselineType {
            xcal_dtstart: xcal_dtstart.require()?,
            xcal_duration: xcal_duration.require()?,
            strm_intervals: strm_intervals.require()?,
            baseline_id: baseline_id.require()?,
            ei_resource_id,
            baseline_name: baseline_name.require()?,
            emix_item_base: emix_item_base.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "eiEventBaselineType")?;
        EiEventBaselineType::read(reader, &attr, "eiEventBaselineType")
    }
}

impl xsd_api::ReadXml for EiEventBaselineType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiEventBaselineType::read_top_level(&mut reader) {
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

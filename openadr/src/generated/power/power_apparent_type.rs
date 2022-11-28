use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PowerApparentType {
    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: crate::scale::SiScaleCodeType,
    pub power_power_attributes: crate::power::PowerAttributesType,
}

impl PowerApparentType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "itemDescription", self.item_description.as_str())?;
        xsd_util::write_simple_element(writer, "itemUnits", self.item_units.as_str())?;
        xsd_util::write_string_enumeration(writer, "scale:siScaleCode", self.scale_si_scale_code)?;
        self.power_power_attributes.write_with_name(
            writer,
            "power:powerAttributes",
            false,
            false,
        )?;
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
            start.attr("xsi:type", "power:PowerApparentType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for PowerApparentType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "power:PowerApparentType", true, false)?;
        Ok(())
    }
}

impl PowerApparentType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut item_description: xsd_util::SetOnce<String> = Default::default();
        let mut item_units: xsd_util::SetOnce<String> = Default::default();
        let mut scale_si_scale_code: xsd_util::SetOnce<crate::scale::SiScaleCodeType> =
            Default::default();
        let mut power_power_attributes: xsd_util::SetOnce<crate::power::PowerAttributesType> =
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
                    "itemDescription" => {
                        item_description.set(xsd_util::read_string(reader, "itemDescription")?)?
                    }
                    "itemUnits" => item_units.set(xsd_util::read_string(reader, "itemUnits")?)?,
                    "scale:siScaleCode" => scale_si_scale_code
                        .set(xsd_util::read_string_enum(reader, "scale:siScaleCode")?)?,
                    "power:powerAttributes" => {
                        power_power_attributes.set(crate::power::PowerAttributesType::read(
                            reader,
                            &attributes,
                            "power:powerAttributes",
                        )?)?
                    }
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
        Ok(PowerApparentType {
            item_description: item_description.require()?,
            item_units: item_units.require()?,
            scale_si_scale_code: scale_si_scale_code.require()?,
            power_power_attributes: power_power_attributes.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "PowerApparentType")?;
        PowerApparentType::read(reader, &attr, "power:PowerApparentType")
    }
}

impl xsd_api::ReadXml for PowerApparentType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match PowerApparentType::read_top_level(&mut reader) {
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

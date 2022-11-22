use crate::*;
use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PowerRealType {
    // --- these fields come from emix:ItemBaseType ---

    // --- these fields come from power:PowerItemType ---
    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: crate::types::scale::SiScaleCodeType,
    pub power_power_attributes: crate::types::power::PowerAttributesType,

    // --- these fields come from power:PowerRealType ---
    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: crate::types::scale::SiScaleCodeType,
    pub power_power_attributes: crate::types::power::PowerAttributesType,
}

impl PowerRealType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        write_simple_tag(writer, "itemDescription", self.item_description.as_str())?;
        write_simple_tag(writer, "itemUnits", self.item_units.as_str())?;
        write_simple_tag(
            writer,
            "scale:siScaleCode",
            self.scale_si_scale_code.as_str(),
        )?;
        self.power_power_attributes.write_with_name(
            writer,
            "power:powerAttributes",
            false,
            false,
        )?;
        write_simple_tag(writer, "itemDescription", self.item_description.as_str())?;
        write_simple_tag(writer, "itemUnits", self.item_units.as_str())?;
        write_simple_tag(
            writer,
            "scale:siScaleCode",
            self.scale_si_scale_code.as_str(),
        )?;
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
            start.attr("xsi:type", "power:PowerRealType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXml for PowerRealType {
    fn write<W>(&self, config: WriteConfig, writer: &mut W) -> core::result::Result<(), WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.to_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "power:PowerRealType", true, false)?;
        Ok(())
    }
}

impl PowerRealType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut item_description: SetOnce<String> = Default::default();
        let mut item_units: SetOnce<String> = Default::default();
        let mut scale_si_scale_code: SetOnce<crate::types::scale::SiScaleCodeType> =
            Default::default();
        let mut power_power_attributes: SetOnce<crate::types::power::PowerAttributesType> =
            Default::default();
        let mut item_description: SetOnce<String> = Default::default();
        let mut item_units: SetOnce<String> = Default::default();
        let mut scale_si_scale_code: SetOnce<crate::types::scale::SiScaleCodeType> =
            Default::default();
        let mut power_power_attributes: SetOnce<crate::types::power::PowerAttributesType> =
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
                        return Err(ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "itemDescription" => {
                        item_description.set(read_string(reader, "itemDescription")?)?
                    }
                    "itemUnits" => item_units.set(read_string(reader, "itemUnits")?)?,
                    "scale:siScaleCode" => {
                        scale_si_scale_code.set(crate::types::scale::SiScaleCodeType::from_str(
                            read_string(reader, "scale:siScaleCode"),
                        )?)?
                    }
                    "power:powerAttributes" => power_power_attributes.set(
                        crate::types::power::PowerAttributesType::read(
                            reader,
                            &attributes,
                            "power:powerAttributes",
                        )?,
                    )?,
                    "itemDescription" => {
                        item_description.set(read_string(reader, "itemDescription")?)?
                    }
                    "itemUnits" => item_units.set(read_string(reader, "itemUnits")?)?,
                    "scale:siScaleCode" => {
                        scale_si_scale_code.set(crate::types::scale::SiScaleCodeType::from_str(
                            read_string(reader, "scale:siScaleCode"),
                        )?)?
                    }
                    "power:powerAttributes" => power_power_attributes.set(
                        crate::types::power::PowerAttributesType::read(
                            reader,
                            &attributes,
                            "power:powerAttributes",
                        )?,
                    )?,
                    _ => return Err(ReadError::UnexpectedEvent),
                },
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => {
                    return Err(ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::EndDocument => return Err(ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::Characters(_) => return Err(ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::ProcessingInstruction { .. } => {
                    return Err(ReadError::UnexpectedEvent)
                }
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(PowerRealType {
            item_description: item_description.require()?,
            item_units: item_units.require()?,
            scale_si_scale_code: scale_si_scale_code.require()?,
            power_power_attributes: power_power_attributes.require()?,
            item_description: item_description.require()?,
            item_units: item_units.require()?,
            scale_si_scale_code: scale_si_scale_code.require()?,
            power_power_attributes: power_power_attributes.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, ReadError>
    where
        R: std::io::Read,
    {
        let attr = read_start_tag(reader, "PowerRealType")?;
        PowerRealType::read(reader, &attr, "power:PowerRealType")
    }
}

impl ReadXml for PowerRealType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match PowerRealType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(ErrorWithLocation {
                    err,
                    line: pos.row,
                    col: pos.column,
                })
            }
        }
    }
}

use crate::*;
use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyReactiveType {
    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: crate::types::scale::SiScaleCodeType,
}

impl EnergyReactiveType {
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
            self.scale_si_scale_code.to_str(),
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
            start.attr("xsi:type", "power:EnergyReactiveType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXml for EnergyReactiveType {
    fn write<W>(&self, config: WriteConfig, writer: &mut W) -> core::result::Result<(), WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.to_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "power:EnergyReactiveType", true, false)?;
        Ok(())
    }
}

impl EnergyReactiveType {
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
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "itemDescription" => {
                            item_description.set(read_string(reader, "itemDescription")?)?
                        }
                        "itemUnits" => item_units.set(read_string(reader, "itemUnits")?)?,
                        "scale:siScaleCode" => scale_si_scale_code.set(
                            crate::types::scale::SiScaleCodeType::from_str(&read_string(
                                reader,
                                "scale:siScaleCode",
                            )?)?,
                        )?,
                        _ => return Err(ReadError::UnexpectedEvent),
                    }
                }
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
        Ok(EnergyReactiveType {
            item_description: item_description.require()?,
            item_units: item_units.require()?,
            scale_si_scale_code: scale_si_scale_code.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, ReadError>
    where
        R: std::io::Read,
    {
        let attr = read_start_tag(reader, "EnergyReactiveType")?;
        EnergyReactiveType::read(reader, &attr, "power:EnergyReactiveType")
    }
}

impl ReadXml for EnergyReactiveType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EnergyReactiveType::read_top_level(&mut reader) {
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

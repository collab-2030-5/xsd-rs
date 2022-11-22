use crate::*;
use xml::common::Position;
use xml::writer::*;

/// A pricing node is directly associated with a connectivity node.  It is a pricing location for which market participants submit their bids, offers, buy/sell CRRs, and settle.
#[derive(Debug, Clone, PartialEq)]
pub struct PnodeType {
    pub power_node: String,
}

impl PnodeType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        write_simple_tag(writer, "power:node", self.power_node.as_str())?;
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
            start.attr("xsi:type", "power:PnodeType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXml for PnodeType {
    fn write<W>(&self, config: WriteConfig, writer: &mut W) -> core::result::Result<(), WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.to_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "power:PnodeType", true, false)?;
        Ok(())
    }
}

impl PnodeType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut power_node: SetOnce<String> = Default::default();

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
                        "power:node" => power_node.set(read_string(reader, "power:node")?)?,
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
        Ok(PnodeType {
            power_node: power_node.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, ReadError>
    where
        R: std::io::Read,
    {
        let attr = read_start_tag(reader, "PnodeType")?;
        PnodeType::read(reader, &attr, "power:PnodeType")
    }
}

impl ReadXml for PnodeType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match PnodeType::read_top_level(&mut reader) {
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
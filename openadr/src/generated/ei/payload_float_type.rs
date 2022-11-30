use xml::writer::*;
use xml::common::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct PayloadFloatType {
    pub value: f32,

}

impl PayloadFloatType {
    fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
        xsd_util::write_element_using_to_string(writer, "value", self.value)?;
        Ok(())
    }

    pub(crate) fn write_with_name<W>(&self, writer: &mut EventWriter<W>, name: &str, top_level: bool, write_type: bool) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
        let start = if top_level { super::add_schema_attr(events::XmlEvent::start_element(name)) } else { events::XmlEvent::start_element(name) };
        let start = if write_type {
            start.attr("xsi:type", "ei:PayloadFloatType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for PayloadFloatType {
    fn write<W>(&self, config: xsd_api::WriteConfig, writer: &mut W) -> core::result::Result<(), xsd_api::WriteError> where W: std::io::Write {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:PayloadFloatType", true, false)?;
        Ok(())
    }
}

impl PayloadFloatType {
    pub(crate) fn read<R>(reader: &mut xml::reader::EventReader<R>, attrs: &Vec<xml::attribute::OwnedAttribute>, parent_tag: &str) -> core::result::Result<Self, xsd_api::ReadError> where R: std::io::Read {
        // one variable for each attribute and element
        let mut value : xsd_util::SetOnce<f32> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                _ => {}, // ignore unknown attributes
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
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "value" => {
                            value.set(xsd_util::read_type_from_string(reader, "value")?)?
                        }
                        _ => return Err(xsd_api::ReadError::UnexpectedEvent)
                    }
                }
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => return Err(xsd_api::ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::EndDocument => return Err(xsd_api::ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::Characters(_) => return Err(xsd_api::ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::ProcessingInstruction { .. } => return Err(xsd_api::ReadError::UnexpectedEvent),
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(PayloadFloatType {
            value : value.require()?,
        })
    }

    fn read_top_level<R>(reader: &mut xml::reader::EventReader<R>) -> core::result::Result<Self, xsd_api::ReadError> where R: std::io::Read {
        let attr = xsd_util::read_start_tag(reader, "PayloadFloatType")?;
        PayloadFloatType::read(reader, &attr, "ei:PayloadFloatType")
    }
}

impl xsd_api::ReadXml for PayloadFloatType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation> where R: std::io::Read {
        let mut reader = xml::reader::EventReader::new(r);

        match PayloadFloatType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(xsd_api::ErrorWithLocation { err, line: pos.row, col: pos.column })
            }
        }
    }
}
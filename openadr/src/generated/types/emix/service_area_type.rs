use crate::*;
use xml::writer::*;
use xml::common::Position;

/// The Service Area is the geographic region that is affected by the EMIX market condition
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceAreaType {

    // --- these fields come from emix:ServiceAreaType ---

    pub gml_feature_collection: crate::types::gml::FeatureCollection,

}

impl ServiceAreaType {
    fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
        self.gml_feature_collection.write_with_name(writer, "gml:FeatureCollection", false, false)?;
        Ok(())
    }

    pub(crate) fn write_with_name<W>(&self, writer: &mut EventWriter<W>, name: &str, top_level: bool, write_type: bool) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
        let start = if top_level { super::add_schema_attr(events::XmlEvent::start_element(name)) } else { events::XmlEvent::start_element(name) };
        let start = if write_type {
            start.attr("xsi:type", "emix:ServiceAreaType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXml for ServiceAreaType {
    fn write<W>(&self, config: WriteConfig, writer: &mut W) -> core::result::Result<(), WriteError> where W: std::io::Write {
        let mut writer = config.to_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "emix:ServiceAreaType", true, false)?;
        Ok(())
    }
}

impl ServiceAreaType {
    pub(crate) fn read<R>(reader: &mut xml::reader::EventReader<R>, attrs: &Vec<xml::attribute::OwnedAttribute>, parent_tag: &str) -> core::result::Result<Self, ReadError> where R: std::io::Read {
        // one variable for each attribute and element
        let mut gml_feature_collection : SetOnce<crate::types::gml::FeatureCollection> = Default::default();

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
                        return Err(ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement { name, attributes, .. } => {
                    match name.local_name.as_str() {
                        "gml:FeatureCollection" => {
                            gml_feature_collection.set(crate::types::gml::FeatureCollection::read(reader, &attributes, "gml:FeatureCollection")?)?
                        }
                        _ => return Err(ReadError::UnexpectedEvent)
                    }
                }
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => return Err(ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::EndDocument => return Err(ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::Characters(_) => return Err(ReadError::UnexpectedEvent),
                xml::reader::XmlEvent::ProcessingInstruction { .. } => return Err(ReadError::UnexpectedEvent),
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(ServiceAreaType {
            gml_feature_collection : gml_feature_collection.require()?,
        })
    }

    fn read_top_level<R>(reader: &mut xml::reader::EventReader<R>) -> core::result::Result<Self, ReadError> where R: std::io::Read {
        let attr = read_start_tag(reader, "ServiceAreaType")?;
        ServiceAreaType::read(reader, &attr, "emix:ServiceAreaType")
    }
}

impl ReadXml for ServiceAreaType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation> where R: std::io::Read {
        let mut reader = xml::reader::EventReader::new(r);

        match ServiceAreaType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(ErrorWithLocation { err, line: pos.row, col: pos.column })
            }
        }
    }
}
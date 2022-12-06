use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DsaKeyValueType {
    pub p: String,
    pub q: String,
    pub g: Option<String>,
    pub y: String,
    pub j: Option<String>,
    pub seed: String,
    pub pgen_counter: String,
}

impl DsaKeyValueType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "P", self.p.as_str())?;
        xsd_util::write_simple_element(writer, "Q", self.q.as_str())?;
        if let Some(elem) = &self.g {
            xsd_util::write_simple_element(writer, "G", elem.as_str())?;
        }
        xsd_util::write_simple_element(writer, "Y", self.y.as_str())?;
        if let Some(elem) = &self.j {
            xsd_util::write_simple_element(writer, "J", elem.as_str())?;
        }
        xsd_util::write_simple_element(writer, "Seed", self.seed.as_str())?;
        xsd_util::write_simple_element(writer, "PgenCounter", self.pgen_counter.as_str())?;
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
            start.attr("xsi:type", "ds:DSAKeyValueType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for DsaKeyValueType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ds:DSAKeyValueType", true, false)?;
        Ok(())
    }
}

impl DsaKeyValueType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut p: xsd_util::SetOnce<String> = Default::default();
        let mut q: xsd_util::SetOnce<String> = Default::default();
        let mut g: xsd_util::SetOnce<String> = Default::default();
        let mut y: xsd_util::SetOnce<String> = Default::default();
        let mut j: xsd_util::SetOnce<String> = Default::default();
        let mut seed: xsd_util::SetOnce<String> = Default::default();
        let mut pgen_counter: xsd_util::SetOnce<String> = Default::default();

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
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "P" => p.set(xsd_util::read_string(reader, "P")?)?,
                        "Q" => q.set(xsd_util::read_string(reader, "Q")?)?,
                        "G" => g.set(xsd_util::read_string(reader, "G")?)?,
                        "Y" => y.set(xsd_util::read_string(reader, "Y")?)?,
                        "J" => j.set(xsd_util::read_string(reader, "J")?)?,
                        "Seed" => seed.set(xsd_util::read_string(reader, "Seed")?)?,
                        "PgenCounter" => {
                            pgen_counter.set(xsd_util::read_string(reader, "PgenCounter")?)?
                        }
                        _ => return Err(xsd_api::ReadError::UnexpectedEvent),
                    }
                }
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
        Ok(DsaKeyValueType {
            p: p.require()?,
            q: q.require()?,
            g: g.get(),
            y: y.require()?,
            j: j.get(),
            seed: seed.require()?,
            pgen_counter: pgen_counter.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "DSAKeyValueType")?;
        DsaKeyValueType::read(reader, &attr, "ds:DSAKeyValueType")
    }
}

impl xsd_api::ReadXml for DsaKeyValueType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match DsaKeyValueType::read_top_level(&mut reader) {
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

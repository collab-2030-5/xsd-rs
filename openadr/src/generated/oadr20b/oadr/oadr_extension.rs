use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OadrExtension {
    pub oadr_extension_name: String,
    pub oadr_oadr_info: Vec<crate::oadr20b::oadr::OadrInfo>,
}

impl OadrExtension {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(
            writer,
            "oadr:oadrExtensionName",
            self.oadr_extension_name.as_str(),
        )?;
        for item in &self.oadr_oadr_info {
            item.write_with_name(writer, "oadr:oadrInfo", false, false)?;
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
            start.attr("xsi:type", "oadrExtension")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrExtension {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrExtension", true, false)?;
        Ok(())
    }
}

impl OadrExtension {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_extension_name: crate::xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_info: Vec<crate::oadr20b::oadr::OadrInfo> = Default::default();

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
                    "oadrExtensionName" => oadr_extension_name
                        .set(crate::xsd_util::read_string(reader, "oadrExtensionName")?)?,
                    "oadrInfo" => oadr_oadr_info.push(crate::oadr20b::oadr::OadrInfo::read(
                        reader,
                        &attributes,
                        "oadrInfo",
                    )?),
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
        Ok(OadrExtension {
            oadr_extension_name: oadr_extension_name.require()?,
            oadr_oadr_info,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrExtension")?;
        OadrExtension::read(reader, &attr, "oadrExtension")
    }
}

impl crate::xsd_util::ReadXml for OadrExtension {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrExtension::read_top_level(&mut reader) {
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

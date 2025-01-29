use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrRequestReregistrationType {
    pub ei_ven_id: String,
    pub ei_schema_version: Option<crate::oadr20b::ei::SchemaVersionType>,
}

impl OadrRequestReregistrationType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(writer, "ei:venID", self.ei_ven_id.as_str())?;
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
        // ---- start attributes ----
        let ei_schema_version = self
            .ei_schema_version
            .as_ref()
            .map(|x| x.as_str().to_string());
        let start = match &ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadrRequestReregistrationType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrRequestReregistrationType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrRequestReregistration", true, false)?;
        Ok(())
    }
}

impl OadrRequestReregistrationType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_ven_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_schema_version: crate::xsd_util::SetOnce<crate::oadr20b::ei::SchemaVersionType> =
            Default::default();

        #[allow(clippy::single_match)]
        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "schemaVersion" => ei_schema_version.set(
                    crate::schema_version_type::convert_string_to_enum(&attr.value)?,
                )?,
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
                        return Err(crate::xsd_util::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "venID" => ei_ven_id.set(crate::xsd_util::read_string(reader, "venID")?)?,
                        name => {
                            return Err(crate::xsd_util::ReadError::UnexpectedToken(
                                crate::xsd_util::ParentToken(parent_tag.to_owned()),
                                crate::xsd_util::ChildToken(name.to_owned()),
                            ))
                        }
                    }
                }
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
        Ok(OadrRequestReregistrationType {
            ei_ven_id: ei_ven_id.require()?,
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrRequestReregistration")?;
        OadrRequestReregistrationType::read(reader, &attr, "oadrRequestReregistration")
    }
}

impl crate::xsd_util::ReadXml for OadrRequestReregistrationType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrRequestReregistrationType::read_top_level(&mut reader) {
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

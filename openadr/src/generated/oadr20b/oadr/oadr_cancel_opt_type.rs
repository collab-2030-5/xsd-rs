use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrCancelOptType {
    pub pyld_request_id: String,
    pub ei_opt_id: String,
    pub ei_ven_id: String,
    pub ei_schema_version: Option<String>,
}

impl OadrCancelOptType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(
            writer,
            "pyld:requestID",
            self.pyld_request_id.as_str(),
        )?;
        crate::xsd_util::write_simple_element(writer, "ei:optID", self.ei_opt_id.as_str())?;
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
        let start = match &self.ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadrCancelOptType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrCancelOptType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrCancelOpt", true, false)?;
        Ok(())
    }
}

impl OadrCancelOptType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut pyld_request_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_opt_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_ven_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_schema_version: crate::xsd_util::SetOnce<String> = Default::default();

        #[allow(clippy::single_match)]
        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "schemaVersion" => ei_schema_version.set(attr.value.clone())?,
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
                        "requestID" => pyld_request_id
                            .set(crate::xsd_util::read_string(reader, "requestID")?)?,
                        "optID" => ei_opt_id.set(crate::xsd_util::read_string(reader, "optID")?)?,
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
        Ok(OadrCancelOptType {
            pyld_request_id: pyld_request_id.require()?,
            ei_opt_id: ei_opt_id.require()?,
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
        let attr = crate::xsd_util::read_start_tag(reader, "oadrCancelOpt")?;
        OadrCancelOptType::read(reader, &attr, "oadrCancelOpt")
    }
}

impl crate::xsd_util::ReadXml for OadrCancelOptType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCancelOptType::read_top_level(&mut reader) {
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

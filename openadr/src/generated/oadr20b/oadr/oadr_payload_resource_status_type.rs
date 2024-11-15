use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrPayloadResourceStatusType {
    /// If true then resource/asset is online, if false then offline.
    pub oadr_online: bool,
    /// If true then the control of the load has been manually overridden
    pub oadr_manual_override: bool,
    pub oadr_oadr_load_control_state: Option<crate::oadr20b::oadr::OadrLoadControlStateType>,
}

impl OadrPayloadResourceStatusType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrOnline",
            self.oadr_online,
        )?;
        crate::xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrManualOverride",
            self.oadr_manual_override,
        )?;
        if let Some(elem) = &self.oadr_oadr_load_control_state {
            elem.write_with_name(writer, "oadr:oadrLoadControlState", false, false)?;
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
            start.attr("xsi:type", "oadrPayloadResourceStatusType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrPayloadResourceStatusType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrPayloadResourceStatus", true, false)?;
        Ok(())
    }
}

impl OadrPayloadResourceStatusType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_online: crate::xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_manual_override: crate::xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_oadr_load_control_state: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrLoadControlStateType,
        > = Default::default();

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
                    "oadrOnline" => oadr_online.set(crate::xsd_util::read_type_from_string(
                        reader,
                        "oadrOnline",
                    )?)?,
                    "oadrManualOverride" => oadr_manual_override.set(
                        crate::xsd_util::read_type_from_string(reader, "oadrManualOverride")?,
                    )?,
                    "oadrLoadControlState" => oadr_oadr_load_control_state.set(
                        crate::oadr20b::oadr::OadrLoadControlStateType::read(
                            reader,
                            &attributes,
                            "oadrLoadControlState",
                        )?,
                    )?,
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
        Ok(OadrPayloadResourceStatusType {
            oadr_online: oadr_online.require()?,
            oadr_manual_override: oadr_manual_override.require()?,
            oadr_oadr_load_control_state: oadr_oadr_load_control_state.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrPayloadResourceStatus")?;
        OadrPayloadResourceStatusType::read(reader, &attr, "oadrPayloadResourceStatus")
    }
}

impl crate::xsd_util::ReadXml for OadrPayloadResourceStatusType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrPayloadResourceStatusType::read_top_level(&mut reader) {
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

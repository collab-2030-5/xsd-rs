use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrPayloadResourceStatusType {
    /// If true then resource/asset is online, if false then offline.
    pub oadr_online: bool,
    /// If true then the control of the load has been manually overridden
    pub oadr_manual_override: bool,
    pub oadr_oadr_load_control_state: Option<crate::oadr::OadrLoadControlStateType>,
}

impl OadrPayloadResourceStatusType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_element_using_to_string(writer, "oadrOnline", self.oadr_online)?;
        xsd_util::write_element_using_to_string(
            writer,
            "oadrManualOverride",
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

impl xsd_api::WriteXml for OadrPayloadResourceStatusType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(
            &mut writer,
            "oadr:oadrPayloadResourceStatusType",
            true,
            false,
        )?;
        Ok(())
    }
}

impl OadrPayloadResourceStatusType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_online: xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_manual_override: xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_oadr_load_control_state: xsd_util::SetOnce<
            crate::oadr::OadrLoadControlStateType,
        > = Default::default();

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
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "oadrOnline" => {
                        oadr_online.set(xsd_util::read_type_from_string(reader, "oadrOnline")?)?
                    }
                    "oadrManualOverride" => oadr_manual_override.set(
                        xsd_util::read_type_from_string(reader, "oadrManualOverride")?,
                    )?,
                    "oadrLoadControlState" => oadr_oadr_load_control_state.set(
                        crate::oadr::OadrLoadControlStateType::read(
                            reader,
                            &attributes,
                            "oadrLoadControlState",
                        )?,
                    )?,
                    _ => return Err(xsd_api::ReadError::UnexpectedEvent),
                },
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
        Ok(OadrPayloadResourceStatusType {
            oadr_online: oadr_online.require()?,
            oadr_manual_override: oadr_manual_override.require()?,
            oadr_oadr_load_control_state: oadr_oadr_load_control_state.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrPayloadResourceStatusType")?;
        OadrPayloadResourceStatusType::read(reader, &attr, "oadrPayloadResourceStatusType")
    }
}

impl xsd_api::ReadXml for OadrPayloadResourceStatusType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrPayloadResourceStatusType::read_top_level(&mut reader) {
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

use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OadrLoadControlStateType {
    pub oadr_capacity: Option<crate::oadr20b::oadr::OadrLoadControlStateTypeType>,
    pub oadr_level_offset: Option<crate::oadr20b::oadr::OadrLoadControlStateTypeType>,
    pub oadr_percent_offset: Option<crate::oadr20b::oadr::OadrLoadControlStateTypeType>,
    pub oadr_set_point: Option<crate::oadr20b::oadr::OadrLoadControlStateTypeType>,
}

impl OadrLoadControlStateType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        if let Some(elem) = &self.oadr_capacity {
            elem.write_with_name(writer, "oadr:oadrCapacity", false, false)?;
        }
        if let Some(elem) = &self.oadr_level_offset {
            elem.write_with_name(writer, "oadr:oadrLevelOffset", false, false)?;
        }
        if let Some(elem) = &self.oadr_percent_offset {
            elem.write_with_name(writer, "oadr:oadrPercentOffset", false, false)?;
        }
        if let Some(elem) = &self.oadr_set_point {
            elem.write_with_name(writer, "oadr:oadrSetPoint", false, false)?;
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
            start.attr("xsi:type", "oadrLoadControlStateType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrLoadControlStateType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrLoadControlState", true, false)?;
        Ok(())
    }
}

impl OadrLoadControlStateType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_capacity: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrLoadControlStateTypeType,
        > = Default::default();
        let mut oadr_level_offset: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrLoadControlStateTypeType,
        > = Default::default();
        let mut oadr_percent_offset: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrLoadControlStateTypeType,
        > = Default::default();
        let mut oadr_set_point: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrLoadControlStateTypeType,
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
                    "oadrCapacity" => oadr_capacity.set(
                        crate::oadr20b::oadr::OadrLoadControlStateTypeType::read(
                            reader,
                            &attributes,
                            "oadrCapacity",
                        )?,
                    )?,
                    "oadrLevelOffset" => oadr_level_offset.set(
                        crate::oadr20b::oadr::OadrLoadControlStateTypeType::read(
                            reader,
                            &attributes,
                            "oadrLevelOffset",
                        )?,
                    )?,
                    "oadrPercentOffset" => oadr_percent_offset.set(
                        crate::oadr20b::oadr::OadrLoadControlStateTypeType::read(
                            reader,
                            &attributes,
                            "oadrPercentOffset",
                        )?,
                    )?,
                    "oadrSetPoint" => oadr_set_point.set(
                        crate::oadr20b::oadr::OadrLoadControlStateTypeType::read(
                            reader,
                            &attributes,
                            "oadrSetPoint",
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
        Ok(OadrLoadControlStateType {
            oadr_capacity: oadr_capacity.get(),
            oadr_level_offset: oadr_level_offset.get(),
            oadr_percent_offset: oadr_percent_offset.get(),
            oadr_set_point: oadr_set_point.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrLoadControlState")?;
        OadrLoadControlStateType::read(reader, &attr, "oadrLoadControlState")
    }
}

impl crate::xsd_util::ReadXml for OadrLoadControlStateType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrLoadControlStateType::read_top_level(&mut reader) {
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

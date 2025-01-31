use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OadrLoadControlStateTypeType {
    pub oadr_min: Option<f32>,
    pub oadr_max: Option<f32>,
    pub oadr_current: f32,
    pub oadr_normal: Option<f32>,
}

impl OadrLoadControlStateTypeType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        if let Some(elem) = &self.oadr_min {
            crate::xsd_util::write_element_using_to_string(writer, "oadr:oadrMin", elem)?;
        }
        if let Some(elem) = &self.oadr_max {
            crate::xsd_util::write_element_using_to_string(writer, "oadr:oadrMax", elem)?;
        }
        crate::xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrCurrent",
            self.oadr_current,
        )?;
        if let Some(elem) = &self.oadr_normal {
            crate::xsd_util::write_element_using_to_string(writer, "oadr:oadrNormal", elem)?;
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
            start.attr("xsi:type", "oadrLoadControlStateTypeType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrLoadControlStateTypeType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(
            &mut writer,
            "oadr:oadrLoadControlStateTypeType",
            true,
            false,
        )?;
        Ok(())
    }
}

impl OadrLoadControlStateTypeType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_min: crate::xsd_util::SetOnce<f32> = Default::default();
        let mut oadr_max: crate::xsd_util::SetOnce<f32> = Default::default();
        let mut oadr_current: crate::xsd_util::SetOnce<f32> = Default::default();
        let mut oadr_normal: crate::xsd_util::SetOnce<f32> = Default::default();

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
                        "oadrMin" => oadr_min
                            .set(crate::xsd_util::read_type_from_string(reader, "oadrMin")?)?,
                        "oadrMax" => oadr_max
                            .set(crate::xsd_util::read_type_from_string(reader, "oadrMax")?)?,
                        "oadrCurrent" => oadr_current.set(
                            crate::xsd_util::read_type_from_string(reader, "oadrCurrent")?,
                        )?,
                        "oadrNormal" => oadr_normal.set(crate::xsd_util::read_type_from_string(
                            reader,
                            "oadrNormal",
                        )?)?,
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
        Ok(OadrLoadControlStateTypeType {
            oadr_min: oadr_min.get(),
            oadr_max: oadr_max.get(),
            oadr_current: oadr_current.require()?,
            oadr_normal: oadr_normal.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrLoadControlStateTypeType")?;
        OadrLoadControlStateTypeType::read(reader, &attr, "oadrLoadControlStateTypeType")
    }
}

impl crate::xsd_util::ReadXml for OadrLoadControlStateTypeType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrLoadControlStateTypeType::read_top_level(&mut reader) {
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

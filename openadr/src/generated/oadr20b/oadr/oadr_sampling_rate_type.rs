use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OadrSamplingRateType {
    /// Minimum sampling period
    pub oadr_min_period: String,
    /// Maximum sampling period
    pub oadr_max_period: String,
    /// If true then the data will be recorded when it changes, but at no greater a frequency than that specified  by minPeriod.
    pub oadr_on_change: bool,
}

impl OadrSamplingRateType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(
            writer,
            "oadr:oadrMinPeriod",
            self.oadr_min_period.as_str(),
        )?;
        crate::xsd_util::write_simple_element(
            writer,
            "oadr:oadrMaxPeriod",
            self.oadr_max_period.as_str(),
        )?;
        crate::xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrOnChange",
            self.oadr_on_change,
        )?;
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
            start.attr("xsi:type", "oadrSamplingRateType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrSamplingRateType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrSamplingRate", true, false)?;
        Ok(())
    }
}

impl OadrSamplingRateType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_min_period: crate::xsd_util::SetOnce<String> = Default::default();
        let mut oadr_max_period: crate::xsd_util::SetOnce<String> = Default::default();
        let mut oadr_on_change: crate::xsd_util::SetOnce<bool> = Default::default();

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
                        "oadrMinPeriod" => oadr_min_period
                            .set(crate::xsd_util::read_string(reader, "oadrMinPeriod")?)?,
                        "oadrMaxPeriod" => oadr_max_period
                            .set(crate::xsd_util::read_string(reader, "oadrMaxPeriod")?)?,
                        "oadrOnChange" => oadr_on_change.set(
                            crate::xsd_util::read_type_from_string(reader, "oadrOnChange")?,
                        )?,
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
        Ok(OadrSamplingRateType {
            oadr_min_period: oadr_min_period.require()?,
            oadr_max_period: oadr_max_period.require()?,
            oadr_on_change: oadr_on_change.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrSamplingRate")?;
        OadrSamplingRateType::read(reader, &attr, "oadrSamplingRate")
    }
}

impl crate::xsd_util::ReadXml for OadrSamplingRateType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrSamplingRateType::read_top_level(&mut reader) {
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

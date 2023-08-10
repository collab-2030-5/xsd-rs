use xml::common::Position;
use xml::writer::*;

/// Describes the subject and attributes of a report.
#[derive(Debug, Clone, PartialEq)]
pub struct OadrReportDescriptionType {
    pub ei_r_id: String,
    pub ei_report_subject: Option<crate::ei::EiTargetType>,
    pub ei_report_data_source: Option<crate::ei::EiTargetType>,
    pub ei_report_type: String,
    /// What is measured or tracked in this report (Units).
    pub emix_item_base: Option<crate::emix::ItemBaseType>,
    pub ei_reading_type: String,
    pub emix_market_context: Option<String>,
    pub oadr_oadr_sampling_rate: Option<crate::oadr::OadrSamplingRateType>,
}

impl OadrReportDescriptionType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:rID", self.ei_r_id.as_str())?;
        if let Some(elem) = &self.ei_report_subject {
            elem.write_with_name(writer, "ei:reportSubject", false, false)?;
        }
        if let Some(elem) = &self.ei_report_data_source {
            elem.write_with_name(writer, "ei:reportDataSource", false, false)?;
        }
        xsd_util::write_simple_element(writer, "ei:reportType", self.ei_report_type.as_str())?;
        if let Some(elem) = &self.emix_item_base {
            elem.write_with_name(writer, "emix:itemBase", false, false)?;
        }
        xsd_util::write_simple_element(writer, "ei:readingType", self.ei_reading_type.as_str())?;
        if let Some(elem) = &self.emix_market_context {
            xsd_util::write_simple_element(writer, "emix:marketContext", elem.as_str())?;
        }
        if let Some(elem) = &self.oadr_oadr_sampling_rate {
            elem.write_with_name(writer, "oadr:oadrSamplingRate", false, false)?;
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
            start.attr("xsi:type", "oadrReportDescriptionType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrReportDescriptionType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrReportDescriptionType", true, false)?;
        Ok(())
    }
}

impl OadrReportDescriptionType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_r_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_report_subject: xsd_util::SetOnce<crate::ei::EiTargetType> = Default::default();
        let mut ei_report_data_source: xsd_util::SetOnce<crate::ei::EiTargetType> =
            Default::default();
        let mut ei_report_type: xsd_util::SetOnce<String> = Default::default();
        let mut emix_item_base: xsd_util::SetOnce<crate::emix::ItemBaseType> = Default::default();
        let mut ei_reading_type: xsd_util::SetOnce<String> = Default::default();
        let mut emix_market_context: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_sampling_rate: xsd_util::SetOnce<crate::oadr::OadrSamplingRateType> =
            Default::default();

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
                    "rID" => ei_r_id.set(xsd_util::read_string(reader, "rID")?)?,
                    "reportSubject" => ei_report_subject.set(crate::ei::EiTargetType::read(
                        reader,
                        &attributes,
                        "reportSubject",
                    )?)?,
                    "reportDataSource" => ei_report_data_source.set(
                        crate::ei::EiTargetType::read(reader, &attributes, "reportDataSource")?,
                    )?,
                    "reportType" => {
                        ei_report_type.set(xsd_util::read_string(reader, "reportType")?)?
                    }
                    "itemBase" => emix_item_base.set(crate::emix::ItemBaseType::read(
                        reader,
                        &attributes,
                        "itemBase",
                    )?)?,
                    "readingType" => {
                        ei_reading_type.set(xsd_util::read_string(reader, "readingType")?)?
                    }
                    "marketContext" => {
                        emix_market_context.set(xsd_util::read_string(reader, "marketContext")?)?
                    }
                    "oadrSamplingRate" => {
                        oadr_oadr_sampling_rate.set(crate::oadr::OadrSamplingRateType::read(
                            reader,
                            &attributes,
                            "oadrSamplingRate",
                        )?)?
                    }
                    name => {
                        return Err(xsd_api::ReadError::UnexpectedToken(
                            xsd_api::ParentToken(parent_tag.to_owned()),
                            xsd_api::ChildToken(name.to_owned()),
                        ))
                    }
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
        Ok(OadrReportDescriptionType {
            ei_r_id: ei_r_id.require()?,
            ei_report_subject: ei_report_subject.get(),
            ei_report_data_source: ei_report_data_source.get(),
            ei_report_type: ei_report_type.require()?,
            emix_item_base: emix_item_base.get(),
            ei_reading_type: ei_reading_type.require()?,
            emix_market_context: emix_market_context.get(),
            oadr_oadr_sampling_rate: oadr_oadr_sampling_rate.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrReportDescriptionType")?;
        OadrReportDescriptionType::read(reader, &attr, "oadrReportDescriptionType")
    }
}

impl xsd_api::ReadXml for OadrReportDescriptionType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrReportDescriptionType::read_top_level(&mut reader) {
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

use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrReportPayloadType {
    /// A reference to a metadata data point description
    pub ei_r_id: String,
    /// Likely variability of prediction: 0-100
    pub ei_confidence: Option<u32>,
    /// Accuracy in same units as interval payload value
    pub ei_accuracy: Option<f32>,
    pub ei_payload_base: crate::oadr20b::ei::PayloadBaseType,
    /// Enumerated value for the quality of this data item
    pub oadr_oadr_data_quality: Option<crate::oadr20b::oadr::OadrDataQualityTypeType>,
}

impl OadrReportPayloadType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(writer, "ei:rID", self.ei_r_id.as_str())?;
        if let Some(elem) = &self.ei_confidence {
            crate::xsd_util::write_element_using_to_string(writer, "ei:confidence", elem)?;
        }
        if let Some(elem) = &self.ei_accuracy {
            crate::xsd_util::write_element_using_to_string(writer, "ei:accuracy", elem)?;
        }
        self.ei_payload_base.write(writer)?;
        if let Some(elem) = &self.oadr_oadr_data_quality {
            elem.write(writer)?;
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
            start.attr("xsi:type", "oadrReportPayloadType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrReportPayloadType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrReportPayload", true, false)?;
        Ok(())
    }
}

impl OadrReportPayloadType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        _attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_r_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_confidence: crate::xsd_util::SetOnce<u32> = Default::default();
        let mut ei_accuracy: crate::xsd_util::SetOnce<f32> = Default::default();
        let mut ei_payload_base: crate::xsd_util::SetOnce<crate::oadr20b::ei::PayloadBaseType> =
            Default::default();
        let mut oadr_oadr_data_quality: crate::xsd_util::SetOnce<
            crate::oadr20b::oadr::OadrDataQualityTypeType,
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
                    "rID" => ei_r_id.set(crate::xsd_util::read_string(reader, "rID")?)?,
                    "confidence" => ei_confidence.set(crate::xsd_util::read_type_from_string(
                        reader,
                        "confidence",
                    )?)?,
                    "accuracy" => ei_accuracy
                        .set(crate::xsd_util::read_type_from_string(reader, "accuracy")?)?,
                    "oadrPayloadResourceStatus" => ei_payload_base.set(
                        crate::oadr20b::ei::PayloadBaseType::OadrPayloadResourceStatus(
                            crate::oadr20b::oadr::OadrPayloadResourceStatusType::read(
                                reader,
                                &attributes,
                                "oadrPayloadResourceStatus",
                            )?,
                        ),
                    )?,
                    "payloadFloat" => {
                        ei_payload_base.set(crate::oadr20b::ei::PayloadBaseType::PayloadFloat(
                            crate::oadr20b::ei::PayloadFloatType::read(
                                reader,
                                &attributes,
                                "payloadFloat",
                            )?,
                        ))?
                    }
                    "oadrDataQuality" => oadr_oadr_data_quality.set(
                        crate::oadr_data_quality_type_type::read_choice_enum(
                            reader,
                            "oadrDataQuality",
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
        Ok(OadrReportPayloadType {
            ei_r_id: ei_r_id.require()?,
            ei_confidence: ei_confidence.get(),
            ei_accuracy: ei_accuracy.get(),
            ei_payload_base: ei_payload_base.require()?,
            oadr_oadr_data_quality: oadr_oadr_data_quality.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrReportPayload")?;
        OadrReportPayloadType::read(reader, &attr, "oadrReportPayload")
    }
}

impl crate::xsd_util::ReadXml for OadrReportPayloadType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrReportPayloadType::read_top_level(&mut reader) {
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

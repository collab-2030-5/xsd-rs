use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReportPayloadType {
    /// A reference to a metadata data point description
    pub ei_r_id: String,
    /// Likely variability of prediction: 0-100
    pub ei_confidence: Option<u32>,
    /// Accuracy in same units as interval payload value
    pub ei_accuracy: Option<f32>,
    pub ei_payload_base: base::PayloadBaseType,
}

impl ReportPayloadType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:rID", self.ei_r_id.as_str())?;
        if let Some(elem) = &self.ei_confidence {
            xsd_util::write_element_using_to_string(writer, "ei:confidence", elem)?;
        }
        if let Some(elem) = &self.ei_accuracy {
            xsd_util::write_element_using_to_string(writer, "ei:accuracy", elem)?;
        }
        self.ei_payload_base
            .write_with_name(writer, "ei:payloadBase")?;
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
            start.attr("xsi:type", "ei:ReportPayloadType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for ReportPayloadType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:ReportPayloadType", true, false)?;
        Ok(())
    }
}

impl ReportPayloadType {
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
        let mut ei_confidence: xsd_util::SetOnce<u32> = Default::default();
        let mut ei_accuracy: xsd_util::SetOnce<f32> = Default::default();
        let mut ei_payload_base: xsd_util::SetOnce<base::PayloadBaseType> = Default::default();

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
                    "ei:rID" => ei_r_id.set(xsd_util::read_string(reader, "ei:rID")?)?,
                    "ei:confidence" => ei_confidence
                        .set(xsd_util::read_type_from_string(reader, "ei:confidence")?)?,
                    "ei:accuracy" => {
                        ei_accuracy.set(xsd_util::read_type_from_string(reader, "ei:accuracy")?)?
                    }
                    "ei:payloadBase" => ei_payload_base.set(base::PayloadBaseType::read(
                        reader,
                        &attributes,
                        "ei:payloadBase",
                    )?)?,
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
        Ok(ReportPayloadType {
            ei_r_id: ei_r_id.require()?,
            ei_confidence: ei_confidence.get(),
            ei_accuracy: ei_accuracy.get(),
            ei_payload_base: ei_payload_base.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "ReportPayloadType")?;
        ReportPayloadType::read(reader, &attr, "ei:ReportPayloadType")
    }
}

impl xsd_api::ReadXml for ReportPayloadType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match ReportPayloadType::read_top_level(&mut reader) {
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

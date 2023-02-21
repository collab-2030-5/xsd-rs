use xml::common::Position;
use xml::writer::*;

/// Opts are used by the VEN to temporarily override the pre-existing agreement. For example, a VEN may opt in to events during the evening, or opt out from events during the world series.
#[derive(Debug, Clone, PartialEq)]
pub struct EiOptType {
    pub ei_opt_id: String,
    pub ei_opt_type: crate::ei::OptTypeType,
    pub ei_opt_reason: String,
    pub emix_market_context: Option<String>,
    pub ei_ven_id: String,
    pub xcal_vavailability: Option<crate::xcal::VavailabilityType>,
    pub ei_created_date_time: String,
    pub ei_schema_version: Option<String>,
}

impl EiOptType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "ei:optID", self.ei_opt_id.as_str())?;
        xsd_util::write_string_enumeration(writer, "ei:optType", self.ei_opt_type)?;
        xsd_util::write_simple_element(writer, "ei:optReason", self.ei_opt_reason.as_str())?;
        if let Some(elem) = &self.emix_market_context {
            xsd_util::write_simple_element(writer, "emix:marketContext", elem.as_str())?;
        }
        xsd_util::write_simple_element(writer, "ei:venID", self.ei_ven_id.as_str())?;
        if let Some(elem) = &self.xcal_vavailability {
            elem.write_with_name(writer, "xcal:vavailability", false, false)?;
        }
        xsd_util::write_simple_element(
            writer,
            "ei:createdDateTime",
            self.ei_created_date_time.as_str(),
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
        // ---- start attributes ----
        let start = match &self.ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "EiOptType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EiOptType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:EiOptType", true, false)?;
        Ok(())
    }
}

impl EiOptType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_opt_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_opt_type: xsd_util::SetOnce<crate::ei::OptTypeType> = Default::default();
        let mut ei_opt_reason: xsd_util::SetOnce<String> = Default::default();
        let mut emix_market_context: xsd_util::SetOnce<String> = Default::default();
        let mut ei_ven_id: xsd_util::SetOnce<String> = Default::default();
        let mut xcal_vavailability: xsd_util::SetOnce<crate::xcal::VavailabilityType> =
            Default::default();
        let mut ei_created_date_time: xsd_util::SetOnce<String> = Default::default();
        let mut ei_schema_version: xsd_util::SetOnce<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "ei:schemaVersion" => ei_schema_version.set(attr.value.clone())?,
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
                    "optID" => ei_opt_id.set(xsd_util::read_string(reader, "optID")?)?,
                    "optType" => ei_opt_type.set(xsd_util::read_string_enum(reader, "optType")?)?,
                    "optReason" => {
                        ei_opt_reason.set(xsd_util::read_string(reader, "optReason")?)?
                    }
                    "marketContext" => {
                        emix_market_context.set(xsd_util::read_string(reader, "marketContext")?)?
                    }
                    "venID" => ei_ven_id.set(xsd_util::read_string(reader, "venID")?)?,
                    "vavailability" => xcal_vavailability.set(
                        crate::xcal::VavailabilityType::read(reader, &attributes, "vavailability")?,
                    )?,
                    "createdDateTime" => ei_created_date_time
                        .set(xsd_util::read_string(reader, "createdDateTime")?)?,
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
        Ok(EiOptType {
            ei_opt_id: ei_opt_id.require()?,
            ei_opt_type: ei_opt_type.require()?,
            ei_opt_reason: ei_opt_reason.require()?,
            emix_market_context: emix_market_context.get(),
            ei_ven_id: ei_ven_id.require()?,
            xcal_vavailability: xcal_vavailability.get(),
            ei_created_date_time: ei_created_date_time.require()?,
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "EiOptType")?;
        EiOptType::read(reader, &attr, "EiOptType")
    }
}

impl xsd_api::ReadXml for EiOptType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiOptType::read_top_level(&mut reader) {
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

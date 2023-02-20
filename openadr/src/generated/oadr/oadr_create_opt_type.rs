use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrCreateOptType {
    pub ei_opt_id: String,
    pub ei_opt_type: crate::ei::OptTypeType,
    pub ei_opt_reason: String,
    pub emix_market_context: Option<String>,
    pub ei_ven_id: String,
    pub xcal_vavailability: Option<crate::xcal::VavailabilityType>,
    pub ei_created_date_time: String,
    pub ei_schema_version: Option<String>,
    pub pyld_request_id: String,
    pub ei_qualified_event_id: Option<crate::ei::QualifiedEventIdType>,
    pub ei_ei_target: crate::ei::EiTargetType,
    pub oadr_oadr_device_class: Option<crate::ei::EiTargetType>,
}

impl OadrCreateOptType {
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
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        if let Some(elem) = &self.ei_qualified_event_id {
            elem.write_with_name(writer, "ei:qualifiedEventID", false, false)?;
        }
        self.ei_ei_target
            .write_with_name(writer, "ei:eiTarget", false, false)?;
        if let Some(elem) = &self.oadr_oadr_device_class {
            elem.write_with_name(writer, "oadr:oadrDeviceClass", false, false)?;
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
        // ---- start attributes ----
        let start = match &self.ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadr:oadrCreateOptType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrCreateOptType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrCreateOptType", true, false)?;
        Ok(())
    }
}

impl OadrCreateOptType {
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
        let mut pyld_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_qualified_event_id: xsd_util::SetOnce<crate::ei::QualifiedEventIdType> =
            Default::default();
        let mut ei_ei_target: xsd_util::SetOnce<crate::ei::EiTargetType> = Default::default();
        let mut oadr_oadr_device_class: xsd_util::SetOnce<crate::ei::EiTargetType> =
            Default::default();

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
                    "ei:optID" => ei_opt_id.set(xsd_util::read_string(reader, "ei:optID")?)?,
                    "ei:optType" => {
                        ei_opt_type.set(xsd_util::read_string_enum(reader, "ei:optType")?)?
                    }
                    "ei:optReason" => {
                        ei_opt_reason.set(xsd_util::read_string(reader, "ei:optReason")?)?
                    }
                    "emix:marketContext" => emix_market_context
                        .set(xsd_util::read_string(reader, "emix:marketContext")?)?,
                    "ei:venID" => ei_ven_id.set(xsd_util::read_string(reader, "ei:venID")?)?,
                    "xcal:vavailability" => {
                        xcal_vavailability.set(crate::xcal::VavailabilityType::read(
                            reader,
                            &attributes,
                            "xcal:vavailability",
                        )?)?
                    }
                    "ei:createdDateTime" => ei_created_date_time
                        .set(xsd_util::read_string(reader, "ei:createdDateTime")?)?,
                    "pyld:requestID" => {
                        pyld_request_id.set(xsd_util::read_string(reader, "pyld:requestID")?)?
                    }
                    "ei:qualifiedEventID" => {
                        ei_qualified_event_id.set(crate::ei::QualifiedEventIdType::read(
                            reader,
                            &attributes,
                            "ei:qualifiedEventID",
                        )?)?
                    }
                    "ei:eiTarget" => ei_ei_target.set(crate::ei::EiTargetType::read(
                        reader,
                        &attributes,
                        "ei:eiTarget",
                    )?)?,
                    "oadr:oadrDeviceClass" => oadr_oadr_device_class.set(
                        crate::ei::EiTargetType::read(reader, &attributes, "oadr:oadrDeviceClass")?,
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
        Ok(OadrCreateOptType {
            ei_opt_id: ei_opt_id.require()?,
            ei_opt_type: ei_opt_type.require()?,
            ei_opt_reason: ei_opt_reason.require()?,
            emix_market_context: emix_market_context.get(),
            ei_ven_id: ei_ven_id.require()?,
            xcal_vavailability: xcal_vavailability.get(),
            ei_created_date_time: ei_created_date_time.require()?,
            ei_schema_version: ei_schema_version.get(),
            pyld_request_id: pyld_request_id.require()?,
            ei_qualified_event_id: ei_qualified_event_id.get(),
            ei_ei_target: ei_ei_target.require()?,
            oadr_oadr_device_class: oadr_oadr_device_class.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrCreateOptType")?;
        OadrCreateOptType::read(reader, &attr, "oadr:oadrCreateOptType")
    }
}

impl xsd_api::ReadXml for OadrCreateOptType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCreateOptType::read_top_level(&mut reader) {
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
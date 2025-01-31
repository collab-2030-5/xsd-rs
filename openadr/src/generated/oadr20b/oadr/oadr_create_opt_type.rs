use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OadrCreateOptType {
    pub ei_opt_id: String,
    pub ei_opt_type: crate::oadr20b::ei::OptTypeType,
    pub ei_opt_reason: crate::oadr20b::ei::OptReasonType,
    pub emix_market_context: Option<String>,
    pub ei_ven_id: String,
    pub xcal_vavailability: Option<crate::oadr20b::xcal::VavailabilityType>,
    pub ei_created_date_time: String,
    pub ei_schema_version: Option<crate::oadr20b::ei::SchemaVersionType>,
    pub pyld_request_id: String,
    pub ei_qualified_event_id: Option<crate::oadr20b::ei::QualifiedEventIdType>,
    pub ei_ei_target: crate::oadr20b::ei::EiTargetType,
    pub oadr_oadr_device_class: Option<crate::oadr20b::ei::EiTargetType>,
}

impl OadrCreateOptType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        crate::xsd_util::write_simple_element(writer, "ei:optID", self.ei_opt_id.as_str())?;
        crate::xsd_util::write_string_enumeration(writer, "ei:optType", &self.ei_opt_type)?;
        self.ei_opt_reason.write(writer)?;
        if let Some(elem) = &self.emix_market_context {
            crate::xsd_util::write_simple_element(writer, "emix:marketContext", elem.as_str())?;
        }
        crate::xsd_util::write_simple_element(writer, "ei:venID", self.ei_ven_id.as_str())?;
        if let Some(elem) = &self.xcal_vavailability {
            elem.write_with_name(writer, "xcal:vavailability", false, false)?;
        }
        crate::xsd_util::write_simple_element(
            writer,
            "ei:createdDateTime",
            self.ei_created_date_time.as_str(),
        )?;
        crate::xsd_util::write_simple_element(
            writer,
            "pyld:requestID",
            self.pyld_request_id.as_str(),
        )?;
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
        let ei_schema_version = self
            .ei_schema_version
            .as_ref()
            .map(|x| x.as_str().to_string());
        let start = match &ei_schema_version {
            Some(attr) => start.attr("ei:schemaVersion", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadrCreateOptType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl crate::xsd_util::WriteXml for OadrCreateOptType {
    fn write<W>(
        &self,
        config: crate::xsd_util::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), crate::xsd_util::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrCreateOpt", true, false)?;
        Ok(())
    }
}

impl OadrCreateOptType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &[xml::attribute::OwnedAttribute],
        parent_tag: &str,
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_opt_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_opt_type: crate::xsd_util::SetOnce<crate::oadr20b::ei::OptTypeType> =
            Default::default();
        let mut ei_opt_reason: crate::xsd_util::SetOnce<crate::oadr20b::ei::OptReasonType> =
            Default::default();
        let mut emix_market_context: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_ven_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut xcal_vavailability: crate::xsd_util::SetOnce<
            crate::oadr20b::xcal::VavailabilityType,
        > = Default::default();
        let mut ei_created_date_time: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_schema_version: crate::xsd_util::SetOnce<crate::oadr20b::ei::SchemaVersionType> =
            Default::default();
        let mut pyld_request_id: crate::xsd_util::SetOnce<String> = Default::default();
        let mut ei_qualified_event_id: crate::xsd_util::SetOnce<
            crate::oadr20b::ei::QualifiedEventIdType,
        > = Default::default();
        let mut ei_ei_target: crate::xsd_util::SetOnce<crate::oadr20b::ei::EiTargetType> =
            Default::default();
        let mut oadr_oadr_device_class: crate::xsd_util::SetOnce<crate::oadr20b::ei::EiTargetType> =
            Default::default();

        #[allow(clippy::single_match)]
        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "schemaVersion" => ei_schema_version.set(
                    crate::schema_version_type::convert_string_to_enum(&attr.value)?,
                )?,
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
                        return Err(crate::xsd_util::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    match name.local_name.as_str() {
                        "optID" => ei_opt_id.set(crate::xsd_util::read_string(reader, "optID")?)?,
                        "optType" => ei_opt_type
                            .set(crate::xsd_util::read_string_enum(reader, "optType")?)?,
                        "optReason" => ei_opt_reason.set(
                            crate::opt_reason_type::read_choice_enum(reader, "optReason")?,
                        )?,
                        "marketContext" => emix_market_context
                            .set(crate::xsd_util::read_string(reader, "marketContext")?)?,
                        "venID" => ei_ven_id.set(crate::xsd_util::read_string(reader, "venID")?)?,
                        "vavailability" => xcal_vavailability.set(
                            crate::oadr20b::xcal::VavailabilityType::read(
                                reader,
                                &attributes,
                                "vavailability",
                            )?,
                        )?,
                        "createdDateTime" => ei_created_date_time
                            .set(crate::xsd_util::read_string(reader, "createdDateTime")?)?,
                        "requestID" => pyld_request_id
                            .set(crate::xsd_util::read_string(reader, "requestID")?)?,
                        "qualifiedEventID" => ei_qualified_event_id.set(
                            crate::oadr20b::ei::QualifiedEventIdType::read(
                                reader,
                                &attributes,
                                "qualifiedEventID",
                            )?,
                        )?,
                        "eiTarget" => ei_ei_target.set(crate::oadr20b::ei::EiTargetType::read(
                            reader,
                            &attributes,
                            "eiTarget",
                        )?)?,
                        "oadrDeviceClass" => {
                            oadr_oadr_device_class.set(crate::oadr20b::ei::EiTargetType::read(
                                reader,
                                &attributes,
                                "oadrDeviceClass",
                            )?)?
                        }
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
    ) -> core::result::Result<Self, crate::xsd_util::ReadError>
    where
        R: std::io::Read,
    {
        let attr = crate::xsd_util::read_start_tag(reader, "oadrCreateOpt")?;
        OadrCreateOptType::read(reader, &attr, "oadrCreateOpt")
    }
}

impl crate::xsd_util::ReadXml for OadrCreateOptType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, crate::xsd_util::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCreateOptType::read_top_level(&mut reader) {
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

use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrCreatedPartyRegistrationType {
    pub ei_ei_response: crate::ei::EiResponseType,
    pub ei_registration_id: Option<crate::ei::RegistrationId>,
    /// venID not included in query unless already registered
    pub ei_ven_id: Option<String>,
    pub ei_vtn_id: String,
    /// VTN response to query registration returns all supported. This element is not required for a registration  response
    pub oadr_oadr_profiles: crate::oadr::OadrProfiles,
    /// HTTP Pull Only - The VEN shall send an oadrPoll payload to the VTN at most once for each duration specified by this element
    pub oadr_oadr_requested_oadr_poll_freq: Option<crate::xcal::DurationPropType>,
    pub oadr_oadr_service_specific_info: Option<crate::oadr::OadrServiceSpecificInfo>,
    pub oadr_extensions: Option<crate::oadr::OadrExtensions>,
    pub ei_schema_version: Option<String>,
}

impl OadrCreatedPartyRegistrationType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.ei_ei_response
            .write_with_name(writer, "ei:eiResponse", false, false)?;
        if let Some(elem) = &self.ei_registration_id {
            elem.write_with_name(writer, "ei:registrationID", false, false)?;
        }
        if let Some(elem) = &self.ei_ven_id {
            xsd_util::write_simple_element(writer, "ei:venID", elem.as_str())?;
        }
        xsd_util::write_simple_element(writer, "ei:vtnID", self.ei_vtn_id.as_str())?;
        self.oadr_oadr_profiles
            .write_with_name(writer, "oadr:oadrProfiles", false, false)?;
        if let Some(elem) = &self.oadr_oadr_requested_oadr_poll_freq {
            elem.write_with_name(writer, "oadr:oadrRequestedOadrPollFreq", false, false)?;
        }
        if let Some(elem) = &self.oadr_oadr_service_specific_info {
            elem.write_with_name(writer, "oadr:oadrServiceSpecificInfo", false, false)?;
        }
        if let Some(elem) = &self.oadr_extensions {
            elem.write_with_name(writer, "oadrExtensions", false, false)?;
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
            start.attr("xsi:type", "oadrCreatedPartyRegistrationType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrCreatedPartyRegistrationType {
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
            "oadr:oadrCreatedPartyRegistrationType",
            true,
            false,
        )?;
        Ok(())
    }
}

impl OadrCreatedPartyRegistrationType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ei_ei_response: xsd_util::SetOnce<crate::ei::EiResponseType> = Default::default();
        let mut ei_registration_id: xsd_util::SetOnce<crate::ei::RegistrationId> =
            Default::default();
        let mut ei_ven_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_vtn_id: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_profiles: xsd_util::SetOnce<crate::oadr::OadrProfiles> =
            Default::default();
        let mut oadr_oadr_requested_oadr_poll_freq: xsd_util::SetOnce<
            crate::xcal::DurationPropType,
        > = Default::default();
        let mut oadr_oadr_service_specific_info: xsd_util::SetOnce<
            crate::oadr::OadrServiceSpecificInfo,
        > = Default::default();
        let mut oadr_extensions: xsd_util::SetOnce<crate::oadr::OadrExtensions> =
            Default::default();
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
                } => {
                    match name.local_name.as_str() {
                        "eiResponse" => ei_ei_response.set(crate::ei::EiResponseType::read(
                            reader,
                            &attributes,
                            "eiResponse",
                        )?)?,
                        "registrationID" => ei_registration_id.set(
                            crate::ei::RegistrationId::read(reader, &attributes, "registrationID")?,
                        )?,
                        "venID" => ei_ven_id.set(xsd_util::read_string(reader, "venID")?)?,
                        "vtnID" => ei_vtn_id.set(xsd_util::read_string(reader, "vtnID")?)?,
                        "oadrProfiles" => oadr_oadr_profiles.set(
                            crate::oadr::OadrProfiles::read(reader, &attributes, "oadrProfiles")?,
                        )?,
                        "oadrRequestedOadrPollFreq" => oadr_oadr_requested_oadr_poll_freq.set(
                            crate::xcal::DurationPropType::read(
                                reader,
                                &attributes,
                                "oadrRequestedOadrPollFreq",
                            )?,
                        )?,
                        "oadrServiceSpecificInfo" => oadr_oadr_service_specific_info.set(
                            crate::oadr::OadrServiceSpecificInfo::read(
                                reader,
                                &attributes,
                                "oadrServiceSpecificInfo",
                            )?,
                        )?,
                        "oadrExtensions" => {
                            oadr_extensions.set(crate::oadr::OadrExtensions::read(
                                reader,
                                &attributes,
                                "oadrExtensions",
                            )?)?
                        }
                        name => {
                            return Err(xsd_api::ReadError::UnexpectedToken(
                                xsd_api::ParentToken(parent_tag.to_owned()),
                                xsd_api::ChildToken(name.to_owned()),
                            ))
                        }
                    }
                }
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
        Ok(OadrCreatedPartyRegistrationType {
            ei_ei_response: ei_ei_response.require()?,
            ei_registration_id: ei_registration_id.get(),
            ei_ven_id: ei_ven_id.get(),
            ei_vtn_id: ei_vtn_id.require()?,
            oadr_oadr_profiles: oadr_oadr_profiles.require()?,
            oadr_oadr_requested_oadr_poll_freq: oadr_oadr_requested_oadr_poll_freq.get(),
            oadr_oadr_service_specific_info: oadr_oadr_service_specific_info.get(),
            oadr_extensions: oadr_extensions.get(),
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrCreatedPartyRegistrationType")?;
        OadrCreatedPartyRegistrationType::read(reader, &attr, "oadrCreatedPartyRegistrationType")
    }
}

impl xsd_api::ReadXml for OadrCreatedPartyRegistrationType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCreatedPartyRegistrationType::read_top_level(&mut reader) {
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

use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrCreatePartyRegistrationType {
    pub pyld_request_id: String,
    /// Used for re-registering an existing registration
    pub ei_registration_id: Option<crate::ei::RegistrationId>,
    pub ei_ven_id: Option<String>,
    pub oadr_oadr_profile_name: String,
    pub oadr_oadr_transport_name: crate::oadr::OadrTransportType,
    /// Address of this VEN. Not required if http pull model
    pub oadr_oadr_transport_address: Option<String>,
    /// ReportOnlyDeviceFlag - True or False
    pub oadr_oadr_report_only: bool,
    /// Implementation supports XML signatures - True or False
    pub oadr_oadr_xml_signature: bool,
    /// Human readable name for VEN
    pub oadr_oadr_ven_name: Option<String>,
    /// If transport is simpleHttp indicate if VEN is operating in pull exchange model - true or false
    pub oadr_oadr_http_pull_model: Option<bool>,
    pub ei_schema_version: Option<String>,
}

impl OadrCreatePartyRegistrationType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_simple_element(writer, "pyld:requestID", self.pyld_request_id.as_str())?;
        if let Some(elem) = &self.ei_registration_id {
            elem.write_with_name(writer, "ei:registrationID", false, false)?;
        }
        if let Some(elem) = &self.ei_ven_id {
            xsd_util::write_simple_element(writer, "ei:venID", elem.as_str())?;
        }
        xsd_util::write_simple_element(
            writer,
            "oadr:oadrProfileName",
            self.oadr_oadr_profile_name.as_str(),
        )?;
        xsd_util::write_string_enumeration(
            writer,
            "oadr:oadrTransportName",
            self.oadr_oadr_transport_name,
        )?;
        if let Some(elem) = &self.oadr_oadr_transport_address {
            xsd_util::write_simple_element(writer, "oadr:oadrTransportAddress", elem.as_str())?;
        }
        xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrReportOnly",
            self.oadr_oadr_report_only,
        )?;
        xsd_util::write_element_using_to_string(
            writer,
            "oadr:oadrXmlSignature",
            self.oadr_oadr_xml_signature,
        )?;
        if let Some(elem) = &self.oadr_oadr_ven_name {
            xsd_util::write_simple_element(writer, "oadr:oadrVenName", elem.as_str())?;
        }
        if let Some(elem) = &self.oadr_oadr_http_pull_model {
            xsd_util::write_element_using_to_string(writer, "oadr:oadrHttpPullModel", elem)?;
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
            start.attr("xsi:type", "oadrCreatePartyRegistrationType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrCreatePartyRegistrationType {
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
            "oadr:oadrCreatePartyRegistrationType",
            true,
            false,
        )?;
        Ok(())
    }
}

impl OadrCreatePartyRegistrationType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut pyld_request_id: xsd_util::SetOnce<String> = Default::default();
        let mut ei_registration_id: xsd_util::SetOnce<crate::ei::RegistrationId> =
            Default::default();
        let mut ei_ven_id: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_profile_name: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_transport_name: xsd_util::SetOnce<crate::oadr::OadrTransportType> =
            Default::default();
        let mut oadr_oadr_transport_address: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_report_only: xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_oadr_xml_signature: xsd_util::SetOnce<bool> = Default::default();
        let mut oadr_oadr_ven_name: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_oadr_http_pull_model: xsd_util::SetOnce<bool> = Default::default();
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
                        "requestID" => {
                            pyld_request_id.set(xsd_util::read_string(reader, "requestID")?)?
                        }
                        "registrationID" => ei_registration_id.set(
                            crate::ei::RegistrationId::read(reader, &attributes, "registrationID")?,
                        )?,
                        "venID" => ei_ven_id.set(xsd_util::read_string(reader, "venID")?)?,
                        "oadrProfileName" => oadr_oadr_profile_name
                            .set(xsd_util::read_string(reader, "oadrProfileName")?)?,
                        "oadrTransportName" => oadr_oadr_transport_name
                            .set(xsd_util::read_string_enum(reader, "oadrTransportName")?)?,
                        "oadrTransportAddress" => oadr_oadr_transport_address
                            .set(xsd_util::read_string(reader, "oadrTransportAddress")?)?,
                        "oadrReportOnly" => oadr_oadr_report_only
                            .set(xsd_util::read_type_from_string(reader, "oadrReportOnly")?)?,
                        "oadrXmlSignature" => oadr_oadr_xml_signature
                            .set(xsd_util::read_type_from_string(reader, "oadrXmlSignature")?)?,
                        "oadrVenName" => {
                            oadr_oadr_ven_name.set(xsd_util::read_string(reader, "oadrVenName")?)?
                        }
                        "oadrHttpPullModel" => oadr_oadr_http_pull_model.set(
                            xsd_util::read_type_from_string(reader, "oadrHttpPullModel")?,
                        )?,
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
        Ok(OadrCreatePartyRegistrationType {
            pyld_request_id: pyld_request_id.require()?,
            ei_registration_id: ei_registration_id.get(),
            ei_ven_id: ei_ven_id.get(),
            oadr_oadr_profile_name: oadr_oadr_profile_name.require()?,
            oadr_oadr_transport_name: oadr_oadr_transport_name.require()?,
            oadr_oadr_transport_address: oadr_oadr_transport_address.get(),
            oadr_oadr_report_only: oadr_oadr_report_only.require()?,
            oadr_oadr_xml_signature: oadr_oadr_xml_signature.require()?,
            oadr_oadr_ven_name: oadr_oadr_ven_name.get(),
            oadr_oadr_http_pull_model: oadr_oadr_http_pull_model.get(),
            ei_schema_version: ei_schema_version.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrCreatePartyRegistrationType")?;
        OadrCreatePartyRegistrationType::read(reader, &attr, "oadrCreatePartyRegistrationType")
    }
}

impl xsd_api::ReadXml for OadrCreatePartyRegistrationType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrCreatePartyRegistrationType::read_top_level(&mut reader) {
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

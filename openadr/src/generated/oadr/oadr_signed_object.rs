use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrSignedObject {
    pub id: Option<String>,
    pub oadr_signed_object_choice: crate::oadr::OadrSignedObjectChoice,
}

impl OadrSignedObject {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.oadr_signed_object_choice.write(writer)?;
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
        let start = match &self.id {
            Some(attr) => start.attr("oadr:Id", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "oadrSignedObject")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrSignedObject {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrSignedObject", true, false)?;
        Ok(())
    }
}

impl OadrSignedObject {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut id: xsd_util::SetOnce<String> = Default::default();
        let mut oadr_signed_object_choice: xsd_util::SetOnce<crate::oadr::OadrSignedObjectChoice> =
            Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "Id" => id.set(attr.value.clone())?,
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
                    "oadrDistributeEvent" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrDistributeEvent(
                            crate::oadr::OadrDistributeEventType::read(
                                reader,
                                &attributes,
                                "oadrDistributeEvent",
                            )?,
                        ),
                    )?,
                    "oadrCreatedEvent" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreatedEvent(
                            crate::oadr::OadrCreatedEventType::read(
                                reader,
                                &attributes,
                                "oadrCreatedEvent",
                            )?,
                        ),
                    )?,
                    "oadrRequestEvent" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrRequestEvent(
                            crate::oadr::OadrRequestEventType::read(
                                reader,
                                &attributes,
                                "oadrRequestEvent",
                            )?,
                        ),
                    )?,
                    "oadrResponse" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrResponse(
                            crate::oadr::OadrResponseType::read(
                                reader,
                                &attributes,
                                "oadrResponse",
                            )?,
                        ),
                    )?,
                    "oadrCancelOpt" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCancelOpt(
                            crate::oadr::OadrCancelOptType::read(
                                reader,
                                &attributes,
                                "oadrCancelOpt",
                            )?,
                        ),
                    )?,
                    "oadrCanceledOpt" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCanceledOpt(
                            crate::oadr::OadrCanceledOptType::read(
                                reader,
                                &attributes,
                                "oadrCanceledOpt",
                            )?,
                        ),
                    )?,
                    "oadrCreateOpt" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreateOpt(
                            crate::oadr::OadrCreateOptType::read(
                                reader,
                                &attributes,
                                "oadrCreateOpt",
                            )?,
                        ),
                    )?,
                    "oadrCreatedOpt" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreatedOpt(
                            crate::oadr::OadrCreatedOptType::read(
                                reader,
                                &attributes,
                                "oadrCreatedOpt",
                            )?,
                        ),
                    )?,
                    "oadrCancelReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCancelReport(
                            crate::oadr::OadrCancelReportType::read(
                                reader,
                                &attributes,
                                "oadrCancelReport",
                            )?,
                        ),
                    )?,
                    "oadrCanceledReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCanceledReport(
                            crate::oadr::OadrCanceledReportType::read(
                                reader,
                                &attributes,
                                "oadrCanceledReport",
                            )?,
                        ),
                    )?,
                    "oadrCreateReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreateReport(
                            crate::oadr::OadrCreateReportType::read(
                                reader,
                                &attributes,
                                "oadrCreateReport",
                            )?,
                        ),
                    )?,
                    "oadrCreatedReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreatedReport(
                            crate::oadr::OadrCreatedReportType::read(
                                reader,
                                &attributes,
                                "oadrCreatedReport",
                            )?,
                        ),
                    )?,
                    "oadrRegisterReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrRegisterReport(
                            crate::oadr::OadrRegisterReportType::read(
                                reader,
                                &attributes,
                                "oadrRegisterReport",
                            )?,
                        ),
                    )?,
                    "oadrRegisteredReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrRegisteredReport(
                            crate::oadr::OadrRegisteredReportType::read(
                                reader,
                                &attributes,
                                "oadrRegisteredReport",
                            )?,
                        ),
                    )?,
                    "oadrUpdateReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrUpdateReport(
                            crate::oadr::OadrUpdateReportType::read(
                                reader,
                                &attributes,
                                "oadrUpdateReport",
                            )?,
                        ),
                    )?,
                    "oadrUpdatedReport" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrUpdatedReport(
                            crate::oadr::OadrUpdatedReportType::read(
                                reader,
                                &attributes,
                                "oadrUpdatedReport",
                            )?,
                        ),
                    )?,
                    "oadrCancelPartyRegistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCancelPartyRegistration(
                            crate::oadr::OadrCancelPartyRegistrationType::read(
                                reader,
                                &attributes,
                                "oadrCancelPartyRegistration",
                            )?,
                        ),
                    )?,
                    "oadrCanceledPartyRegistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCanceledPartyRegistration(
                            crate::oadr::OadrCanceledPartyRegistrationType::read(
                                reader,
                                &attributes,
                                "oadrCanceledPartyRegistration",
                            )?,
                        ),
                    )?,
                    "oadrCreatePartyRegistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreatePartyRegistration(
                            crate::oadr::OadrCreatePartyRegistrationType::read(
                                reader,
                                &attributes,
                                "oadrCreatePartyRegistration",
                            )?,
                        ),
                    )?,
                    "oadrCreatedPartyRegistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrCreatedPartyRegistration(
                            crate::oadr::OadrCreatedPartyRegistrationType::read(
                                reader,
                                &attributes,
                                "oadrCreatedPartyRegistration",
                            )?,
                        ),
                    )?,
                    "oadrRequestReregistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrRequestReregistration(
                            crate::oadr::OadrRequestReregistrationType::read(
                                reader,
                                &attributes,
                                "oadrRequestReregistration",
                            )?,
                        ),
                    )?,
                    "oadrQueryRegistration" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrQueryRegistration(
                            crate::oadr::OadrQueryRegistrationType::read(
                                reader,
                                &attributes,
                                "oadrQueryRegistration",
                            )?,
                        ),
                    )?,
                    "oadrPoll" => oadr_signed_object_choice.set(
                        crate::oadr::OadrSignedObjectChoice::OadrOadrPoll(
                            crate::oadr::OadrPollType::read(reader, &attributes, "oadrPoll")?,
                        ),
                    )?,
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
        Ok(OadrSignedObject {
            id: id.get(),
            oadr_signed_object_choice: oadr_signed_object_choice.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrSignedObject")?;
        OadrSignedObject::read(reader, &attr, "oadrSignedObject")
    }
}

impl xsd_api::ReadXml for OadrSignedObject {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrSignedObject::read_top_level(&mut reader) {
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

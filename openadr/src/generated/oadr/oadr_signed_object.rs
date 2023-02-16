use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum OadrSignedObject {
    OadrOadrDistributeEvent(crate::oadr::OadrDistributeEventType),
    OadrOadrCreatedEvent(crate::oadr::OadrCreatedEventType),
    OadrOadrRequestEvent(crate::oadr::OadrRequestEventType),
    OadrOadrResponse(crate::oadr::OadrResponseType),
    OadrOadrCancelOpt(crate::oadr::OadrCancelOptType),
    OadrOadrCanceledOpt(crate::oadr::OadrCanceledOptType),
    OadrOadrCreateOpt(crate::oadr::OadrCreateOptType),
    OadrOadrCreatedOpt(crate::oadr::OadrCreatedOptType),
    OadrOadrCancelReport(crate::oadr::OadrCancelReportType),
    OadrOadrCanceledReport(crate::oadr::OadrCanceledReportType),
    OadrOadrCreateReport(crate::oadr::OadrCreateReportType),
    OadrOadrCreatedReport(crate::oadr::OadrCreatedReportType),
    OadrOadrRegisterReport(crate::oadr::OadrRegisterReportType),
    OadrOadrRegisteredReport(crate::oadr::OadrRegisteredReportType),
    OadrOadrUpdateReport(crate::oadr::OadrUpdateReportType),
    OadrOadrUpdatedReport(crate::oadr::OadrUpdatedReportType),
    OadrOadrCancelPartyRegistration(crate::oadr::OadrCancelPartyRegistrationType),
    OadrOadrCanceledPartyRegistration(crate::oadr::OadrCanceledPartyRegistrationType),
    OadrOadrCreatePartyRegistration(crate::oadr::OadrCreatePartyRegistrationType),
    OadrOadrCreatedPartyRegistration(crate::oadr::OadrCreatedPartyRegistrationType),
    OadrOadrRequestReregistration(crate::oadr::OadrRequestReregistrationType),
    OadrOadrQueryRegistration(crate::oadr::OadrQueryRegistrationType),
    OadrOadrPoll(crate::oadr::OadrPollType),
}

impl OadrSignedObject {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            OadrSignedObject::OadrOadrDistributeEvent(x) => {
                x.write_with_name(writer, "oadr:oadrDistributeEvent", false, false)?;
            }
            OadrSignedObject::OadrOadrCreatedEvent(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedEvent", false, false)?;
            }
            OadrSignedObject::OadrOadrRequestEvent(x) => {
                x.write_with_name(writer, "oadr:oadrRequestEvent", false, false)?;
            }
            OadrSignedObject::OadrOadrResponse(x) => {
                x.write_with_name(writer, "oadr:oadrResponse", false, false)?;
            }
            OadrSignedObject::OadrOadrCancelOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCancelOpt", false, false)?;
            }
            OadrSignedObject::OadrOadrCanceledOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledOpt", false, false)?;
            }
            OadrSignedObject::OadrOadrCreateOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCreateOpt", false, false)?;
            }
            OadrSignedObject::OadrOadrCreatedOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedOpt", false, false)?;
            }
            OadrSignedObject::OadrOadrCancelReport(x) => {
                x.write_with_name(writer, "oadr:oadrCancelReport", false, false)?;
            }
            OadrSignedObject::OadrOadrCanceledReport(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledReport", false, false)?;
            }
            OadrSignedObject::OadrOadrCreateReport(x) => {
                x.write_with_name(writer, "oadr:oadrCreateReport", false, false)?;
            }
            OadrSignedObject::OadrOadrCreatedReport(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedReport", false, false)?;
            }
            OadrSignedObject::OadrOadrRegisterReport(x) => {
                x.write_with_name(writer, "oadr:oadrRegisterReport", false, false)?;
            }
            OadrSignedObject::OadrOadrRegisteredReport(x) => {
                x.write_with_name(writer, "oadr:oadrRegisteredReport", false, false)?;
            }
            OadrSignedObject::OadrOadrUpdateReport(x) => {
                x.write_with_name(writer, "oadr:oadrUpdateReport", false, false)?;
            }
            OadrSignedObject::OadrOadrUpdatedReport(x) => {
                x.write_with_name(writer, "oadr:oadrUpdatedReport", false, false)?;
            }
            OadrSignedObject::OadrOadrCancelPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCancelPartyRegistration", false, false)?;
            }
            OadrSignedObject::OadrOadrCanceledPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledPartyRegistration", false, false)?;
            }
            OadrSignedObject::OadrOadrCreatePartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCreatePartyRegistration", false, false)?;
            }
            OadrSignedObject::OadrOadrCreatedPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedPartyRegistration", false, false)?;
            }
            OadrSignedObject::OadrOadrRequestReregistration(x) => {
                x.write_with_name(writer, "oadr:oadrRequestReregistration", false, false)?;
            }
            OadrSignedObject::OadrOadrQueryRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrQueryRegistration", false, false)?;
            }
            OadrSignedObject::OadrOadrPoll(x) => {
                x.write_with_name(writer, "oadr:oadrPoll", false, false)?;
            }
        }
        Ok(())
    }

    pub(crate) fn read<R>(
        _reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        unimplemented!()
    }
}

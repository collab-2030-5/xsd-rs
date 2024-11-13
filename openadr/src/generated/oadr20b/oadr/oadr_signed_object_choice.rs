use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum OadrSignedObjectChoice {
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

impl OadrSignedObjectChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            OadrSignedObjectChoice::OadrOadrDistributeEvent(x) => {
                x.write_with_name(writer, "oadr:oadrDistributeEvent", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreatedEvent(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedEvent", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrRequestEvent(x) => {
                x.write_with_name(writer, "oadr:oadrRequestEvent", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrResponse(x) => {
                x.write_with_name(writer, "oadr:oadrResponse", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCancelOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCancelOpt", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCanceledOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledOpt", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreateOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCreateOpt", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreatedOpt(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedOpt", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCancelReport(x) => {
                x.write_with_name(writer, "oadr:oadrCancelReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCanceledReport(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreateReport(x) => {
                x.write_with_name(writer, "oadr:oadrCreateReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreatedReport(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrRegisterReport(x) => {
                x.write_with_name(writer, "oadr:oadrRegisterReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrRegisteredReport(x) => {
                x.write_with_name(writer, "oadr:oadrRegisteredReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrUpdateReport(x) => {
                x.write_with_name(writer, "oadr:oadrUpdateReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrUpdatedReport(x) => {
                x.write_with_name(writer, "oadr:oadrUpdatedReport", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCancelPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCancelPartyRegistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCanceledPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCanceledPartyRegistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreatePartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCreatePartyRegistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrCreatedPartyRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrCreatedPartyRegistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrRequestReregistration(x) => {
                x.write_with_name(writer, "oadr:oadrRequestReregistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrQueryRegistration(x) => {
                x.write_with_name(writer, "oadr:oadrQueryRegistration", false, false)?;
            }
            OadrSignedObjectChoice::OadrOadrPoll(x) => {
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

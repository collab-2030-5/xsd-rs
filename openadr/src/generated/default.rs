use crate::oadr20b::ei::{
    CurrentValueTypeChoice, OptReasonType, PayloadBaseType, ReadingTypeType, ReportTypeType,
    SignalNameType,
};

use crate::oadr20b::oadr::OadrSignedObjectChoice;

impl Default for PayloadBaseType {
    fn default() -> Self {
        PayloadBaseType::PayloadFloat(crate::oadr20b::ei::PayloadFloatType { value: 0.0 })
    }
}

impl Default for ReportTypeType {
    fn default() -> Self {
        ReportTypeType::EiExtensionTokenType("x-Default".to_string())
    }
}

impl Default for ReadingTypeType {
    fn default() -> Self {
        ReadingTypeType::EiExtensionTokenType("x-Default".to_string())
    }
}

impl Default for CurrentValueTypeChoice {
    fn default() -> Self {
        CurrentValueTypeChoice::EiPayloadFloat(crate::oadr20b::ei::PayloadFloatType { value: 0.0 })
    }
}

impl Default for OptReasonType {
    fn default() -> Self {
        OptReasonType::EiExtensionTokenType("x-Default".to_string())
    }
}

impl Default for SignalNameType {
    fn default() -> Self {
        SignalNameType::EiExtensionTokenType("x-Default".to_string())
    }
}

impl Default for OadrSignedObjectChoice {
    fn default() -> Self {
        OadrSignedObjectChoice::OadrOadrPoll(crate::oadr20b::oadr::OadrPollType::default())
    }
}

impl Default for crate::oadr20a::ei::CurrentValueTypeChoice {
    fn default() -> Self {
        crate::oadr20a::ei::CurrentValueTypeChoice::EiPayloadFloat(
            crate::oadr20a::ei::PayloadFloat { value: 0.0 },
        )
    }
}

impl Default for crate::oadr20a::strm::StreamPayloadBaseType {
    fn default() -> Self {
        crate::oadr20a::strm::StreamPayloadBaseType::SignalPayload(
            crate::oadr20a::ei::SignalPayloadType::default(),
        )
    }
}

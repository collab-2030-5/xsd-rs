#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OadrServiceNameType {
    /// xml value == 'EiEvent'
    EiEvent,
    /// xml value == 'EiOpt'
    EiOpt,
    /// xml value == 'EiReport'
    EiReport,
    /// xml value == 'EiRegisterParty'
    EiRegisterParty,
    /// xml value == 'OadrPoll'
    OadrPoll,
}

impl xsd_util::StringEnumeration for OadrServiceNameType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "EiEvent" => Some(Self::EiEvent),
            "EiOpt" => Some(Self::EiOpt),
            "EiReport" => Some(Self::EiReport),
            "EiRegisterParty" => Some(Self::EiRegisterParty),
            "OadrPoll" => Some(Self::OadrPoll),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::EiEvent => "EiEvent",
            Self::EiOpt => "EiOpt",
            Self::EiReport => "EiReport",
            Self::EiRegisterParty => "EiRegisterParty",
            Self::OadrPoll => "OadrPoll",
        }
    }
}

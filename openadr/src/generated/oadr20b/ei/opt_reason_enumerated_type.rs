#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// Enumerated reasons for opting.
pub enum OptReasonEnumeratedType {
    /// xml value == 'economic'
    Economic,
    /// xml value == 'emergency'
    Emergency,
    /// xml value == 'mustRun'
    MustRun,
    /// xml value == 'notParticipating'
    NotParticipating,
    /// xml value == 'outageRunStatus'
    OutageRunStatus,
    /// xml value == 'overrideStatus'
    OverrideStatus,
    /// xml value == 'participating'
    Participating,
    /// xml value == 'x-schedule'
    XSchedule,
}

impl xsd_util::StringEnumeration for OptReasonEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "economic" => Some(Self::Economic),
            "emergency" => Some(Self::Emergency),
            "mustRun" => Some(Self::MustRun),
            "notParticipating" => Some(Self::NotParticipating),
            "outageRunStatus" => Some(Self::OutageRunStatus),
            "overrideStatus" => Some(Self::OverrideStatus),
            "participating" => Some(Self::Participating),
            "x-schedule" => Some(Self::XSchedule),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Economic => "economic",
            Self::Emergency => "emergency",
            Self::MustRun => "mustRun",
            Self::NotParticipating => "notParticipating",
            Self::OutageRunStatus => "outageRunStatus",
            Self::OverrideStatus => "overrideStatus",
            Self::Participating => "participating",
            Self::XSchedule => "x-schedule",
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum OptTypeType {
    #[default]
    /// xml value == 'optIn'
    OptIn,
    /// xml value == 'optOut'
    OptOut,
}

impl crate::xsd_util::StringEnumeration for OptTypeType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "optIn" => Some(Self::OptIn),
            "optOut" => Some(Self::OptOut),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::OptIn => "optIn",
            Self::OptOut => "optOut",
        }
    }
}

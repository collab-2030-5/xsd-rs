#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// Defines what type of response is requried
pub enum ResponseRequiredType {
    /// Always send a repsonse for every event received. (xml value == 'always')
    Always,
    /// Never repsond. (xml value == 'never')
    Never,
}

impl xsd_util::StringEnumeration for ResponseRequiredType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "always" => Some(Self::Always),
            "never" => Some(Self::Never),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
        }
    }
}

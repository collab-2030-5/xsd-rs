#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OadrProfileType {
    /// xml value == '2.0a'
    E20a,
    /// xml value == '2.0b'
    E20b,
}

impl crate::xsd_util::StringEnumeration for OadrProfileType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "2.0a" => Some(Self::E20a),
            "2.0b" => Some(Self::E20b),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::E20a => "2.0a",
            Self::E20b => "2.0b",
        }
    }
}

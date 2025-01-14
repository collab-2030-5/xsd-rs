#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ItemUnits {
    /// xml value == 'W'
    W,
    /// xml value == 'J/s'
    JS,
}

impl crate::xsd_util::StringEnumeration for ItemUnits {
    fn find(s: &str) -> Option<Self> {
        match s {
            "W" => Some(Self::W),
            "J/s" => Some(Self::JS),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::W => "W",
            Self::JS => "J/s",
        }
    }
}

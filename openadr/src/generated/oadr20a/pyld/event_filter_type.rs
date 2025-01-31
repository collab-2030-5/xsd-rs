#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
/// Used to restrict the Events exchanged in Event Requests.
pub enum EventFilterType {
    #[default]
    /// An event qualifies if it would qualify as either Active or Pending. (xml value == 'all')
    All,
}

impl crate::xsd_util::StringEnumeration for EventFilterType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "all" => Some(Self::All),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::All => "all",
        }
    }
}

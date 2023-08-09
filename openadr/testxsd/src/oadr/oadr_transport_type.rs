#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OadrTransportType {
    /// xml value == 'simpleHttp'
    SimpleHttp,
    /// xml value == 'xmpp'
    Xmpp,
}

impl xsd_util::StringEnumeration for OadrTransportType {
    fn find(s: &str) -> Option<Self>{
        match s {
            "simpleHttp" => Some(Self::SimpleHttp),
            "xmpp" => Some(Self::Xmpp),
            _ => None
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::SimpleHttp => "simpleHttp",
            Self::Xmpp => "xmpp",
        }
    }
}

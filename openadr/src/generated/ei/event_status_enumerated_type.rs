#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EventStatusEnumeratedType {
    /// No event pending (xml value == 'none')
    None,
    /// Event pending in the far future. The exact definition of how far in the future this refers is dependent upon the market context, but typically means the next day. (xml value == 'far')
    Far,
    /// Event pending in the near future. The exact definition of how near in the future the pending event is active is dependent on the market context. (xml value == 'near')
    Near,
    /// The event has been initiated and is currently active. (xml value == 'active')
    Active,
    /// The event has completed. (xml value == 'completed')
    Completed,
    /// The event has been canceled. (xml value == 'cancelled')
    Cancelled,
}

impl xsd_util::StringEnumeration for EventStatusEnumeratedType {
    fn find(s: &str) -> Option<Self>{
        match s {
            "none" => Some(Self::None),
            "far" => Some(Self::Far),
            "near" => Some(Self::Near),
            "active" => Some(Self::Active),
            "completed" => Some(Self::Completed),
            "cancelled" => Some(Self::Cancelled),
            _ => None
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Far => "far",
            Self::Near => "near",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CurrencyItemDescriptionType {
    /// xml value == 'currency'
    Currency,
    /// xml value == 'currencyPerKW'
    CurrencyPerKw,
    /// xml value == 'currencyPerKWh'
    CurrencyPerKWh,
}

impl xsd_util::StringEnumeration for CurrencyItemDescriptionType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "currency" => Some(Self::Currency),
            "currencyPerKW" => Some(Self::CurrencyPerKw),
            "currencyPerKWh" => Some(Self::CurrencyPerKWh),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Currency => "currency",
            Self::CurrencyPerKw => "currencyPerKW",
            Self::CurrencyPerKWh => "currencyPerKWh",
        }
    }
}

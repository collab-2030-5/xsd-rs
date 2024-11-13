#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TemperatureUnitType {
    /// xml value == 'celsius'
    Celsius,
    /// xml value == 'fahrenheit'
    Fahrenheit,
}

impl xsd_util::StringEnumeration for TemperatureUnitType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "celsius" => Some(Self::Celsius),
            "fahrenheit" => Some(Self::Fahrenheit),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Celsius => "celsius",
            Self::Fahrenheit => "fahrenheit",
        }
    }
}

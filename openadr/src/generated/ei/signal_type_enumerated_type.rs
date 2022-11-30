#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// SignalTypeEnumerated lists the pre-defined types used to specify the payload types and conformance in a stream
pub enum SignalTypeEnumeratedType {
    /// Signal indicates the amount to change from what one would have used without the signal. (xml value == 'delta')
    Delta,
    /// Signal indicates a program level. (xml value == 'level')
    Level,
    /// Signal indicates a multiplier applied to the current rate of  delivery or usage from what one would have used without the signal. (xml value == 'multiplier')
    Multiplier,
    /// Signal indicates the price. (xml value == 'price')
    Price,
    /// Signal indicates the price multiplier. Extended price is the computed price value multiplied by the number of units. (xml value == 'priceMultiplier')
    PriceMultiplier,
    /// Signal indicates the relative price. (xml value == 'priceRelative')
    PriceRelative,
    /// Signal indicates a target amount of units. (xml value == 'setpoint')
    Setpoint,
    /// This is an instruction for the load controller to operate at a level that is some percentage of its maximum load consumption capacity. This can be mapped to specific load controllers to do things like duty cycling. Note that 1.0 refers to 100% consumption. In the case of simple ON/OFF type devices then 0 = OFF and 1 = ON. (xml value == 'x-loadControlCapacity')
    XLoadControlCapacity,
    /// Discrete integer levels that are relative to normal operations where 0 is normal operations. (xml value == 'x-loadControlLevelOffset')
    XLoadControlLevelOffset,
    /// Percentage change from normal load control operations. (xml value == 'x-loadControlPercentOffset')
    XLoadControlPercentOffset,
    /// Load controller set points. (xml value == 'x-loadControlSetpoint')
    XLoadControlSetpoint,
}

impl xsd_util::StringEnumeration for SignalTypeEnumeratedType {
    fn find(s: &str) -> Option<Self>{
        match s {
            "delta" => Some(Self::Delta),
            "level" => Some(Self::Level),
            "multiplier" => Some(Self::Multiplier),
            "price" => Some(Self::Price),
            "priceMultiplier" => Some(Self::PriceMultiplier),
            "priceRelative" => Some(Self::PriceRelative),
            "setpoint" => Some(Self::Setpoint),
            "x-loadControlCapacity" => Some(Self::XLoadControlCapacity),
            "x-loadControlLevelOffset" => Some(Self::XLoadControlLevelOffset),
            "x-loadControlPercentOffset" => Some(Self::XLoadControlPercentOffset),
            "x-loadControlSetpoint" => Some(Self::XLoadControlSetpoint),
            _ => None
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Delta => "delta",
            Self::Level => "level",
            Self::Multiplier => "multiplier",
            Self::Price => "price",
            Self::PriceMultiplier => "priceMultiplier",
            Self::PriceRelative => "priceRelative",
            Self::Setpoint => "setpoint",
            Self::XLoadControlCapacity => "x-loadControlCapacity",
            Self::XLoadControlLevelOffset => "x-loadControlLevelOffset",
            Self::XLoadControlPercentOffset => "x-loadControlPercentOffset",
            Self::XLoadControlSetpoint => "x-loadControlSetpoint",
        }
    }
}

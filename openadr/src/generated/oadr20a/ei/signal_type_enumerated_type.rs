#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
/// SignalTypeEnumerated lists the pre-defined Types used to specify the Payload Types and conformance in a Stream
pub enum SignalTypeEnumeratedType {
    #[default]
    /// Signal indicates the amount to change (denominated in Itembase or in the EMIX Product) from what one would have used without the Signal. This may or may not be accompanied by a baseline. Payload Type Quantity (xml value == 'delta')
    Delta,
    /// Signal indicates a Program Level. Payload Type is Program Level (xml value == 'level')
    Level,
    /// Signal indicates a multiplier applied to the current rate of  delivery or usage (denominated in Itembase or in the EMIX Product) from what one would have used without the Signal. This may or may not be accompanied by a baseline. Payload Type is Float (xml value == 'multiplier')
    Multiplier,
    /// Signal indicates the Price. Extended Price is the value multiplied by the number of units units (denominated in Itembase or in the EMIX Product). Payload Type is emix:price (xml value == 'price')
    Price,
    /// Signal indicates the Price Multiplier. Extended Price is the computed price (as described in EMIX) the value multiplied by the number of units units (denominated in Itembase or in the EMIX Product). Payload Type is emix:priceMultiplier (xml value == 'priceMultiplier')
    PriceMultiplier,
    /// Signal indicates the Relative Price. Extended Price is the computed price (as described in EMIX) the value multiplied by the number of units units (denominated in Itembase or in the EMIX Product). Payload Type is emix:priceRelative (xml value == 'priceRelative')
    PriceRelative,
    /// Signal indicates the Product for each interval. Payload Type is an EMIX Product Description (xml value == 'product')
    Product,
    /// Signal indicates a target amount of units (denominated in Itembase or in the EMIX Product). Payload Type is Quantity (xml value == 'setpoint')
    Setpoint,
}

impl crate::xsd_util::StringEnumeration for SignalTypeEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "delta" => Some(Self::Delta),
            "level" => Some(Self::Level),
            "multiplier" => Some(Self::Multiplier),
            "price" => Some(Self::Price),
            "priceMultiplier" => Some(Self::PriceMultiplier),
            "priceRelative" => Some(Self::PriceRelative),
            "product" => Some(Self::Product),
            "setpoint" => Some(Self::Setpoint),
            _ => None,
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
            Self::Product => "product",
            Self::Setpoint => "setpoint",
        }
    }
}

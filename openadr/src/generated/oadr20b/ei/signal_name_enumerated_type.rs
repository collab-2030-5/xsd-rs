#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum SignalNameEnumeratedType {
    #[default]
    /// Simple levels (OpenADR 2.0a compliant) (xml value == 'SIMPLE')
    Simple,
    /// depreciated - for backwards compatibility with A profile (xml value == 'simple')
    Simple2,
    /// This is the cost of electricity (xml value == 'ELECTRICITY_PRICE')
    ElectricityPrice,
    /// This is the cost of energy (xml value == 'ENERGY_PRICE')
    EnergyPrice,
    /// This is the demand charge (xml value == 'DEMAND_CHARGE')
    DemandCharge,
    /// This is the price that was bid by the resource (xml value == 'BID_PRICE')
    BidPrice,
    /// This is the amount of load that was bid by a resource into a program (xml value == 'BID_LOAD')
    BidLoad,
    /// This is the amount of energy from a resource that was bid into a program (xml value == 'BID_ENERGY')
    BidEnergy,
    /// State of energy storage resource (xml value == 'CHARGE_STATE')
    ChargeState,
    /// This is used to dispatch load (xml value == 'LOAD_DISPATCH')
    LoadDispatch,
    /// Set load output to relative values (xml value == 'LOAD_CONTROL')
    LoadControl,
}

impl crate::xsd_util::StringEnumeration for SignalNameEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "SIMPLE" => Some(Self::Simple),
            "simple" => Some(Self::Simple2),
            "ELECTRICITY_PRICE" => Some(Self::ElectricityPrice),
            "ENERGY_PRICE" => Some(Self::EnergyPrice),
            "DEMAND_CHARGE" => Some(Self::DemandCharge),
            "BID_PRICE" => Some(Self::BidPrice),
            "BID_LOAD" => Some(Self::BidLoad),
            "BID_ENERGY" => Some(Self::BidEnergy),
            "CHARGE_STATE" => Some(Self::ChargeState),
            "LOAD_DISPATCH" => Some(Self::LoadDispatch),
            "LOAD_CONTROL" => Some(Self::LoadControl),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Simple => "SIMPLE",
            Self::Simple2 => "simple",
            Self::ElectricityPrice => "ELECTRICITY_PRICE",
            Self::EnergyPrice => "ENERGY_PRICE",
            Self::DemandCharge => "DEMAND_CHARGE",
            Self::BidPrice => "BID_PRICE",
            Self::BidLoad => "BID_LOAD",
            Self::BidEnergy => "BID_ENERGY",
            Self::ChargeState => "CHARGE_STATE",
            Self::LoadDispatch => "LOAD_DISPATCH",
            Self::LoadControl => "LOAD_CONTROL",
        }
    }
}

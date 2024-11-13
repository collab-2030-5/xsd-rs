#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// Enumerated report types
pub enum ReportEnumeratedType {
    /// Report indicates a reading, as from a meter. Readings are moments in time-changes over time can be computed from the difference between successive readings. Payload type is float (xml value == 'reading')
    Reading,
    /// Report indicates an amount of units (denominated in ItemBase or in the EMIX Product) over a period. Payload type is Quantity. A typical ItemBase is Real Energy. (xml value == 'usage')
    Usage,
    /// Report indicates an amount of units (denominated in ItemBase or in the EMIX Product). Payload type is Quantity. A typical ItemBase is Real Power. (xml value == 'demand')
    Demand,
    /// Report indicates the amount (denominated in ItemBase or in the EMIX Product) currently set. May be a confirmation/return of the setpoint control value sent from the VTN. Payload type is Quantity. A typical ItemBase is Real Power. (xml value == 'setPoint')
    SetPoint,
    /// Change in usage as compared to the baseline. See usage for more information (xml value == 'deltaUsage')
    DeltaUsage,
    /// Changes in setpoint from previous schedule. (xml value == 'deltaSetPoint')
    DeltaSetPoint,
    /// Change in demand as compared to the baseline. See demand for more information (xml value == 'deltaDemand')
    DeltaDemand,
    /// Can be demand or usage, as indicated by ItemBase. Indicates what [measurement] would be if not for the event or regulation. Report is of the format Baseline. (xml value == 'baseline')
    Baseline,
    /// Difference between some instruction and actual state. (xml value == 'deviation')
    Deviation,
    /// Average usage over the duration indicated by the Granularity. See usage for more information. (xml value == 'avgUsage')
    AvgUsage,
    /// Average usage over the duration indicated by the Granularity. See demand for more information. (xml value == 'avgDemand')
    AvgDemand,
    /// Generalized state of a resource such as on/off, occupancy of building, etc. No ItemBase is relevant. Requires an Application Specific Payload Extension. (xml value == 'operatingState')
    OperatingState,
    /// Up Regulation capacity available for dispatch, expressed in EMIX Real Power. Payload is always expressed as positive Quantity. (xml value == 'upRegulationCapacityAvailable')
    UpRegulationCapacityAvailable,
    /// Down Regulation capacity available for dispatch, expressed in EMIX Real Power. Payload is always expressed as positive Quantity. (xml value == 'downRegulationCapacityAvailable')
    DownRegulationCapacityAvailable,
    /// Regulation setpoint as instructed as part of regulation services (xml value == 'regulationSetpoint')
    RegulationSetpoint,
    /// Stored Energy is expressed as Real Energy and Payload is expressed as a Quantity. (xml value == 'storedEnergy')
    StoredEnergy,
    /// Target Energy is expressed as Real Energy and Payload is expressed as a Quantity. (xml value == 'targetEnergyStorage')
    TargetEnergyStorage,
    /// Capacity available for further energy storage, perhaps to get to Target Energy Storage (xml value == 'availableEnergyStorage')
    AvailableEnergyStorage,
    /// Price per ItemBase at each Interval (xml value == 'price')
    Price,
    /// Simple level from market at each Interval. Itembase is not relevant. (xml value == 'level')
    Level,
    /// Power factor for the resource. (xml value == 'powerFactor')
    PowerFactor,
    /// Percentage of usage. (xml value == 'percentUsage')
    PercentUsage,
    /// Percentage of demand (xml value == 'percentDemand')
    PercentDemand,
    /// Percentage of demand (xml value == 'x-resourceStatus')
    XResourceStatus,
}

impl xsd_util::StringEnumeration for ReportEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "reading" => Some(Self::Reading),
            "usage" => Some(Self::Usage),
            "demand" => Some(Self::Demand),
            "setPoint" => Some(Self::SetPoint),
            "deltaUsage" => Some(Self::DeltaUsage),
            "deltaSetPoint" => Some(Self::DeltaSetPoint),
            "deltaDemand" => Some(Self::DeltaDemand),
            "baseline" => Some(Self::Baseline),
            "deviation" => Some(Self::Deviation),
            "avgUsage" => Some(Self::AvgUsage),
            "avgDemand" => Some(Self::AvgDemand),
            "operatingState" => Some(Self::OperatingState),
            "upRegulationCapacityAvailable" => Some(Self::UpRegulationCapacityAvailable),
            "downRegulationCapacityAvailable" => Some(Self::DownRegulationCapacityAvailable),
            "regulationSetpoint" => Some(Self::RegulationSetpoint),
            "storedEnergy" => Some(Self::StoredEnergy),
            "targetEnergyStorage" => Some(Self::TargetEnergyStorage),
            "availableEnergyStorage" => Some(Self::AvailableEnergyStorage),
            "price" => Some(Self::Price),
            "level" => Some(Self::Level),
            "powerFactor" => Some(Self::PowerFactor),
            "percentUsage" => Some(Self::PercentUsage),
            "percentDemand" => Some(Self::PercentDemand),
            "x-resourceStatus" => Some(Self::XResourceStatus),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Reading => "reading",
            Self::Usage => "usage",
            Self::Demand => "demand",
            Self::SetPoint => "setPoint",
            Self::DeltaUsage => "deltaUsage",
            Self::DeltaSetPoint => "deltaSetPoint",
            Self::DeltaDemand => "deltaDemand",
            Self::Baseline => "baseline",
            Self::Deviation => "deviation",
            Self::AvgUsage => "avgUsage",
            Self::AvgDemand => "avgDemand",
            Self::OperatingState => "operatingState",
            Self::UpRegulationCapacityAvailable => "upRegulationCapacityAvailable",
            Self::DownRegulationCapacityAvailable => "downRegulationCapacityAvailable",
            Self::RegulationSetpoint => "regulationSetpoint",
            Self::StoredEnergy => "storedEnergy",
            Self::TargetEnergyStorage => "targetEnergyStorage",
            Self::AvailableEnergyStorage => "availableEnergyStorage",
            Self::Price => "price",
            Self::Level => "level",
            Self::PowerFactor => "powerFactor",
            Self::PercentUsage => "percentUsage",
            Self::PercentDemand => "percentDemand",
            Self::XResourceStatus => "x-resourceStatus",
        }
    }
}

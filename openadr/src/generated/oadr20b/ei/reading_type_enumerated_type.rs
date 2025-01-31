#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum ReadingTypeEnumeratedType {
    #[default]
    /// Reading is read from a device that increases monotonically, and usage must be computed from pairs of start and stop readings. (xml value == 'Direct Read')
    DirectRead,
    /// Meter or [resource] prepares its own calculation of total use over time. (xml value == 'Net')
    Net,
    /// Meter covers several [resources] and usage is inferred through some sort of pro data computation. (xml value == 'Allocated')
    Allocated,
    /// Used when a reading is absent in a series in which most readings are present. (xml value == 'Estimated')
    Estimated,
    /// Several meters together provide the reading for this [resource]. This is specifically a different than aggregated, which refers to multiple [resources] in the same payload. See also Hybrid. (xml value == 'Summed')
    Summed,
    /// Usage is inferred through knowledge of run-time, normal operation, etc. (xml value == 'Derived')
    Derived,
    /// Reading is the mean value over the period indicated in Granularity (xml value == 'Mean')
    Mean,
    /// Reading is Peak (highest) value over the period indicated in granularity. For some measurements, it may make more sense as the lowest value. May not be consistent with aggregate readings. Only valid for flow-rate Item Bases, i.e., Power not Energy. (xml value == 'Peak')
    Peak,
    /// If aggregated, refers to different reading types in the aggregate number. (xml value == 'Hybrid')
    Hybrid,
    /// Indicates reading is pro forma, i.e., is reported at agreed upon rates (xml value == 'Contract')
    Contract,
    /// Indicates reading is in the future, and has not yet been measured. (xml value == 'Projected')
    Projected,
    /// Root Mean Square (xml value == 'x-RMS')
    XRms,
    /// Not Applicable (xml value == 'x-notApplicable')
    XNotApplicable,
}

impl crate::xsd_util::StringEnumeration for ReadingTypeEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "Direct Read" => Some(Self::DirectRead),
            "Net" => Some(Self::Net),
            "Allocated" => Some(Self::Allocated),
            "Estimated" => Some(Self::Estimated),
            "Summed" => Some(Self::Summed),
            "Derived" => Some(Self::Derived),
            "Mean" => Some(Self::Mean),
            "Peak" => Some(Self::Peak),
            "Hybrid" => Some(Self::Hybrid),
            "Contract" => Some(Self::Contract),
            "Projected" => Some(Self::Projected),
            "x-RMS" => Some(Self::XRms),
            "x-notApplicable" => Some(Self::XNotApplicable),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::DirectRead => "Direct Read",
            Self::Net => "Net",
            Self::Allocated => "Allocated",
            Self::Estimated => "Estimated",
            Self::Summed => "Summed",
            Self::Derived => "Derived",
            Self::Mean => "Mean",
            Self::Peak => "Peak",
            Self::Hybrid => "Hybrid",
            Self::Contract => "Contract",
            Self::Projected => "Projected",
            Self::XRms => "x-RMS",
            Self::XNotApplicable => "x-notApplicable",
        }
    }
}

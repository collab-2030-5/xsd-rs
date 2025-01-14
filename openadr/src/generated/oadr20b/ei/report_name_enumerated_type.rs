#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ReportNameEnumeratedType {
    /// xml value == 'METADATA_HISTORY_USAGE'
    MetadataHistoryUsage,
    /// xml value == 'HISTORY_USAGE'
    HistoryUsage,
    /// xml value == 'METADATA_HISTORY_GREENBUTTON'
    MetadataHistoryGreenbutton,
    /// xml value == 'HISTORY_GREENBUTTON'
    HistoryGreenbutton,
    /// xml value == 'METADATA_TELEMETRY_USAGE'
    MetadataTelemetryUsage,
    /// xml value == 'TELEMETRY_USAGE'
    TelemetryUsage,
    /// xml value == 'METADATA_TELEMETRY_STATUS'
    MetadataTelemetryStatus,
    /// xml value == 'TELEMETRY_STATUS'
    TelemetryStatus,
}

impl crate::xsd_util::StringEnumeration for ReportNameEnumeratedType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "METADATA_HISTORY_USAGE" => Some(Self::MetadataHistoryUsage),
            "HISTORY_USAGE" => Some(Self::HistoryUsage),
            "METADATA_HISTORY_GREENBUTTON" => Some(Self::MetadataHistoryGreenbutton),
            "HISTORY_GREENBUTTON" => Some(Self::HistoryGreenbutton),
            "METADATA_TELEMETRY_USAGE" => Some(Self::MetadataTelemetryUsage),
            "TELEMETRY_USAGE" => Some(Self::TelemetryUsage),
            "METADATA_TELEMETRY_STATUS" => Some(Self::MetadataTelemetryStatus),
            "TELEMETRY_STATUS" => Some(Self::TelemetryStatus),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::MetadataHistoryUsage => "METADATA_HISTORY_USAGE",
            Self::HistoryUsage => "HISTORY_USAGE",
            Self::MetadataHistoryGreenbutton => "METADATA_HISTORY_GREENBUTTON",
            Self::HistoryGreenbutton => "HISTORY_GREENBUTTON",
            Self::MetadataTelemetryUsage => "METADATA_TELEMETRY_USAGE",
            Self::TelemetryUsage => "TELEMETRY_USAGE",
            Self::MetadataTelemetryStatus => "METADATA_TELEMETRY_STATUS",
            Self::TelemetryStatus => "TELEMETRY_STATUS",
        }
    }
}

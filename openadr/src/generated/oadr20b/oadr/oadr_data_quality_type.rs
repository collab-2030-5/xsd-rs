#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OadrDataQualityType {
    /// xml value == 'No Quality - No Value'
    NoQualityNoValue,
    /// xml value == 'No New Value - Previous Value Used'
    NoNewValuePreviousValueUsed,
    /// xml value == 'Quality Bad - Non Specific'
    QualityBadNonSpecific,
    /// xml value == 'Quality Bad - Configuration Error'
    QualityBadConfigurationError,
    /// xml value == 'Quality Bad - Not Connected'
    QualityBadNotConnected,
    /// xml value == 'Quality Bad - Device Failure'
    QualityBadDeviceFailure,
    /// xml value == 'Quality Bad - Sensor Failure'
    QualityBadSensorFailure,
    /// xml value == 'Quality Bad - Last Known Value'
    QualityBadLastKnownValue,
    /// xml value == 'Quality Bad - Comm Failure'
    QualityBadCommFailure,
    /// xml value == 'Quality Bad - Out of Service'
    QualityBadOutOfService,
    /// xml value == 'Quality Uncertain - Non Specific'
    QualityUncertainNonSpecific,
    /// xml value == 'Quality Uncertain - Last Usable Value'
    QualityUncertainLastUsableValue,
    /// xml value == 'Quality Uncertain - Sensor Not Accurate'
    QualityUncertainSensorNotAccurate,
    /// xml value == 'Quality Uncertain - EU Units Exceeded'
    QualityUncertainEuUnitsExceeded,
    /// xml value == 'Quality Uncertain - Sub Normal'
    QualityUncertainSubNormal,
    /// xml value == 'Quality Good - Non Specific'
    QualityGoodNonSpecific,
    /// xml value == 'Quality Good - Local Override'
    QualityGoodLocalOverride,
    /// xml value == 'Quality Limit - Field/Not'
    QualityLimitFieldNot,
    /// xml value == 'Quality Limit - Field/Low'
    QualityLimitFieldLow,
    /// xml value == 'Quality Limit - Field/High'
    QualityLimitFieldHigh,
    /// xml value == 'Quality Limit - Field/Constant'
    QualityLimitFieldConstant,
}

impl crate::xsd_util::StringEnumeration for OadrDataQualityType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "No Quality - No Value" => Some(Self::NoQualityNoValue),
            "No New Value - Previous Value Used" => Some(Self::NoNewValuePreviousValueUsed),
            "Quality Bad - Non Specific" => Some(Self::QualityBadNonSpecific),
            "Quality Bad - Configuration Error" => Some(Self::QualityBadConfigurationError),
            "Quality Bad - Not Connected" => Some(Self::QualityBadNotConnected),
            "Quality Bad - Device Failure" => Some(Self::QualityBadDeviceFailure),
            "Quality Bad - Sensor Failure" => Some(Self::QualityBadSensorFailure),
            "Quality Bad - Last Known Value" => Some(Self::QualityBadLastKnownValue),
            "Quality Bad - Comm Failure" => Some(Self::QualityBadCommFailure),
            "Quality Bad - Out of Service" => Some(Self::QualityBadOutOfService),
            "Quality Uncertain - Non Specific" => Some(Self::QualityUncertainNonSpecific),
            "Quality Uncertain - Last Usable Value" => Some(Self::QualityUncertainLastUsableValue),
            "Quality Uncertain - Sensor Not Accurate" => {
                Some(Self::QualityUncertainSensorNotAccurate)
            }
            "Quality Uncertain - EU Units Exceeded" => Some(Self::QualityUncertainEuUnitsExceeded),
            "Quality Uncertain - Sub Normal" => Some(Self::QualityUncertainSubNormal),
            "Quality Good - Non Specific" => Some(Self::QualityGoodNonSpecific),
            "Quality Good - Local Override" => Some(Self::QualityGoodLocalOverride),
            "Quality Limit - Field/Not" => Some(Self::QualityLimitFieldNot),
            "Quality Limit - Field/Low" => Some(Self::QualityLimitFieldLow),
            "Quality Limit - Field/High" => Some(Self::QualityLimitFieldHigh),
            "Quality Limit - Field/Constant" => Some(Self::QualityLimitFieldConstant),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::NoQualityNoValue => "No Quality - No Value",
            Self::NoNewValuePreviousValueUsed => "No New Value - Previous Value Used",
            Self::QualityBadNonSpecific => "Quality Bad - Non Specific",
            Self::QualityBadConfigurationError => "Quality Bad - Configuration Error",
            Self::QualityBadNotConnected => "Quality Bad - Not Connected",
            Self::QualityBadDeviceFailure => "Quality Bad - Device Failure",
            Self::QualityBadSensorFailure => "Quality Bad - Sensor Failure",
            Self::QualityBadLastKnownValue => "Quality Bad - Last Known Value",
            Self::QualityBadCommFailure => "Quality Bad - Comm Failure",
            Self::QualityBadOutOfService => "Quality Bad - Out of Service",
            Self::QualityUncertainNonSpecific => "Quality Uncertain - Non Specific",
            Self::QualityUncertainLastUsableValue => "Quality Uncertain - Last Usable Value",
            Self::QualityUncertainSensorNotAccurate => "Quality Uncertain - Sensor Not Accurate",
            Self::QualityUncertainEuUnitsExceeded => "Quality Uncertain - EU Units Exceeded",
            Self::QualityUncertainSubNormal => "Quality Uncertain - Sub Normal",
            Self::QualityGoodNonSpecific => "Quality Good - Non Specific",
            Self::QualityGoodLocalOverride => "Quality Good - Local Override",
            Self::QualityLimitFieldNot => "Quality Limit - Field/Not",
            Self::QualityLimitFieldLow => "Quality Limit - Field/Low",
            Self::QualityLimitFieldHigh => "Quality Limit - Field/High",
            Self::QualityLimitFieldConstant => "Quality Limit - Field/Constant",
        }
    }
}

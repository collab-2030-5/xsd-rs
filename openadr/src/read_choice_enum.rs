pub mod signal_name_type {
    use crate::oadr20b::ei::SignalNameType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<SignalNameType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::ei::SignalNameEnumeratedType::find(&value) {
            return Ok(SignalNameType::SignalNameEnumeratedType(value));
        }

        // Err(ReadError::UnknownEnumVariant)

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(SignalNameType::EiExtensionTokenType(value));
    }
}

pub mod opt_reason_type {
    use crate::oadr20b::ei::OptReasonType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<OptReasonType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::ei::OptReasonEnumeratedType::find(&value) {
            return Ok(OptReasonType::OptReasonEnumeratedType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(OptReasonType::EiExtensionTokenType(value));
    }
}

pub mod reading_type_type {
    use crate::oadr20b::ei::ReadingTypeType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<ReadingTypeType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::ei::ReadingTypeEnumeratedType::find(&value) {
            return Ok(ReadingTypeType::ReadingTypeEnumeratedType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(ReadingTypeType::EiExtensionTokenType(value));
    }
}

pub mod report_type_type {
    use crate::oadr20b::ei::ReportTypeType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<ReportTypeType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::ei::ReportEnumeratedType::find(&value) {
            return Ok(ReportTypeType::ReportEnumeratedType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(ReportTypeType::EiExtensionTokenType(value));
    }
}

pub mod oadr_data_quality_type_type {
    use crate::oadr20b::oadr::OadrDataQualityTypeType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<OadrDataQualityTypeType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::oadr::OadrDataQualityType::find(&value) {
            return Ok(OadrDataQualityTypeType::OadrDataQualityType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(OadrDataQualityTypeType::EiExtensionTokenType(value));
    }
}

pub mod report_name_type {
    use crate::oadr20b::ei::ReportNameType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn read_choice_enum<R>(
        reader: &mut xml::reader::EventReader<R>,
        parent_name: &str,
    ) -> Result<ReportNameType, ReadError>
    where
        R: std::io::Read,
    {
        let value = crate::xsd_util::read_string(reader, parent_name)?;

        if let Some(value) = crate::oadr20b::ei::ReportNameEnumeratedType::find(&value) {
            return Ok(ReportNameType::ReportNameEnumeratedType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(ReportNameType::EiExtensionTokenType(value));
    }
}

pub mod schema_version_type {
    use crate::oadr20b::ei::SchemaVersionType;
    use crate::xsd_util::{ReadError, StringEnumeration};

    pub fn convert_string_to_enum(value: &str) -> Result<SchemaVersionType, ReadError> {
        if let Some(value) = crate::oadr20b::ei::SchemaVersionEnumeratedType::find(&value) {
            return Ok(SchemaVersionType::SchemaVersionEnumeratedType(value));
        }

        // TODO: validate value against the regex: pattern="x-\S.*"
        return Ok(SchemaVersionType::EiExtensionTokenType(value.to_string()));
    }
}

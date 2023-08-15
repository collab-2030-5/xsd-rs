use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBase {
    FrequencyType(crate::oadr::FrequencyType),
    VoltageType(crate::power::VoltageType),
    BaseUnitType(crate::oadr::BaseUnitType),
    TemperatureType(crate::oadr::TemperatureType),
    PulseCountType(crate::oadr::PulseCountType),
    CurrencyType(crate::oadr::CurrencyType),
    CurrentType(crate::oadr::CurrentType),
    EnergyItemType(crate::power::EnergyItemType),
    PowerItemType(crate::power::PowerItemType),
    ThermType(crate::oadr::ThermType),
}

impl ItemBase {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            ItemBase::FrequencyType(x) => {
                x.write_with_name(writer, "FrequencyType", false, false)?;
            }
            ItemBase::VoltageType(x) => {
                x.write_with_name(writer, "VoltageType", false, false)?;
            }
            ItemBase::BaseUnitType(x) => {
                x.write_with_name(writer, "BaseUnitType", false, false)?;
            }
            ItemBase::TemperatureType(x) => {
                x.write_with_name(writer, "temperatureType", false, false)?;
            }
            ItemBase::PulseCountType(x) => {
                x.write_with_name(writer, "pulseCountType", false, false)?;
            }
            ItemBase::CurrencyType(x) => {
                x.write_with_name(writer, "currencyType", false, false)?;
            }
            ItemBase::CurrentType(x) => {
                x.write_with_name(writer, "CurrentType", false, false)?;
            }
            ItemBase::EnergyItemType(x) => {
                x.write_with_name(writer, "EnergyItemType", false, false)?;
            }
            ItemBase::PowerItemType(x) => {
                x.write_with_name(writer, "PowerItemType", false, false)?;
            }
            ItemBase::ThermType(x) => {
                x.write_with_name(writer, "ThermType", false, false)?;
            }
        }
        Ok(())
    }

    pub(crate) fn read<R>(
        _reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        unimplemented!()
    }
}

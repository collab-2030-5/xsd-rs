use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBaseType {
    BaseUnitType(crate::oadr::BaseUnitType),
    CurrentType(crate::oadr::CurrentType),
    FrequencyType(crate::oadr::FrequencyType),
    ThermType(crate::oadr::ThermType),
    VoltageType(crate::power::VoltageType),
    CurrencyType(crate::oadr::CurrencyType),
    PulseCountType(crate::oadr::PulseCountType),
    TemperatureType(crate::oadr::TemperatureType),
    PowerApparentType(crate::power::PowerApparentType),
    PowerReactiveType(crate::power::PowerReactiveType),
    PowerRealType(crate::power::PowerRealType),
    EnergyApparentType(crate::power::EnergyApparentType),
    EnergyReactiveType(crate::power::EnergyReactiveType),
    EnergyRealType(crate::power::EnergyRealType),
}

impl ItemBaseType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            ItemBaseType::BaseUnitType(x) => {
                x.write_with_name(writer, "BaseUnitType", false, false)?;
            }
            ItemBaseType::CurrentType(x) => {
                x.write_with_name(writer, "CurrentType", false, false)?;
            }
            ItemBaseType::FrequencyType(x) => {
                x.write_with_name(writer, "FrequencyType", false, false)?;
            }
            ItemBaseType::ThermType(x) => {
                x.write_with_name(writer, "ThermType", false, false)?;
            }
            ItemBaseType::VoltageType(x) => {
                x.write_with_name(writer, "VoltageType", false, false)?;
            }
            ItemBaseType::CurrencyType(x) => {
                x.write_with_name(writer, "currencyType", false, false)?;
            }
            ItemBaseType::PulseCountType(x) => {
                x.write_with_name(writer, "pulseCountType", false, false)?;
            }
            ItemBaseType::TemperatureType(x) => {
                x.write_with_name(writer, "temperatureType", false, false)?;
            }
            ItemBaseType::PowerApparentType(x) => {
                x.write_with_name(writer, "PowerApparentType", false, false)?;
            }
            ItemBaseType::PowerReactiveType(x) => {
                x.write_with_name(writer, "PowerReactiveType", false, false)?;
            }
            ItemBaseType::PowerRealType(x) => {
                x.write_with_name(writer, "PowerRealType", false, false)?;
            }
            ItemBaseType::EnergyApparentType(x) => {
                x.write_with_name(writer, "EnergyApparentType", false, false)?;
            }
            ItemBaseType::EnergyReactiveType(x) => {
                x.write_with_name(writer, "EnergyReactiveType", false, false)?;
            }
            ItemBaseType::EnergyRealType(x) => {
                x.write_with_name(writer, "EnergyRealType", false, false)?;
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

use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBaseType {
    VoltageType(crate::power::VoltageType),
    ThermType(crate::oadr::ThermType),
    EnergyItemType(crate::power::EnergyItemType),
    TemperatureType(crate::oadr::TemperatureType),
    PowerItemType(crate::power::PowerItemType),
    PulseCountType(crate::oadr::PulseCountType),
    CurrentType(crate::oadr::CurrentType),
    CurrencyType(crate::oadr::CurrencyType),
    FrequencyType(crate::oadr::FrequencyType),
    BaseUnitType(crate::oadr::BaseUnitType),
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
            ItemBaseType::VoltageType(x) => {
                x.write_with_name(writer, "VoltageType", false, false)?;
            }
            ItemBaseType::ThermType(x) => {
                x.write_with_name(writer, "ThermType", false, false)?;
            }
            ItemBaseType::EnergyItemType(x) => {
                x.write(writer)?;
            }
            ItemBaseType::TemperatureType(x) => {
                x.write_with_name(writer, "temperatureType", false, false)?;
            }
            ItemBaseType::PowerItemType(x) => {
                x.write_with_name(writer, "PowerItemType", false, false)?;
            }
            ItemBaseType::PulseCountType(x) => {
                x.write_with_name(writer, "pulseCountType", false, false)?;
            }
            ItemBaseType::CurrentType(x) => {
                x.write_with_name(writer, "CurrentType", false, false)?;
            }
            ItemBaseType::CurrencyType(x) => {
                x.write_with_name(writer, "currencyType", false, false)?;
            }
            ItemBaseType::FrequencyType(x) => {
                x.write_with_name(writer, "FrequencyType", false, false)?;
            }
            ItemBaseType::BaseUnitType(x) => {
                x.write_with_name(writer, "BaseUnitType", false, false)?;
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

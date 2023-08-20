use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBaseType {
    Therm(crate::oadr::ThermType),
    Currency(crate::oadr::CurrencyType),
    Current(crate::oadr::CurrentType),
    CustomUnit(crate::oadr::BaseUnitType),
    Frequency(crate::oadr::FrequencyType),
    PulseCount(crate::oadr::PulseCountType),
    Temperature(crate::oadr::TemperatureType),
    Voltage(crate::power::VoltageType),
    PowerApparent(crate::power::PowerApparentType),
    PowerReactive(crate::power::PowerReactiveType),
    PowerReal(crate::power::PowerRealType),
    EnergyApparent(crate::power::EnergyApparentType),
    EnergyReactive(crate::power::EnergyReactiveType),
    EnergyReal(crate::power::EnergyRealType),
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
            ItemBaseType::Therm(x) => {
                x.write_with_name(writer, "Therm", false, false)?;
            }
            ItemBaseType::Currency(x) => {
                x.write_with_name(writer, "currency", false, false)?;
            }
            ItemBaseType::Current(x) => {
                x.write_with_name(writer, "current", false, false)?;
            }
            ItemBaseType::CustomUnit(x) => {
                x.write_with_name(writer, "customUnit", false, false)?;
            }
            ItemBaseType::Frequency(x) => {
                x.write_with_name(writer, "frequency", false, false)?;
            }
            ItemBaseType::PulseCount(x) => {
                x.write_with_name(writer, "pulseCount", false, false)?;
            }
            ItemBaseType::Temperature(x) => {
                x.write_with_name(writer, "temperature", false, false)?;
            }
            ItemBaseType::Voltage(x) => {
                x.write_with_name(writer, "voltage", false, false)?;
            }
            ItemBaseType::PowerApparent(x) => {
                x.write_with_name(writer, "powerApparent", false, false)?;
            }
            ItemBaseType::PowerReactive(x) => {
                x.write_with_name(writer, "powerReactive", false, false)?;
            }
            ItemBaseType::PowerReal(x) => {
                x.write_with_name(writer, "powerReal", false, false)?;
            }
            ItemBaseType::EnergyApparent(x) => {
                x.write_with_name(writer, "energyApparent", false, false)?;
            }
            ItemBaseType::EnergyReactive(x) => {
                x.write_with_name(writer, "energyReactive", false, false)?;
            }
            ItemBaseType::EnergyReal(x) => {
                x.write_with_name(writer, "energyReal", false, false)?;
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

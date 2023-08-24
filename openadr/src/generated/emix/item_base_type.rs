use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBaseType {
    Therm(crate::oadr::ThermType),
    Currency(crate::oadr::CurrencyType),
    CurrencyPerKw(crate::oadr::CurrencyType),
    CurrencyPerKWh(crate::oadr::CurrencyType),
    CurrencyPerThm(crate::oadr::CurrencyType),
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
                x.write_with_name(writer, "oadr:Therm", false, false)?;
            }
            ItemBaseType::Currency(x) => {
                x.write_with_name(writer, "oadr:currency", false, false)?;
            }
            ItemBaseType::CurrencyPerKw(x) => {
                x.write_with_name(writer, "oadr:currencyPerKW", false, false)?;
            }
            ItemBaseType::CurrencyPerKWh(x) => {
                x.write_with_name(writer, "oadr:currencyPerKWh", false, false)?;
            }
            ItemBaseType::CurrencyPerThm(x) => {
                x.write_with_name(writer, "oadr:currencyPerThm", false, false)?;
            }
            ItemBaseType::Current(x) => {
                x.write_with_name(writer, "oadr:current", false, false)?;
            }
            ItemBaseType::CustomUnit(x) => {
                x.write_with_name(writer, "oadr:customUnit", false, false)?;
            }
            ItemBaseType::Frequency(x) => {
                x.write_with_name(writer, "oadr:frequency", false, false)?;
            }
            ItemBaseType::PulseCount(x) => {
                x.write_with_name(writer, "oadr:pulseCount", false, false)?;
            }
            ItemBaseType::Temperature(x) => {
                x.write_with_name(writer, "oadr:temperature", false, false)?;
            }
            ItemBaseType::Voltage(x) => {
                x.write_with_name(writer, "power:voltage", false, false)?;
            }
            ItemBaseType::PowerApparent(x) => {
                x.write_with_name(writer, "power:powerApparent", false, false)?;
            }
            ItemBaseType::PowerReactive(x) => {
                x.write_with_name(writer, "power:powerReactive", false, false)?;
            }
            ItemBaseType::PowerReal(x) => {
                x.write_with_name(writer, "power:powerReal", false, false)?;
            }
            ItemBaseType::EnergyApparent(x) => {
                x.write_with_name(writer, "power:energyApparent", false, false)?;
            }
            ItemBaseType::EnergyReactive(x) => {
                x.write_with_name(writer, "power:energyReactive", false, false)?;
            }
            ItemBaseType::EnergyReal(x) => {
                x.write_with_name(writer, "power:energyReal", false, false)?;
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

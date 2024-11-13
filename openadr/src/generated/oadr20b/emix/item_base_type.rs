use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBaseType {
    Therm(crate::oadr20b::oadr::ThermType),
    Currency(crate::oadr20b::oadr::CurrencyType),
    CurrencyPerKw(crate::oadr20b::oadr::CurrencyType),
    CurrencyPerKWh(crate::oadr20b::oadr::CurrencyType),
    CurrencyPerThm(crate::oadr20b::oadr::CurrencyType),
    Current(crate::oadr20b::oadr::CurrentType),
    CustomUnit(crate::oadr20b::oadr::BaseUnitType),
    Frequency(crate::oadr20b::oadr::FrequencyType),
    PulseCount(crate::oadr20b::oadr::PulseCountType),
    Temperature(crate::oadr20b::oadr::TemperatureType),
    Voltage(crate::oadr20b::power::VoltageType),
    EnergyApparent(crate::oadr20b::power::EnergyApparentType),
    EnergyReactive(crate::oadr20b::power::EnergyReactiveType),
    EnergyReal(crate::oadr20b::power::EnergyRealType),
    PowerApparent(crate::oadr20b::power::PowerApparentType),
    PowerReactive(crate::oadr20b::power::PowerReactiveType),
    PowerReal(crate::oadr20b::power::PowerRealType),
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
            ItemBaseType::EnergyApparent(x) => {
                x.write_with_name(writer, "power:energyApparent", false, false)?;
            }
            ItemBaseType::EnergyReactive(x) => {
                x.write_with_name(writer, "power:energyReactive", false, false)?;
            }
            ItemBaseType::EnergyReal(x) => {
                x.write_with_name(writer, "power:energyReal", false, false)?;
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

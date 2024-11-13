use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EnergyItemType {
    EnergyApparent(crate::oadr20b::power::EnergyApparentType),
    EnergyReactive(crate::oadr20b::power::EnergyReactiveType),
    EnergyReal(crate::oadr20b::power::EnergyRealType),
}

impl EnergyItemType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            EnergyItemType::EnergyApparent(x) => {
                x.write_with_name(writer, "power:energyApparent", false, false)?;
            }
            EnergyItemType::EnergyReactive(x) => {
                x.write_with_name(writer, "power:energyReactive", false, false)?;
            }
            EnergyItemType::EnergyReal(x) => {
                x.write_with_name(writer, "power:energyReal", false, false)?;
            }
        }
        Ok(())
    }
}

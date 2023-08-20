use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EnergyItemType {
    EnergyApparent(crate::power::EnergyApparentType),
    EnergyReactive(crate::power::EnergyReactiveType),
    EnergyReal(crate::power::EnergyRealType),
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
                x.write_with_name(writer, "energyApparent", false, false)?;
            }
            EnergyItemType::EnergyReactive(x) => {
                x.write_with_name(writer, "energyReactive", false, false)?;
            }
            EnergyItemType::EnergyReal(x) => {
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

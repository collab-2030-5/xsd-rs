use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EnergyItemType {
    EnergyRealType(crate::power::EnergyRealType),
    EnergyApparentType(crate::power::EnergyApparentType),
    EnergyReactiveType(crate::power::EnergyReactiveType),
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
            EnergyItemType::EnergyRealType(x) => {
                x.write_with_name(writer, "EnergyRealType", false, false)?;
            }
            EnergyItemType::EnergyApparentType(x) => {
                x.write_with_name(writer, "EnergyApparentType", false, false)?;
            }
            EnergyItemType::EnergyReactiveType(x) => {
                x.write_with_name(writer, "EnergyReactiveType", false, false)?;
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

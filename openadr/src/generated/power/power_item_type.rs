use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PowerItemType {
    PowerReactiveType(crate::power::PowerReactiveType),
    PowerApparentType(crate::power::PowerApparentType),
    PowerRealType(crate::power::PowerRealType),
}

impl PowerItemType {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            PowerItemType::PowerReactiveType(x) => {
                x.write_with_name(writer, "PowerReactiveType", false, false)?;
            }
            PowerItemType::PowerApparentType(x) => {
                x.write_with_name(writer, "PowerApparentType", false, false)?;
            }
            PowerItemType::PowerRealType(x) => {
                x.write_with_name(writer, "PowerRealType", false, false)?;
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

use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PowerItemType {
    PowerApparent(crate::power::PowerApparentType),
    PowerReactive(crate::power::PowerReactiveType),
    PowerReal(crate::power::PowerRealType),
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
            PowerItemType::PowerApparent(x) => {
                x.write_with_name(writer, "power:powerApparent", false, false)?;
            }
            PowerItemType::PowerReactive(x) => {
                x.write_with_name(writer, "power:powerReactive", false, false)?;
            }
            PowerItemType::PowerReal(x) => {
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

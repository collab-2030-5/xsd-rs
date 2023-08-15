use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PowerItem {
    PowerRealType(crate::power::PowerRealType),
    PowerReactiveType(crate::power::PowerReactiveType),
    PowerApparentType(crate::power::PowerApparentType),
}

impl PowerItem {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            PowerItem::PowerRealType(x) => {
                x.write_with_name(writer, "PowerRealType", false, false)?;
            }
            PowerItem::PowerReactiveType(x) => {
                x.write_with_name(writer, "PowerReactiveType", false, false)?;
            }
            PowerItem::PowerApparentType(x) => {
                x.write_with_name(writer, "PowerApparentType", false, false)?;
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

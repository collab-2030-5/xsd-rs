use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TransformTypeChoice {
    XsString(String),
}

impl TransformTypeChoice {
    pub(crate) fn write<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        match self {
            TransformTypeChoice::XsString(x) => {
                xsd_util::write_simple_element(writer, "xs:string", x.as_str())?;
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

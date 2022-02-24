/// serializable XML types implement this trait
pub trait WriteToXml {
    fn write_to_xml<W>(&self, w: &mut W) -> core::result::Result<(), WriteError> where W: std::io::Write;
}
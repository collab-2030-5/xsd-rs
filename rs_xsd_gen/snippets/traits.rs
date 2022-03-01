/// XML types implement this trait to write them to any std::io::Write
pub trait WriteToXml {
    fn write_to_xml<W>(&self, w: &mut W) -> core::result::Result<(), WriteError> where W: std::io::Write;
}

/// XML types implement this trait to be read from any std::io::Read
pub trait ReadFromXml : Sized {
    fn read_from_xml<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation> where R: std::io::Read;
}

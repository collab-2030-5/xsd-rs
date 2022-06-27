use super::*;

/// XML types implement this trait to write them to any std::io::Write
pub trait WriteXml {

    /// write xml to any std::io::Write
    fn write<W>(&self, config: WriteConfig, w: &mut W) -> core::result::Result<(), WriteError>
        where
            W: std::io::Write;

    /// write to utf-8 encoded Vec<u8>
    fn write_bytes(&self, config: WriteConfig) -> core::result::Result<Vec<u8>, WriteError> {
        let mut out = Vec::new();
        self.write(config, &mut out)?;
        Ok(out)
    }

    /// write to a String
    fn write_string(&self, config: WriteConfig) -> core::result::Result<String, WriteError> {
        let bytes = self.write_bytes(config)?;
        let ret = String::from_utf8(bytes)?;
        Ok(ret)
    }
}

/// XML types implement this trait to be read from any std::io::Read
pub trait ReadXml: Sized {

    /// parse XML from any std::io::Read
    fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation>
        where
            R: std::io::Read;

    /// parse XML from &str
    fn read_str<S: AsRef<str>>(data: S) -> core::result::Result<Self, ErrorWithLocation> {
        let mut cursor = std::io::Cursor::new(data.as_ref());
        Self::read(&mut cursor)
    }

    /// parse XML from &str
    fn read_file<S: AsRef<std::path::Path>>(path: S) -> core::result::Result<Self, ErrorWithLocation> {
        let mut reader = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);
        Self::read(&mut reader)
    }
}

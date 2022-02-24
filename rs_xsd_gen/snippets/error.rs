/// errors that can occur when writing XML
#[derive(Debug)]
pub enum WriteError {
    /// I/O errors
    Io(::std::io::Error),
    /// other backend dependent errors
    Other(Box<dyn std::error::Error>),
}

impl From<xml::writer::Error> for WriteError {
    fn from(err: xml::writer::Error) -> Self {
        match err {
            xml::writer::Error::Io(x) => Self::Io(x),
            _ => WriteError::Other(err.into()),
        }
    }
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WriteError::Io(x) => write!(f, "{}", x),
            WriteError::Other(x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for WriteError {}
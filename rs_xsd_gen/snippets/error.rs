/// errors that can occur when writing XML
#[derive(Debug)]
pub enum WriteError {
    /// I/O errors
    Io(::std::io::Error),
    /// other backend dependent errors
    Other(Box<dyn std::error::Error>),
}

/// errors that can occur when reading XML
#[derive(Debug)]
pub enum ReadError {
    /// I/O errors
    Io(::std::io::Error),
    /// Duplicate field
    DuplicateField,
    /// Duplicate field
    MissingMandatoryField,
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

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadError::Io(x) => write!(f, "{}", x),
            ReadError::DuplicateField => write!(f, "duplicate field"),
            ReadError::Other(x) => write!(f, "{}", x),
            ReadError::MissingMandatoryField => write!(f, "missing mandatory field"),
        }
    }
}

impl std::error::Error for WriteError {}
impl std::error::Error for ReadError {}
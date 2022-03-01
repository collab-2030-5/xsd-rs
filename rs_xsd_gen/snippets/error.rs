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
    Io,
    /// Duplicate field
    DuplicateField,
    /// Missing mandatory field
    MissingMandatoryField,
    /// Unknown attribute
    UnknownAttribute,
    /// String -> integer parsing failed
    BadInteger,
    /// hex string bad format
    BadHexString,
    /// Parser encountered an unexpected event
    UnexpectedEvent,
    /// other backend dependent errors
    Other(Box<dyn std::error::Error>),
}

/// provides ReadError with line/col info
#[derive(Debug)]
pub struct ErrorWithLocation {
    pub err: ReadError,
    pub line: u64,
    pub col: u64,
}

impl std::fmt::Display for ErrorWithLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{} - {}", self.line, self.col, self.err)
    }
}

impl std::error::Error for ErrorWithLocation {}

impl From<xml::writer::Error> for WriteError {
    fn from(err: xml::writer::Error) -> Self {
        match err {
            xml::writer::Error::Io(x) => Self::Io(x),
            _ => WriteError::Other(err.into()),
        }
    }
}

impl From<xml::reader::Error> for ReadError {
    fn from(err: xml::reader::Error) -> Self {
        match err.kind() {
            xml::reader::ErrorKind::Io(_) => Self::Io,
            _ => ReadError::Other(err.into()),
        }
    }
}

impl From<std::num::ParseIntError> for ReadError {
    fn from(_: std::num::ParseIntError) -> Self {
        ReadError::BadInteger
    }
}

impl From<std::str::ParseBoolError> for ReadError {
    fn from(_: std::str::ParseBoolError) -> Self {
        ReadError::BadInteger
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
            ReadError::Io => write!(f, "io error"),
            ReadError::DuplicateField => write!(f, "duplicate field"),
            ReadError::Other(x) => write!(f, "{}", x),
            ReadError::MissingMandatoryField => write!(f, "missing mandatory field"),
            ReadError::UnknownAttribute => write!(f, "unknown attribute"),
            ReadError::BadInteger => write!(f, "bad integer"),
            ReadError::BadHexString => write!(f, "bad hex string"),
            ReadError::UnexpectedEvent => write!(f, "unexpected event"),
        }
    }
}

impl std::error::Error for WriteError {}
impl std::error::Error for ReadError {}
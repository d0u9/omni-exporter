use std::fmt;

#[derive(Debug)]
pub enum IOError {
    NotFound(String),
    NotDir(String),
    Other(String),
}

#[derive(Debug)]
pub enum Err {
    ParseStringErr(String),
    IOErr(IOError),
    InvalidIndex(String),
    NotImplemented,
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Err::ParseStringErr(ref err) => write!(f, "Parsing String Error: {}", err),
            Err::IOErr(ref err) => write!(f, "IO Error: {:?}", err),
            Err::InvalidIndex(ref err) => write!(f, "Invalid Index: {}", err),
            Err::NotImplemented => write!(f, "Not Implemented"),
        }
    }
}

impl std::error::Error for Err {}

pub type Result<T> = std::result::Result<T, Err>;

impl From<std::num::ParseIntError> for Err {
    fn from(err: std::num::ParseIntError) -> Self {
        Err::ParseStringErr(err.to_string())
    }
}

impl From<std::io::Error> for Err {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Err::IOErr(IOError::NotFound(err.to_string())),
            _ => Err::IOErr(IOError::Other(err.to_string())),
        }
    }
}

impl From<std::num::ParseFloatError> for Err {
    fn from(err: std::num::ParseFloatError) -> Self {
        Err::ParseStringErr(err.to_string())
    }
}

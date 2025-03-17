use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseStringErr(String),
    IOErr(String),
    InvalidIndex(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseStringErr(ref err) => write!(f, "Parsing String Error: {}", err),
            Error::IOErr(ref err) => write!(f, "IO Error: {}", err),
            Error::InvalidIndex(ref err) => write!(f, "Invalid Index: {}", err),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseStringErr(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOErr(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseStringErr(err.to_string())
    }
}

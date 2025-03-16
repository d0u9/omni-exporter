use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseStringErr(String),
    IOError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseStringErr(ref err) => write!(f, "Parsing String Error: {}", err),
            Error::IOError(ref err) => write!(f, "IO Error: {}", err),
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
        Error::IOError(err.to_string())
    }
}

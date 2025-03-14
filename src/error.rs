use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    NotFound,
    NotImplemented,
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidInput => write!(f, "Invalid input provided"),
            Error::NotFound => write!(f, "Resource not found"),
            Error::NotImplemented => write!(f, "Not implemented"),
            Error::CustomError(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

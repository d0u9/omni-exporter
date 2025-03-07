use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    NotFound,
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidInput => write!(f, "Invalid input provided"),
            Error::NotFound => write!(f, "Resource not found"),
            Error::CustomError(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

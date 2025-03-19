use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    NotFound,
    NotImplemented,
    FFIError(String),
    CustomError(String),
    ProcfsError(String),
    LabelKeyAlreadyExists(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidInput => write!(f, "Invalid input provided"),
            Error::NotFound => write!(f, "Resource not found"),
            Error::NotImplemented => write!(f, "Not implemented"),
            Error::CustomError(ref err) => write!(f, "Error: {}", err),
            Error::FFIError(ref err) => write!(f, "FFI error: {}", err),
            Error::ProcfsError(ref err) => write!(f, "Procfs error: {}", err),
            Error::LabelKeyAlreadyExists(ref key) => write!(f, "Label key already exists: {}", key),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(target_os = "linux")]
impl From<crate::procfs::Err> for Error {
    fn from(err: crate::procfs::Err) -> Self {
        Error::ProcfsError(err.to_string())
    }
}

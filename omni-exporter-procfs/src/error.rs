use std::fmt;

#[derive(Debug)]
pub enum IOErr {
    NotFound(String),
    NotDir(String),
    Other(String),
}

#[derive(Debug)]
pub enum Err {
    Sysfs(String),
    Procfs(String),
    ParseString(String),
    IO(IOErr),
    InvalidIndex(String),
    NotImplemented,
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Err::ParseString(ref err) => write!(f, "Parsing String Error: {}", err),
            Err::IO(ref err) => write!(f, "IO Error: {:?}", err),
            Err::Sysfs(ref err) => write!(f, "Sysfs Error: {}", err),
            Err::Procfs(ref err) => write!(f, "Procfs Error: {}", err),
            Err::InvalidIndex(ref err) => write!(f, "Invalid Index: {}", err),
            Err::NotImplemented => write!(f, "Not Implemented"),
        }
    }
}

impl std::error::Error for Err {}

pub type Result<T> = std::result::Result<T, Err>;

impl From<std::io::Error> for Err {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Err::IO(IOErr::NotFound(err.to_string())),
            _ => Err::IO(IOErr::Other(err.to_string())),
        }
    }
}

impl From<std::num::ParseIntError> for Err {
    fn from(err: std::num::ParseIntError) -> Self {
        Err::ParseString(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Err {
    fn from(err: std::num::ParseFloatError) -> Self {
        Err::ParseString(err.to_string())
    }
}

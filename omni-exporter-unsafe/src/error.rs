use std::fmt;

#[derive(Debug)]
pub enum Err {
    FFI(String),
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Err::FFI(ref err) => write!(f, "C Error: {}", err),
        }
    }
}

impl std::error::Error for Err {}

pub type Result<T> = std::result::Result<T, Err>;

impl From<std::num::ParseIntError> for Err {
    fn from(err: std::num::ParseIntError) -> Self {
        Err::FFI(err.to_string())
    }
}

impl From<std::io::Error> for Err {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Err::FFI(err.to_string()),
            _ => Err::FFI(err.to_string()),
        }
    }
}

impl From<std::num::ParseFloatError> for Err {
    fn from(err: std::num::ParseFloatError) -> Self {
        Err::FFI(err.to_string())
    }
}

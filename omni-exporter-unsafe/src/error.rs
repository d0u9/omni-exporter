use std::fmt;

#[derive(Debug)]
pub enum Err {
    FFI(String),
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Err::FFI(ref err) => write!(f, "FFI Error: {}", err),
        }
    }
}

impl std::error::Error for Err {}

pub type Result<T> = std::result::Result<T, Err>;

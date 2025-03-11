use std::convert::Into;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    None,
    F64(f64),
    U64(u64),
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::F64(v) => v,
            Value::U64(v) => v as f64,
            _ => f64::NAN,
        }
    }
}

pub type Timestamp = Option<u64>;

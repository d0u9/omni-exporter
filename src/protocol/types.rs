use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    None,
    F64(f64),
    U64(u64),
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        match value {
            Value::F64(v) => v,
            Value::U64(v) => v as f64,
            _ => f64::NAN,
        }
    }
}

pub type Timestamp = Option<u64>;

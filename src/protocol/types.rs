use std::convert::Into;

#[derive(Debug, Clone, Copy)]
pub enum ProtocolValue {
    None,
    F64(f64),
    U64(u64),
}

impl Into<f64> for ProtocolValue {
    fn into(self) -> f64 {
        match self {
            ProtocolValue::F64(v) => v,
            ProtocolValue::U64(v) => v as f64,
            _ => f64::NAN,
        }
    }
}



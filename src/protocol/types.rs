use std::convert::Into;

use crate::error::Result;

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

pub type Key = String;
pub type Val = String;

#[derive(Debug, Clone)]
pub struct Label {
    key: Key,
    val: Val,
}

pub type Timestamp = Option<u64>;

pub type MetricName = String;

#[derive(Debug, Clone)]
pub struct ProtoItem3 {
    metric_name: MetricName,
    labels: Vec<Label>,
    value: Value,
    timestamp: Timestamp,
}

pub trait ProtoItem {
    fn metric_name(&self) -> &str;
    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;
    fn value(&self) -> impl Into<Value>;
    fn timestamp(&self) -> Timestamp;
}

pub trait ProtoWriter<I>
where
    I: ProtoItem,
{
    fn add_item(&mut self, item: I) -> Result<()>;
}

pub trait ProtoReader {
    type Item: ProtoItem;

    fn next(&mut self) -> Option<Self::Item>;
}

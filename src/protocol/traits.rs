use crate::error::Result;

use super::Value;
use super::Timestamp;

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information
pub trait ProtocolGetter {
    fn get_metric_name(&self) -> &str;
    fn get_labels(&self) -> impl Iterator<Item = (&str, &str)>;
    fn get_value(&self) -> Result<Value>;
}

pub trait ProtocolSetter {
    fn set_metric_name(&mut self, name: &str);
    fn set_labels(&mut self, key: &str, value: &str);
    fn set_value(&mut self, value: Value);
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

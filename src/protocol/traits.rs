use crate::error::Result;

use super::Timestamp;
use super::Value;

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information
pub trait Item {
    fn metric_name(&self) -> &str;
    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;
    fn value(&self) -> impl Into<Value>;
    fn timestamp(&self) -> Timestamp;
}

pub trait Writer<I>
where
    I: Item,
{
    fn add_item(&mut self, item: I) -> Result<()>;
}

pub trait Reader: Iterator {}

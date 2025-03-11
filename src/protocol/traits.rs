use crate::error::Result;

use super::Timestamp;
use super::Value;

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information
pub trait ItemGetter {
    fn metric_name(&self) -> &str;
    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;
    fn value(&self) -> impl Into<Value>;
    fn timestamp(&self) -> Timestamp;
}

pub trait ItemSetter {
    fn set_metric_name(&mut self, name: &str);
    fn set_labels<'a>(&mut self, labels: impl Iterator<Item = (&'a str, &'a str)>);
    fn set_value(&mut self, value: impl Into<Value>);
    fn set_timestamp(&mut self, timestamp: Timestamp);
}

pub trait Writer<I>
where
    I: ItemGetter,
{
    fn add_item(&mut self, item: I) -> Result<()>;

    fn add_items(&mut self, items: impl Iterator<Item = I>) -> Result<()> {
        for item in items {
            self.add_item(item)?;
        }
        Ok(())
    }
}

pub trait Reader: Iterator {}

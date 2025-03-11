use std::collections::HashMap;

use crate::error::Result;

use super::Item;
use super::Reader;
use super::Timestamp;
use super::Value as ProtoValue;
use super::Writer;

pub struct Plain {
    pub items: Vec<PlainItem>,
}

impl Plain {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<I: Item> Writer<I> for Plain {
    fn add_item(&mut self, item: I) -> Result<()> {
        let item = PlainItem {
            metric_name: item.metric_name().to_owned(),
            labels: item
                .labels()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            value: item.value().into(),
            timestamp: item.timestamp(),
        };

        self.items.push(item);

        Ok(())
    }
}

impl Iterator for Plain {
    type Item = PlainItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

impl Reader for Plain {}

pub struct PlainItem {
    pub metric_name: String,
    pub labels: HashMap<String, String>,
    pub value: ProtoValue,
    pub timestamp: Timestamp,
}

impl PlainItem {
    pub fn new() -> Self {
        Self {
            metric_name: String::new(),
            labels: HashMap::new(),
            value: ProtoValue::None,
            timestamp: None,
        }
    }
}

impl Item for PlainItem {
    fn metric_name(&self) -> &str {
        self.metric_name.as_ref()
    }

    fn labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.labels.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn value(&self) -> impl Into<ProtoValue> {
        self.value
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

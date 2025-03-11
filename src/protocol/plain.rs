use std::collections::HashMap;

use crate::error::Result;

use super::ItemGetter;
use super::ItemSetter;
use super::Reader;
use super::Timestamp;
use super::Value as ProtoValue;
use super::Writer;

pub struct Plain {
    items: Vec<PlainItem>,
}

impl Default for Plain {
    fn default() -> Self {
        Self::new()
    }
}

impl Plain {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<I: ItemGetter> Writer<I> for Plain {
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
    metric_name: String,
    labels: HashMap<String, String>,
    value: ProtoValue,
    timestamp: Timestamp,
}

impl Default for PlainItem {
    fn default() -> Self {
        Self::new()
    }
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

impl ItemGetter for PlainItem {
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

impl ItemSetter for PlainItem {
    fn set_metric_name(&mut self, name: &str) {
        self.metric_name = name.to_owned();
    }

    fn set_labels<'a>(&mut self, labels: impl Iterator<Item = (&'a str, &'a str)>) {
        self.labels.clear();
        for (k, v) in labels {
            self.labels.insert(k.to_owned(), v.to_owned());
        }
    }

    fn set_value(&mut self, value: impl Into<ProtoValue>) {
        self.value = value.into();
    }

    fn set_timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp = timestamp;
    }
}

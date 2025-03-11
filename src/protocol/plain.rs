use std::collections::HashMap;

use crate::error::Result;

use super::ProtocolGetter;
use super::ProtocolSetter;
use super::Value as ProtoValue;
use super::types::ProtoItem;
use super::types::ProtoReader;
use super::types::ProtoWriter;
use super::types::Timestamp;

pub struct Plain {
    items: Vec<PlainItem>,
}

impl<I: ProtoItem> ProtoWriter<I> for Plain {
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

impl ProtoReader for Plain {
    type Item = PlainItem;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct PlainItem {
    metric_name: String,
    labels: HashMap<String, String>,
    value: ProtoValue,
    timestamp: Timestamp,
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

impl ProtoItem for PlainItem {
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

pub struct PlainItemOld {
    metric_name: String,
    labels: HashMap<String, String>,
    value: ProtoValue,
}

impl PlainItemOld {
    pub fn new() -> Self {
        Self {
            metric_name: String::new(),
            labels: HashMap::new(),
            value: ProtoValue::None,
        }
    }
}

impl ProtocolGetter for PlainItemOld {
    fn get_metric_name(&self) -> &str {
        &self.metric_name
    }

    fn get_labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.labels.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn get_value(&self) -> Result<ProtoValue> {
        Ok(self.value)
    }
}

impl ProtocolSetter for PlainItemOld {
    fn set_metric_name(&mut self, name: &str) {
        self.metric_name = name.to_string();
    }

    fn set_labels(&mut self, key: &str, value: &str) {
        self.labels.insert(key.to_string(), value.to_string());
    }

    fn set_value(&mut self, value: ProtoValue) {
        self.value = value;
    }
}

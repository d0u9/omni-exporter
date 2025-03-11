use std::collections::HashMap;

use crate::error::Result;

use super::ProtoValue;
use super::ProtocolGetter;
use super::ProtocolSetter;

pub struct Plain {
    metric_name: String,
    labels: HashMap<String, String>,
    value: ProtoValue,
}

impl Plain {
    pub fn new() -> Self {
        Self {
            metric_name: String::new(),
            labels: HashMap::new(),
            value: ProtoValue::None,
        }
    }
}

impl ProtocolGetter for Plain {
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

impl ProtocolSetter for Plain {
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

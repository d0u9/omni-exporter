use std::collections::HashMap;
use std::convert::Into;

use super::ProtocolGetter;
use super::ProtocolSetter;

pub struct Plain {
    metric_name: String,
    labels: HashMap<String, String>,
    value: String,
}

impl Plain {
    pub fn new() -> Self {
        Self {
            metric_name: String::new(),
            labels: HashMap::new(),
            value: String::new(),
        }
    }
}

impl ProtocolSetter for Plain {
    type Cell = String;

    fn metric_name(&mut self, name: &str) {
        self.metric_name = name.to_string();
    }

    fn label<T: Into<Self::Cell>>(&mut self, key: &str, value: T) {
        self.labels.insert(key.to_string(), value.into());
    }

    fn value<T: Into<Self::Cell>>(&mut self, value: T) {
        self.value = value.into();
    }
}

impl ProtocolGetter for Plain {
    type Cell = String;

    fn metric_name(&self) -> &str {
        &self.metric_name
    }

    fn label<T>(&self, key: &str) -> Option<T>
    where
        T: From<Cell>,
    {
        let v = self.labels.get(key);
        match v {
            Some(v) => Some(T::from(v.to_owned())),
            None => None,
        }
    }

    fn value<T>(&self) -> Option<T>
    where
        T: From<Cell>,
    {
        Some(T::from(self.value.to_owned()))
    }
}

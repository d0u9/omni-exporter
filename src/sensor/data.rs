use std::collections::HashMap;

use crate::error::Result;
use crate::protocol::ItemGetter as ProtoItem;
use crate::protocol::Value as ProtoValue;

use super::traits::SensorData;

pub type SensorDataChunk<T> = Vec<T>;

#[derive(Debug)]
pub struct OwnedSensorData {
    metric_name: String,
    value: ProtoValue,
    labels: HashMap<String, String>,
}

impl SensorData for OwnedSensorData {
    fn metric_name(&self) -> &str {
        &self.metric_name
    }

    fn value(&self) -> ProtoValue {
        self.value
    }

    fn labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.labels.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn from_proto<P>(proto: P) -> Result<Self>
    where
        P: ProtoItem + Send + Sync + 'static,
    {
        let s = Self {
            metric_name: proto.metric_name().to_owned(),
            value: proto.value().into(),
            labels: proto
                .labels()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
        };

        Ok(s)
    }
}

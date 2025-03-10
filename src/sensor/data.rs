use std::collections::HashMap;

use crate::error::Result;
use crate::protocol::ProtocolGetter;
use crate::protocol::ProtocolValue;

use super::traits::SensorData;

pub type SensorDataChunk<T> = Vec<T>;

#[derive(Debug)]
pub struct OwnedSensorData {
    metric_name: String,
    value: ProtocolValue,
    labels: HashMap<String, String>,
}

impl SensorData for OwnedSensorData {
    fn metric_name(&self) -> &str {
        &self.metric_name
    }

    fn value(&self) -> ProtocolValue {
        self.value
    }

    fn labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.labels.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn from_proto<P>(proto: P) -> Result<Self>
    where
        P: ProtocolGetter + Send + Sync + 'static,
    {
        let d = proto.get_value()?;
        let mut labels = HashMap::new();
        for (k, v) in proto.get_labels() {
            labels.insert(k.to_string(), v.to_string());
        }

        Ok(Self {
            metric_name: proto.get_metric_name().to_string(),
            value: d,
            labels,
        })
    }
}

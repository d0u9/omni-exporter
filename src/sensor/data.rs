use crate::protocol::ProtocolGetter;

use super::SensorData;

#[derive(Debug)]
pub struct OwnedSensorData {
    metric_name: String,
    data: String,
}

impl SensorData for OwnedSensorData {
    fn name(&self) -> &str {
        "OwnedSensorData"
    }

    fn from_proto<P>(proto: P) -> Self
    where
        P: ProtocolGetter + Send + Sync + 'static,
    {
        Self {
            metric_name: proto.metric_name().to_string(),
            data: proto
                .value::<String>()
                .expect("Failed to get value from protocol"),
        }
    }
}

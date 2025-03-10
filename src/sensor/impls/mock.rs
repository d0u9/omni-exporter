use std::marker::PhantomData;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::exotic::sysinfo::System;
use crate::exotic::uuid;
use crate::protocol::ProtocolSetter;
use crate::protocol::plain::Plain;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use async_trait::async_trait;

const MOCK_SENSOR_NAME: &str = "MockSensor";

pub struct SensorInner {
    sys: System,
}

impl SensorInner {
    fn new() -> Self {
        Self { sys: System::new() }
    }
}

pub struct SensorImpl {
    inner: Arc<RwLock<SensorInner>>,
}

impl SensorImpl {
    pub fn new() -> Self {
        let inner = SensorInner::new();
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn get_reader<T: SensorData>(&self) -> SensorReaderImpl<T> {
        SensorReaderImpl::new()
    }
}

// T is a type that implements SensorData
pub struct SensorReaderImpl<T> {
    name: &'static str,
    id: String,
    _phantom: PhantomData<T>,
}

impl<T> SensorReaderImpl<T> {
    pub fn new() -> Self {
        Self {
            name: MOCK_SENSOR_NAME,
            id: uuid::new(),
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<T> SensorReader for SensorReaderImpl<T>
where
    T: SensorData + Send + Sync + 'static,
{
    type Data = T;

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &str {
        &self.id
    }

    async fn read(&self) -> Self::Data {
        let mut plain_proto = Plain::new();
        plain_proto.metric_name("null_metric");
        plain_proto.value("null_value");
        Self::Data::from_proto(plain_proto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::data::OwnedSensorData;

    #[tokio::test]
    async fn test_sensor_reader() {
        let sensor = SensorImpl::new();
        let reader = sensor.get_reader::<OwnedSensorData>();

        assert_eq!(reader.name(), MOCK_SENSOR_NAME);

        let data = reader.read().await;
        assert_eq!(data.name(), "OwnedSensorData");
    }
}

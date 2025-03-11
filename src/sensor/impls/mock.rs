use rand::Rng;
use std::marker::PhantomData;

use crate::error::Result;
use crate::exotic::uuid;
use crate::protocol::ProtoValue;
use crate::protocol::ProtocolSetter;
use crate::protocol::plain::Plain as PlainProto;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use async_trait::async_trait;

const MOCK_SENSOR_NAME: &str = "MockSensor";

pub struct SensorImpl {}

impl SensorImpl {
    pub fn new() -> Self {
        Self {}
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

    fn gen_mock_data(&self) -> PlainProto {
        let mut plain_proto = PlainProto::new();

        use rand::Rng;
        let mut rng = rand::rng();

        let random_value = rng.random_range(1..=10);
        plain_proto.set_metric_name(format!("mock_metric_{}", random_value).as_str());
        plain_proto.set_value(ProtoValue::U64(random_value));

        let random_value = rng.random_range(0..=3);
        for i in 1..random_value {
            plain_proto.set_labels(
                format!("key_{}", i).as_str(),
                format!("value_{}", i).as_str(),
            );
        }

        plain_proto
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

    async fn read(&self) -> Result<Vec<Self::Data>> {
        let proto = self.gen_mock_data();

        Ok(vec![Self::Data::from_proto(proto).unwrap()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::data::OwnedSensorData;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_sensor_reader() {
        let sensor = SensorImpl::new();
        let reader = sensor.get_reader::<OwnedSensorData>();

        assert_eq!(reader.name(), MOCK_SENSOR_NAME);

        let datas = reader.read().await.unwrap();
        assert!(!datas.is_empty());
        let data = &datas[0];
        assert!(data.metric_name().starts_with("mock_metric_"));

        match data.value() {
            ProtoValue::U64(v) => {
                dbg!(v);
                assert!(v > 0);
            }
            _ => panic!("Expected U64 value"),
        }
    }
}

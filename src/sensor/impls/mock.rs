use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::error::Result;
use crate::exotic::uuid;
use crate::protocol::Value as ProtoValue;
use crate::protocol::plain::Plain as Proto;
use crate::protocol::plain::PlainItem as ProtoItem;
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

    fn gen_labels() -> HashMap<String, String> {
        let mut rng = rand::rng();

        let mut labels = HashMap::new();
        let rdm = rng.random_range(2..=5);
        for i in 1..=rdm {
            labels.insert(format!("mock_key_{}", i), format!("mock_val_{}", i));
        }

        labels
    }

    fn gen_mock_data(&self) -> Proto {
        let mut rng = rand::rng();

        let mut items = Vec::<ProtoItem>::new();

        let rdm = rng.random_range(2..5);
        for i in 1..=rdm {
            items.push(ProtoItem {
                metric_name: format!("mock_metric_{}", i),
                labels: Self::gen_labels(),
                value: ProtoValue::F64(3.14),
                timestamp: None,
            });
        }

        Proto { items }
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
        Ok(proto
            .items
            .into_iter()
            .map(|item| Self::Data::from_proto(item).unwrap())
            .collect())
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

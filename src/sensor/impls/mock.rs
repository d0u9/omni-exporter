use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::error::Result;
use crate::exotic::uuid;
use crate::protocol::ItemSetter;
use crate::protocol::Value as ProtoValue;
use crate::protocol::Writer as ProtoWriter;
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
            let mut item = ProtoItem::new();
            item.set_metric_name(&format!("mock_metric_{}", i));
            item.set_labels(
                Self::gen_labels()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
            );
            item.set_value(ProtoValue::F64(std::f64::consts::PI));
            item.set_timestamp(None);
            items.push(item);
        }

        let mut proto = Proto::new();
        let _ = proto.add_items(items.into_iter());
        proto
    }
}

#[async_trait]
impl<T> SensorReader for SensorReaderImpl<T>
where
    T: SensorData + Send + Sync + 'static,
{
    type Data = T;

    fn name(&self) -> &str {
        self.name
    }

    fn id(&self) -> &str {
        &self.id
    }

    async fn read(&self) -> Result<Vec<Self::Data>> {
        let proto = self.gen_mock_data();
        Ok(proto
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

        let value: f64 = data.value().into();
        assert_eq!(value, std::f64::consts::PI);
    }
}

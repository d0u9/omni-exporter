use std::marker::PhantomData;

use crate::error::Result;
use crate::exotic::sysinfo::System;
use crate::exotic::uuid;
use crate::protocol::ProtocolSetter;
use crate::protocol::ProtocolValue;
use crate::protocol::plain::Plain;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use async_trait::async_trait;

const SENSOR_NAME: &str = "SystemSensor";

pub struct SensorImpl {
    sys: System,
}

impl SensorImpl {
    pub fn new() -> Self {
        Self { sys: System::new() }
    }

    pub fn get_reader<T: SensorData>(&self) -> SensorReaderImpl<T> {
        SensorReaderImpl::new_with_sysinfo(self.sys.clone())
    }
}

// T is a type that implements SensorData
pub struct SensorReaderImpl<T> {
    name: &'static str,
    id: String,
    sys: System,
    _phantom: PhantomData<T>,
}

impl<T> SensorReaderImpl<T> {
    pub fn new_with_sysinfo(sys: System) -> Self {
        Self {
            name: SENSOR_NAME,
            id: uuid::new(),
            sys,
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

    async fn read(&self) -> Result<Vec<Self::Data>> {
        let sys = self.sys.free_memory().await;

        let mut plain_proto = Plain::new();
        plain_proto.set_metric_name("null_metric");
        plain_proto.set_value(ProtocolValue::U64(sys));
        plain_proto.set_labels("key1", "value1");
        plain_proto.set_labels("key2", "value2");
        Ok(vec![Self::Data::from_proto(plain_proto).unwrap()])
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

        assert_eq!(reader.name(), SENSOR_NAME);

        let datas = reader.read().await.unwrap();
        assert!(!datas.is_empty());
        let data = &datas[0];
        assert_eq!(data.metric_name(), "null_metric");

        match data.value() {
            ProtocolValue::U64(v) => {
                dbg!(v);
                assert!(v > 0);
            }
            _ => panic!("Expected U64 value"),
        }

        let labels = data.labels().collect::<HashMap<_, _>>();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels.get("key1"), Some(&"value1"));
        assert_eq!(labels.get("key2"), Some(&"value2"));
    }
}


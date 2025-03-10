use std::marker::PhantomData;

use crate::exotic::sysinfo::System;
use crate::exotic::uuid;
use crate::protocol::Protocol;
use crate::protocol::plain::Plain;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;

const MOCK_SENSOR_NAME: &str = "MockSensor";

pub struct SensorImpl {
    sys: System,
}

impl SensorImpl {
    pub fn new() -> Self {
        Self { sys: System::new() }
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

    fn read(&self) -> Self::Data {
        let mut plain_proto = Plain::new();
        plain_proto.add_item("hello, from palin proto");
        Self::Data::from_proto(plain_proto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::data::OwnedSensorData;

    #[test]
    fn test_sensor_reader() {
        let sensor = SensorImpl::new();
        let reader = sensor.get_reader::<OwnedSensorData>();

        assert_eq!(reader.name(), MOCK_SENSOR_NAME);

        let data = reader.read();
        assert_eq!(data.name(), "OwnedSensorData");
    }
}

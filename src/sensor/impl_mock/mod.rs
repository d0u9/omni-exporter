use crate::protocol::Protocol;
use crate::protocol::plain::Plain;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use std::marker::PhantomData;

pub struct MockSensor {}

impl MockSensor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_reader<T: SensorData>(&self) -> MockSensorReader<T> {
        MockSensorReader::new()
    }
}

pub struct MockSensorReader<T> {
    name: String,
    id: String,
    _phantom: PhantomData<T>,
}

impl<T> MockSensorReader<T> {
    pub fn new() -> Self {
        Self {
            name: "MockSensor".to_string(),
            id: "1234567890".to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<T> SensorReader for MockSensorReader<T>
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

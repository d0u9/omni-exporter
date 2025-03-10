use std::marker::PhantomData;

use crate::protocol::Protocol;
use crate::protocol::plain::Plain;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;

pub struct SensorImpl {}

impl SensorImpl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_reader<T: SensorData>(&self) -> SensorReaderImpl<T> {
        SensorReaderImpl::new()
    }
}

pub struct SensorReaderImpl<T> {
    name: String,
    id: String,
    _phantom: PhantomData<T>,
}

impl<T> SensorReaderImpl<T> {
    pub fn new() -> Self {
        Self {
            name: "MockSensor".to_string(),
            id: "1234567890".to_string(),
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

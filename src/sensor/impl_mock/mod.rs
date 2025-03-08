use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use crate::sensor::data::OwnedSensorData;
use std::marker::PhantomData;

pub struct MockSensor {}

impl MockSensor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_reader<T: SensorData>(&self) -> MockSensorReader<T> {
        MockSensorReader::<T>::new()
    }
}

pub struct MockSensorReader<T: SensorData> {
    name: String,
    id: String,
    _phantom: PhantomData<T>,
}

impl<T: SensorData> MockSensorReader<T> {
    pub fn new() -> Self {
        Self {
            name: "MockSensor".to_string(),
            id: "1234567890".to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<T: SensorData> SensorReader for MockSensorReader<T> {
    type Data = T;

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn read(&self) -> Self::Data {
        T::from_str("1234567890")
    }
}

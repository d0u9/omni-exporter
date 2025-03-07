use crate::sensor::SensorReader;

pub struct MockSensor {}

impl MockSensor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_reader(&self) -> MockSensorReader {
        MockSensorReader::new()
    }
}

pub struct MockSensorReader {
    name: String,
    id: String,
}

impl MockSensorReader {
    pub fn new() -> Self {
        Self {
            name: "MockSensor".to_string(),
            id: "1234567890".to_string(),
        }
    }
}

impl SensorReader for MockSensorReader {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn read(&self) -> String {
        "Hello, World!".to_string()
    }
}

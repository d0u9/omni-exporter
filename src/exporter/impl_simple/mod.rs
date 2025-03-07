use std::collections::HashMap;

use super::ExporterResult;
use super::traits::Collector;
use super::traits::Exporter;
use crate::sensor::SensorReader;
use crate::types::SensorData;

// SimpleExporter schedules grabing data from sensors synchronously.
// Each time the scrape() method is called, it will read data from all sensors and return the result.
pub struct Simple {
    sensors: HashMap<String, Box<dyn SensorReader>>,
}

impl Simple {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
        }
    }

    pub async fn do_scrape(&self) -> ExporterResult<Vec<SensorData>> {
        let mut result = Vec::new();
        for sensor in self.sensors.values() {
            result.push(sensor.read());
        }
        Ok(result)
    }
}

impl Exporter for Simple {
    type Chips = Vec<SensorData>;

    async fn scrape(&self) -> ExporterResult<Self::Chips> {
        self.do_scrape().await
    }
}

impl Collector for Simple {
    fn add_sensor<T: SensorReader + Send + Sync + Sized>(&mut self, sensor: T) {
        self.sensors
            .insert(sensor.name().to_string(), Box::new(sensor));
    }
}

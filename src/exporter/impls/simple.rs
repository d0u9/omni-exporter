use std::collections::HashMap;

use super::super::ExporterResult;
use super::super::traits::Collector;
use super::super::traits::Exporter;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;

// SimpleExporter schedules grabing data from sensors synchronously.
// Each time the scrape() method is called, it will read data from all sensors and return the result.
pub struct Simple<D: SensorData> {
    sensors: HashMap<String, Box<dyn SensorReader<Data = D>>>,
}

impl<D: SensorData + 'static> Simple<D> {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
        }
    }

    pub async fn do_scrape(&self) -> ExporterResult<String> {
        let mut result = String::new();
        for sensor in self.sensors.values() {
            let data = sensor.read().await;
            dbg!(&data);
            result.push_str(data.name());
        }
        Ok(result)
    }
}

impl<D: SensorData + 'static> Exporter for Simple<D> {
    type Chips = String;

    async fn scrape(&self) -> ExporterResult<Self::Chips> {
        self.do_scrape().await
    }
}

impl<D: SensorData + 'static> Collector<D> for Simple<D> {
    fn add_sensor<T>(&mut self, sensor_reader: T)
    where
        T: SensorReader<Data = D> + Send + Sync + Sized,
    {
        self.sensors
            .insert(sensor_reader.name().to_string(), Box::new(sensor_reader));
    }
}

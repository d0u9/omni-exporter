use std::collections::HashMap;
use std::default::Default;

use crate::error::Result;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;

use super::super::Chip as ChipTrait;
use super::super::Collector;
use super::super::Exporter;
use super::super::Metric as MetricTrait;

#[derive(Debug, Default)]
pub struct Chip {
    metrics: Vec<Metric>,
}

impl ChipTrait for Chip {
    type M = Metric;

    fn metrics(&self) -> impl Iterator<Item = &Self::M> {
        self.metrics.iter()
    }
}

#[derive(Debug)]
pub struct Metric {
    name: String,
    labels: HashMap<String, String>,
    value: f64,
    timestamp: Option<u64>,
}

impl<T: SensorData> From<T> for Metric {
    fn from(data: T) -> Self {
        Metric {
            name: data.metric_name().to_owned(),
            labels: HashMap::new(),
            value: data.value().into(),
            timestamp: None,
        }
    }
}

impl MetricTrait for Metric {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.labels.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn timestamp(&self) -> Option<u64> {
        self.timestamp
    }
}

// SimpleExporter schedules grabing data from sensors synchronously.
// Each time the scrape() method is called, it will read data from all sensors and return the result.
pub struct Simple<D: SensorData> {
    sensors: HashMap<String, Box<dyn SensorReader<Data = D> + Send + Sync>>,
}

impl<D: SensorData + 'static> Default for Simple<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: SensorData + 'static> Simple<D> {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
        }
    }

    pub async fn do_scrape(&self) -> Result<Chip> {
        let mut chip: Chip = Default::default();

        for sensor in self.sensors.values() {
            let data = sensor.read().await?;

            dbg!(&data);
            for item in data {
                chip.metrics.push(Metric::from(item));
            }
        }
        Ok(chip)
    }
}

impl<D: SensorData + 'static> Exporter for Simple<D> {
    type Metrics = Chip;

    async fn scrape(&self) -> Result<Self::Metrics> {
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

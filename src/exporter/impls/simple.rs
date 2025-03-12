use crate::error::Result;
use crate::sensor::SensorData;
use crate::sensor::SensorReader;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;

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
pub struct Simple<D>
where
    D: SensorData + Debug + 'static,
{
    sensors: HashMap<String, Box<dyn SensorReader<D> + Send + Sync>>,
}

impl<D> Default for Simple<D>
where
    D: SensorData + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Simple<D>
where
    D: SensorData + Debug,
{
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
            for item in data.into_iter() {
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

impl<D> Collector<D> for Simple<D>
where
    D: SensorData + Debug + 'static,
{
    fn add_sensor<T>(&mut self, sensor_reader: T)
    where
        T: SensorReader<D> + Send + Sync + Sized + 'static,
    {
        self.sensors
            .insert(sensor_reader.name().to_string(), Box::new(sensor_reader));
    }
}

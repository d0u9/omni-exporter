use std::future::Future;

use crate::sensor::SensorReader;

use super::ExporterResult;

// Exporter charges for gathering data from sensors with is scheduler.
pub trait Exporter {
    type Chips;

    fn scrape(&self) -> impl Future<Output = ExporterResult<Self::Chips>> + Send;
}

pub trait Collector {
    fn add_sensor<T: SensorReader + Send + 'static>(&mut self, sensor: T);
}

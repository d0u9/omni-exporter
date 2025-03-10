use std::future::Future;

use crate::error::Result;

use crate::sensor::SensorData;
use crate::sensor::SensorReader;

// Exporter charges for gathering data from sensors with is scheduler.
pub trait Exporter {
    type Chips;

    fn scrape(&self) -> impl Future<Output = Result<Self::Chips>> + Send;
}

pub trait Collector<D: SensorData> {
    fn add_sensor<T: SensorReader<Data = D> + Send + 'static>(&mut self, sensor: T);
}

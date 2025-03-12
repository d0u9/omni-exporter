use std::future::Future;

use crate::error::Result;

use crate::sensor::SensorData;
use crate::sensor::SensorReader;

// Exporter charges for gathering data from sensors with is scheduler.
pub trait Exporter {
    type Metrics;

    fn scrape(&self) -> impl Future<Output = Result<Self::Metrics>> + Send;
}

pub trait Collector<D, I>
where
    D: SensorData + 'static,
    I: IntoIterator<Item = D> + 'static,
    I::IntoIter: Send + Sync,
{
    fn add_sensor<T: SensorReader<D, I> + Send + 'static>(&mut self, sensor: T);
}

pub trait Chip {
    type M: Metric;

    fn metrics(&self) -> impl Iterator<Item = &Self::M>;
}

pub trait Metric {
    fn name(&self) -> &str;

    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;

    fn value(&self) -> f64;
    fn timestamp(&self) -> Option<u64>;
}

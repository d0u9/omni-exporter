use crate::metric::Metric;
use crate::sensor::{SensorReader, SystemReader};

pub enum Sensor {
    System(SystemReader),
    External(Box<dyn SensorReader<Metrics = Vec<Metric>>>),
}

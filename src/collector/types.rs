use crate::sensor::{SensorReader, SystemReader};

pub enum Sensor {
    System(SystemReader),
    External(Box<dyn SensorReader>),
}

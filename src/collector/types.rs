use crate::metric::Metrics;
use crate::sensor::{SensorReader, SysctlReader, SystemReader};

pub enum Sensor {
    System(SystemReader),
    Sysctl(SysctlReader),
    External(Box<dyn SensorReader<Metrics = Metrics>>),
}

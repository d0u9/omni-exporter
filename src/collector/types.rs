use crate::metric::Metrics;
use crate::sensor::{CoreReader, SensorReader, SysctlReader};

pub enum Sensor {
    Core(CoreReader),
    Sysctl(SysctlReader),
    External(Box<dyn SensorReader<Metrics = Metrics>>),
}

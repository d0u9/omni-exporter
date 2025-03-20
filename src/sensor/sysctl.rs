use async_trait::async_trait;

use crate::collector::Sensor;
use crate::error::Result;
use crate::metric::MetricValue;
use crate::metric::{Metric, Metrics};
use crate::sensor::SensorReader;

pub struct SysctlReader;

impl SysctlReader {
    pub fn new() -> Self {
        Self
    }

    pub fn new_sensor() -> Sensor {
        Sensor::Sysctl(Self::new())
    }
}

impl Default for SysctlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SensorReader for SysctlReader {
    type Metrics = Metrics;

    async fn read(&self) -> Result<Self::Metrics> {
        Ok(Metrics::from_vec(vec![
            Metric::new("sysctl_a", MetricValue::U64(1)),
            Metric::new("sysctl_b", MetricValue::U64(2)),
            Metric::new("sysctl_c", MetricValue::U64(3)),
        ]))
    }
}

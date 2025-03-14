use crate::error::Result;
use crate::metric::MetricValue;
use crate::metric::{Metric, Metrics};
use crate::sensor::SensorReader;
use async_trait::async_trait;

pub struct SysctlReader;

impl SysctlReader {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SensorReader for SysctlReader {
    type Metrics = Metrics;

    async fn read(&self) -> Result<Self::Metrics> {
        Ok(Metrics::from_vec(vec![
            Metric::new(MetricValue::U64(1)),
            Metric::new(MetricValue::U64(2)),
            Metric::new(MetricValue::U64(3)),
        ]))
    }
}

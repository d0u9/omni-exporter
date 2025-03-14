use async_trait::async_trait;

use crate::error::Result;
use crate::types::{Metric, MetricLabels};

#[async_trait]
pub trait InternalSensorReader {
    type Labels: MetricLabels;
    type Metrics: IntoIterator<Item = Metric<Self::Labels>>;

    async fn update(&self) -> Result<Self::Metrics>;
}

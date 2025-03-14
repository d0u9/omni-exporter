use async_trait::async_trait;

use crate::error::Result;
use crate::metric;

#[async_trait]
pub trait SensorReader {
    type Metrics: IntoIterator<Item = metric::Metric>;

    async fn read(&self) -> Result<Self::Metrics>;
}

use async_trait::async_trait;

use crate::error::Result;
use crate::metric::Metric;

#[async_trait]
pub trait SensorReader {
    type Metrics: IntoIterator<Item = Metric>;

    async fn read(&self) -> Result<Self::Metrics>;
}

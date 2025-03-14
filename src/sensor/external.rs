use async_trait::async_trait;

use crate::error::Result;
use crate::types::Metric;

pub type Labels = Vec<(String, String)>;

#[async_trait]
pub trait SensorReader {
    async fn update(&self) -> Result<Vec<Metric<Labels>>>;
}

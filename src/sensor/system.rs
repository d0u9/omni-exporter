use async_trait::async_trait;
use std::collections::HashMap;

use crate::error::Result;
use crate::fetcher::Meminfo;
use crate::sensor::InternalSensorReader;
use crate::types::Metric;

pub struct SystemReader {
    meminfo: Meminfo,
}

impl SystemReader {
    pub fn new() -> Self {
        SystemReader {
            meminfo: Meminfo::new(),
        }
    }
}

type Labels = HashMap<String, String>;
type Metrics = Vec<Metric<Labels>>;

impl SystemReader {
    async fn update_meminfo(&self, metrics: &mut Metrics) -> Result<()> {
        metrics.extend::<Vec<_>>(self.meminfo.get_meminfo().await?.into());
        Ok(())
    }
}

#[async_trait]
impl InternalSensorReader for SystemReader {
    type Labels = HashMap<String, String>;
    type Metrics = Vec<Metric<Self::Labels>>;

    async fn update(&self) -> Result<Self::Metrics> {
        let mut metrics = Self::Metrics::new();

        self.update_meminfo(&mut metrics).await?;

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_sensor() {
        let s = SystemReader::new();
        let metrics = s.update().await.unwrap();
        println!("{:?}", metrics);
    }
}

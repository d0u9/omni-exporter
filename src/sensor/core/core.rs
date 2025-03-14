use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::fetcher::Meminfo;
use crate::metric::{MetricFamily, Metrics};
use crate::sensor::SensorReader;
use async_trait::async_trait;

use super::meminfo;

enum FamilyNames {
    Memory,
}

impl FamilyNames {
    const MEMORY: &str = "memory";

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn to_str(&self) -> &'static str {
        match self {
            FamilyNames::Memory => Self::MEMORY,
        }
    }
}

pub struct CoreReader {
    meminfo: Meminfo,
    metric_families: HashMap<String, &'static MetricFamily>,
}

impl CoreReader {
    pub fn new() -> Self {
        CoreReader {
            meminfo: Meminfo::new(),
            metric_families: HashMap::from([(FamilyNames::Memory.to_string(), meminfo::family())]),
        }
    }

    async fn meminfo(&self) -> Result<Metrics> {
        let famliy = self
            .metric_families
            .get(FamilyNames::Memory.to_str())
            .ok_or(Error::CustomError(
                "memory metric family not found".to_string(),
            ))?;
        let meminfo = self.meminfo.get_meminfo().await?;
        let mut metrics = Metrics::from(meminfo);
        metrics
            .as_mut()
            .iter_mut()
            .for_each(|metric| famliy.tag_metric(metric));
        Ok(metrics)
    }

    async fn read_all(&self) -> Result<Metrics> {
        let mut metrics = Vec::new();
        metrics.extend(self.meminfo().await?);
        Ok(Metrics::from_vec(metrics))
    }
}

#[async_trait]
impl SensorReader for CoreReader {
    type Metrics = Metrics;

    async fn read(&self) -> Result<Self::Metrics> {
        self.read_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_sensor() {
        let s = CoreReader::new();
        let metrics = s.read().await.unwrap();
        println!("{:?}", metrics);
    }
}

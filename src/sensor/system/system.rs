use std::collections::{HashMap, HashSet};

use crate::error::{Error, Result};
use crate::fetcher::Meminfo;
use crate::metric::{Metric, MetricFamily, MetricType};
use crate::sensor::SensorReader;
use async_trait::async_trait;

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

enum MemoryLabelKeys {
    Label1,
    Label2,
}

impl MemoryLabelKeys {
    const LABEL1: &str = "label1";
    const LABEL2: &str = "label2";

    fn to_str(&self) -> &'static str {
        match self {
            MemoryLabelKeys::Label1 => Self::LABEL1,
            MemoryLabelKeys::Label2 => Self::LABEL2,
        }
    }

    fn list_all() -> &'static [&'static str; 2] {
        &[Self::LABEL1, Self::LABEL2]
    }
}

pub struct SystemReader {
    meminfo: Meminfo,
    metric_families: HashMap<String, MetricFamily>,
}

impl SystemReader {
    pub fn new() -> Self {
        SystemReader {
            meminfo: Meminfo::new(),
            metric_families: HashMap::from([(
                FamilyNames::Memory.to_string(),
                MetricFamily::new(
                    FamilyNames::Memory.to_string(),
                    "Memory information".to_string(),
                    MetricType::Gauge,
                    HashSet::from_iter(MemoryLabelKeys::list_all().iter().map(|s| s.to_string())),
                ),
            )]),
        }
    }

    async fn meminfo(&self) -> Result<Vec<Metric>> {
        let famliy = self
            .metric_families
            .get(FamilyNames::Memory.to_str())
            .ok_or(Error::CustomError(
                "memory metric family not found".to_string(),
            ))?;
        let meminfo = self.meminfo.get_meminfo().await?;
        let mut metrics: Vec<Metric> = meminfo.into();
        metrics
            .iter_mut()
            .for_each(|metric| famliy.tag_metric(metric));
        Ok(metrics)
    }

    async fn read_all(&self) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();
        metrics.extend(self.meminfo().await?);
        Ok(metrics)
    }
}

#[async_trait]
impl SensorReader for SystemReader {
    type Metrics = Vec<Metric>;

    async fn read(&self) -> Result<Self::Metrics> {
        self.read_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_sensor() {
        let s = SystemReader::new();
        let metrics = s.read().await.unwrap();
        println!("{:?}", metrics);
    }
}

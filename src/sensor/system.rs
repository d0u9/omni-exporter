use async_trait::async_trait;
use std::collections::HashMap;

use crate::error::Result;
use crate::fetcher::Meminfo;
use crate::sensor::traits::SensorReader;
use crate::types::{Metric, MetricLabels};

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

#[async_trait]
impl SensorReader for SystemReader {
    type Labels = HashMap<String, String>;
    type Metrics = Vec<Metric<Self::Labels>>;

    async fn update(&self) -> Result<Self::Metrics> {
        let mut metrics = Self::Metrics::new();

        if let Ok(meminfo_metrics) = self.meminfo.get_meminfo().await {
            metrics.extend::<Vec<_>>(meminfo_metrics.into());
        }

        Ok(metrics)
    }
}

impl MetricLabels for HashMap<String, String> {
    fn labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.iter().map(|(k, v)| (k.as_str(), v.as_str()))
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

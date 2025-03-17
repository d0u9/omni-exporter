use crate::error::Result;

use super::super::FetcherMetric;
use super::super::FetcherMetricName;

pub type CpuMetric = FetcherMetric<CpuMetricNames>;

pub struct CPUInner {}

impl CPUInner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn update(&self) -> Result<Vec<CpuMetric>> {
        let metrics = vec![];
        Ok(metrics)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CpuMetricNames {
    CpuTotal,
}

impl FetcherMetricName for CpuMetricNames {
    fn to_str(&self) -> &'static str {
        match self {
            CpuMetricNames::CpuTotal => "cpu_total",
        }
    }
}

impl AsRef<str> for CpuMetricNames {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl From<CpuMetricNames> for &'static str {
    fn from(name: CpuMetricNames) -> Self {
        name.to_str()
    }
}

#[tokio::test]
async fn test_get_cpu_usage() {
    let cpu = CPUInner::new();
    let metrics = cpu.update().await.unwrap();
    println!("{:?}", metrics);
}

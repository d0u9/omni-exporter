use crate::error::Result;
use std::borrow::Cow;

use super::super::FetcherMetric;
use super::super::FetcherMetricName;
use crate::metric::Label;
use nosafe::ffi;

pub type CpuMetric = FetcherMetric<CpuMetricNames>;

pub struct CPUInner {}

impl CPUInner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn update(&self) -> Result<Vec<CpuMetric>> {
        let cpus = ffi::macos::cpu::get_cpu()?;
        let metrics = cpus
            .into_iter()
            .enumerate()
            .flat_map(|(i, cpu)| {
                vec![
                    FetcherMetric::new_with_labels(
                        CpuMetricNames::User,
                        cpu.user,
                        vec![
                            Label::new("cpu", i.to_string()),
                            Label::new("mode", CpuMetricNames::User.as_str().to_string()),
                        ],
                    ),
                    FetcherMetric::new_with_labels(
                        CpuMetricNames::System,
                        cpu.system,
                        vec![
                            Label::new("cpu", i.to_string()),
                            Label::new("mode", CpuMetricNames::System.as_str().to_string()),
                        ],
                    ),
                    FetcherMetric::new_with_labels(
                        CpuMetricNames::Nice,
                        cpu.nice,
                        vec![
                            Label::new("cpu", i.to_string()),
                            Label::new("mode", CpuMetricNames::Nice.as_str().to_string()),
                        ],
                    ),
                    FetcherMetric::new_with_labels(
                        CpuMetricNames::Idle,
                        cpu.idle,
                        vec![
                            Label::new("cpu", i.to_string()),
                            Label::new("mode", CpuMetricNames::Idle.as_str().to_string()),
                        ],
                    ),
                ]
            })
            .collect();
        Ok(metrics)
    }
}

impl Default for CPUInner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CpuMetricNames {
    User,
    System,
    Nice,
    Idle,
}

impl CpuMetricNames {
    pub fn as_str(&self) -> &'static str {
        match self {
            CpuMetricNames::User => "user",
            CpuMetricNames::System => "system",
            CpuMetricNames::Nice => "nice",
            CpuMetricNames::Idle => "idle",
        }
    }
}

impl FetcherMetricName for CpuMetricNames {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl From<CpuMetricNames> for Cow<'static, str> {
    fn from(name: CpuMetricNames) -> Self {
        Cow::Borrowed(name.as_str())
    }
}

#[tokio::test]
async fn test_get_cpu_usage() {
    let cpu = CPUInner::new();
    let metrics = cpu.update().await.unwrap();
    println!("{:?}", metrics);
}

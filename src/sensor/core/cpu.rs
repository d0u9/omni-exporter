use std::sync::OnceLock;

use crate::error::Result;
use crate::fetcher::cpu::CPU as Fetcher;
use crate::fetcher::cpu::CpuMetric;
use crate::metric::{Metric, MetricFamily, MetricType, Metrics, Timestamp};

pub mod consts {
    pub const FAMILY_NAMESPACE: &str = "node";
}

mod cpu_family {
    use super::*;

    pub const FAMILY_NAME: &str = "seconds_total";
    pub const FAMILY_HELP: &str = "Seconds the CPUs spent in each mode.";

    #[allow(dead_code)]
    pub enum LabelKeys {
        Cpu,
        Mode,
    }

    impl AsRef<str> for LabelKeys {
        fn as_ref(&self) -> &str {
            self.to_str()
        }
    }

    impl LabelKeys {
        const CPU: &str = "cpu";
        const MODE: &str = "mode";

        pub fn to_str(&self) -> &'static str {
            match self {
                LabelKeys::Cpu => Self::CPU,
                LabelKeys::Mode => Self::MODE,
            }
        }

        pub fn list_all() -> &'static [&'static str] {
            &[Self::CPU, Self::MODE]
        }
    }

    pub fn family() -> &'static MetricFamily {
        static METRIC_FAMILY: OnceLock<MetricFamily> = OnceLock::new();
        METRIC_FAMILY.get_or_init(|| {
            MetricFamily::new_with_namespace(
                consts::FAMILY_NAMESPACE,
                FAMILY_NAME,
                FAMILY_HELP,
                MetricType::Counter,
                LabelKeys::list_all(),
            )
        })
    }
}

mod online_family {
    use super::*;

    pub const FAMILY_NAME: &str = "online";
    pub const FAMILY_HELP: &str = "CPUs that are online and being scheduled.";

    #[allow(dead_code)]
    pub enum LabelKeys {
        Cpu,
    }

    impl AsRef<str> for LabelKeys {
        fn as_ref(&self) -> &str {
            self.to_str()
        }
    }

    impl LabelKeys {
        const CPU: &str = "cpu";
        const MODE: &str = "mode";

        pub fn to_str(&self) -> &'static str {
            match self {
                LabelKeys::Cpu => Self::CPU,
            }
        }

        pub fn list_all() -> &'static [&'static str] {
            &[Self::CPU]
        }
    }

    pub fn family() -> &'static MetricFamily {
        static METRIC_FAMILY: OnceLock<MetricFamily> = OnceLock::new();
        METRIC_FAMILY.get_or_init(|| {
            MetricFamily::new_with_namespace(
                consts::FAMILY_NAMESPACE,
                FAMILY_NAME,
                FAMILY_HELP,
                MetricType::Gauge,
                LabelKeys::list_all(),
            )
        })
    }
}

impl From<Vec<CpuMetric>> for Metrics {
    fn from(cpu: Vec<CpuMetric>) -> Self {
        Metrics::from_vec(
            cpu.into_iter()
                .map(|m| Metric::new_with_labels(m.name, m.value, m.labels.unwrap_or_default()))
                .collect(),
        )
    }
}

pub struct Cpu;

impl Cpu {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn update(&self) -> Result<Metrics> {
        let fetcher = Self::fetcher()?;
        let mut metrics: Metrics = fetcher.update().await?.into();
        metrics.iter_mut().for_each(|m| {
            cpu_family::family().tag_metric(m);
            if m.timestamp == Timestamp::None {
                m.timestamp = Timestamp::now();
            }
        });
        Ok(metrics)
    }

    fn fetcher() -> Result<&'static Fetcher> {
        static METRIC_FAMILY: OnceLock<Fetcher> = OnceLock::new();
        let fetcher = METRIC_FAMILY.get_or_init(Fetcher::new);
        Ok(fetcher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meminfo() {
        let cpu = Cpu::new();
        let metrics = cpu.update().await.unwrap();
        println!("{:?}", metrics);
    }
}

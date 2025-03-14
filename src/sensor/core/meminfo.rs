use std::sync::OnceLock;
use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::metric::{Metric, MetricFamily, MetricType, Metrics};

pub fn family() -> &'static MetricFamily {
    static METRIC_FAMILY: OnceLock<MetricFamily> = OnceLock::new();
    METRIC_FAMILY.get_or_init(|| {
        MetricFamily::new(
            "meminfo",
            "Memory information",
            MetricType::Gauge,
            LabelKeys::list_all(),
        )
    })
}

#[allow(dead_code)]
pub enum MetricNames {
    MemFreeBytes,
}

#[allow(dead_code)]
pub enum LabelKeys {
    Label1,
    Label2,
}

impl LabelKeys {
    const LABEL1: &str = "label1";
    const LABEL2: &str = "label2";

    pub fn to_str(&self) -> &'static str {
        match self {
            LabelKeys::Label1 => Self::LABEL1,
            LabelKeys::Label2 => Self::LABEL2,
        }
    }

    pub fn list_all() -> &'static [&'static str; 2] {
        &[Self::LABEL1, Self::LABEL2]
    }
}

impl MetricNames {
    const MEM_FREE_BYTES: &str = "mem_free_bytes";

    pub fn to_str(&self) -> &'static str {
        match self {
            MetricNames::MemFreeBytes => Self::MEM_FREE_BYTES,
        }
    }
}

impl From<MeminfoMetrics> for Metrics {
    fn from(meminfo: MeminfoMetrics) -> Self {
        Metrics::from_vec(vec![Metric::new(
            MetricNames::MemFreeBytes.to_str(),
            meminfo.mem_free_bytes.into(),
        )])
    }
}

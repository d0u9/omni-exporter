use crate::error::Result;
use crate::exotic::system as syslib;
use crate::metric::MetricValue;

use super::Metric;

pub struct MeminfoInner {
    inner: syslib::System,
}

impl MeminfoInner {
    pub fn new() -> Self {
        let sys = syslib::System::new();
        Self::new_with_system(sys)
    }

    pub fn new_with_system(sys: syslib::System) -> Self {
        Self { inner: sys }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();

        let m = self.inner.free_memory().await;
        metrics.push(Metric {
            name: MetricNames::FreeBytes,
            value: MetricValue::U64(m),
        });

        Ok(metrics)
    }
}

#[derive(Debug, PartialEq)]
pub enum MetricNames {
    ActiveBytes,
    CompressedBytes,
    InactiveBytes,
    WiredBytes,
    FreeBytes,
    SwappedInBytesTotal,
    SwappedOutBytesTotal,
    InternalBytes,
    PurgeableBytes,
    TotalBytes,
    SwapUsedBytes,
    SwapTotalBytes,
}

impl MetricNames {
    pub fn to_str(&self) -> &'static str {
        match self {
            MetricNames::FreeBytes => "free_bytes",
            MetricNames::ActiveBytes => "active_bytes",
            MetricNames::CompressedBytes => "compressed_bytes",
            MetricNames::InactiveBytes => "inactive_bytes",
            MetricNames::WiredBytes => "wired_bytes",
            MetricNames::SwappedInBytesTotal => "swapped_in_bytes_total",
            MetricNames::SwappedOutBytesTotal => "swapped_out_bytes_total",
            MetricNames::InternalBytes => "internal_bytes",
            MetricNames::PurgeableBytes => "purgeable_bytes",
            MetricNames::TotalBytes => "total_bytes",
            MetricNames::SwapUsedBytes => "swap_used_bytes",
            MetricNames::SwapTotalBytes => "swap_total_bytes",
        }
    }
}

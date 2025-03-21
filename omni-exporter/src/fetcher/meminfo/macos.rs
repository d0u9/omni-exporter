use std::borrow::Cow;

use crate::error::Result;
use crate::ffi;

use super::super::FetcherMetric;
use super::super::FetcherMetricName;

pub type MeminfoMetric = FetcherMetric<MeminfoMetricNames>;

pub struct MeminfoInner;

impl MeminfoInner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_meminfo(&self) -> Result<Vec<MeminfoMetric>> {
        let meminfo = ffi::macos::meminfo::get_meminfo()?;

        let metrics = vec![
            MeminfoMetric::new(MeminfoMetricNames::FreeBytes, meminfo.free_bytes),
            MeminfoMetric::new(MeminfoMetricNames::ActiveBytes, meminfo.active_bytes),
            MeminfoMetric::new(
                MeminfoMetricNames::CompressedBytes,
                meminfo.compressed_bytes,
            ),
            MeminfoMetric::new(MeminfoMetricNames::InactiveBytes, meminfo.inactive_bytes),
            MeminfoMetric::new(MeminfoMetricNames::WiredBytes, meminfo.wired_bytes),
            MeminfoMetric::new(
                MeminfoMetricNames::SwappedInBytesTotal,
                meminfo.swapped_in_bytes_total,
            ),
            MeminfoMetric::new(
                MeminfoMetricNames::SwappedOutBytesTotal,
                meminfo.swapped_out_bytes_total,
            ),
            MeminfoMetric::new(MeminfoMetricNames::InternalBytes, meminfo.internal_bytes),
            MeminfoMetric::new(MeminfoMetricNames::PurgeableBytes, meminfo.purgeable_bytes),
            MeminfoMetric::new(MeminfoMetricNames::TotalBytes, meminfo.total_bytes),
            MeminfoMetric::new(MeminfoMetricNames::SwapUsedBytes, meminfo.swap_used_bytes),
            MeminfoMetric::new(MeminfoMetricNames::SwapTotalBytes, meminfo.swap_total_bytes),
        ];

        Ok(metrics)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MeminfoMetricNames {
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

impl MeminfoMetricNames {
    pub fn as_str(&self) -> &'static str {
        match self {
            MeminfoMetricNames::FreeBytes => "free_bytes",
            MeminfoMetricNames::ActiveBytes => "active_bytes",
            MeminfoMetricNames::CompressedBytes => "compressed_bytes",
            MeminfoMetricNames::InactiveBytes => "inactive_bytes",
            MeminfoMetricNames::WiredBytes => "wired_bytes",
            MeminfoMetricNames::SwappedInBytesTotal => "swapped_in_bytes_total",
            MeminfoMetricNames::SwappedOutBytesTotal => "swapped_out_bytes_total",
            MeminfoMetricNames::InternalBytes => "internal_bytes",
            MeminfoMetricNames::PurgeableBytes => "purgeable_bytes",
            MeminfoMetricNames::TotalBytes => "total_bytes",
            MeminfoMetricNames::SwapUsedBytes => "swap_used_bytes",
            MeminfoMetricNames::SwapTotalBytes => "swap_total_bytes",
        }
    }
}

impl FetcherMetricName for MeminfoMetricNames {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl From<MeminfoMetricNames> for Cow<'static, str> {
    fn from(name: MeminfoMetricNames) -> Self {
        Cow::Borrowed(name.as_str())
    }
}

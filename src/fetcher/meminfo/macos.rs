use crate::error::Result;
use crate::metric::MetricValue;

use crate::ffi;

use super::Metric;

pub struct MeminfoInner;

impl MeminfoInner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_meminfo(&self) -> Result<Vec<Metric>> {
        let meminfo = ffi::macos::meminfo::get_meminfo()?;

        let metrics = vec![
            Metric {
                name: MetricNames::FreeBytes,
                value: MetricValue::U64(meminfo.free_bytes),
            },
            Metric {
                name: MetricNames::ActiveBytes,
                value: MetricValue::U64(meminfo.active_bytes),
            },
            Metric {
                name: MetricNames::CompressedBytes,
                value: MetricValue::U64(meminfo.compressed_bytes),
            },
            Metric {
                name: MetricNames::InactiveBytes,
                value: MetricValue::U64(meminfo.inactive_bytes),
            },
            Metric {
                name: MetricNames::WiredBytes,
                value: MetricValue::U64(meminfo.wired_bytes),
            },
            Metric {
                name: MetricNames::SwappedInBytesTotal,
                value: MetricValue::U64(meminfo.swapped_in_bytes_total),
            },
            Metric {
                name: MetricNames::SwappedOutBytesTotal,
                value: MetricValue::U64(meminfo.swapped_out_bytes_total),
            },
            Metric {
                name: MetricNames::InternalBytes,
                value: MetricValue::U64(meminfo.internal_bytes),
            },
            Metric {
                name: MetricNames::PurgeableBytes,
                value: MetricValue::U64(meminfo.purgeable_bytes),
            },
            Metric {
                name: MetricNames::TotalBytes,
                value: MetricValue::U64(meminfo.total_bytes),
            },
            Metric {
                name: MetricNames::SwapUsedBytes,
                value: MetricValue::U64(meminfo.swap_used_bytes),
            },
            Metric {
                name: MetricNames::SwapTotalBytes,
                value: MetricValue::U64(meminfo.swap_total_bytes),
            },
        ];

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
